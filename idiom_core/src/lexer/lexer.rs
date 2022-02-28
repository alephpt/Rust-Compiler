use crate::lexer::*;

pub struct Lexer<'a> {
    // Readable positions
    pub cur_line: usize,
    pub cur_col: usize,

    // Raw Index position
    pub codepoint_offset: usize,

    chars: std::iter::Peekable<std::str::Chars<'a>>,
    parameter_state: std::collections::HashMap<char, ParameterDepthType>,
}


impl<'a> Lexer<'a> {
    pub fn new(chars: &'a str) -> Lexer<'a> {
        Lexer {
            cur_col: 1,
            cur_line: 1,
            codepoint_offset: 0,

            chars: chars.chars().peekable(),
            parameter_state: std::collections::HashMap::new()
        }
    }

    fn map_delimiters(c: &char) -> char {
        match c {
            '|' => '_', // Open Function
            '_' => '|', // Close Function
            ',' => '-', // Open Parameters
            '-' => ',', // Close Paremeters
            '{' => '}', // Open Object
            '}' => '{', // Close Object
            '<' => '>', // Open Vector
            '>' => '<', // Close Vector
            '[' => ']', // Open Array
            ']' => '[', // Close Array
            '(' => ')', // Open Join
            ')' => '(', // Close Join

            _ => panic!("How are you going to use Unmapped Delimiters that don't doesn't Map existing?")
        }
    }

    fn open_delimiters(&mut self, c: &char) -> ParameterDepthType {
        if let Some(v) = self.parameter_state.get_mut(&c) {
            *v += 1;
            *v - 1
        } else {
            self.parameter_state.insert(*c, 1);
            0
        }
    }

    fn close_delimiters(&mut self, c: &char) -> Result<ParameterDepthType, LexerError> {
        if let Some(v) = self.parameter_state.get_mut(&Lexer::map_delimiters(&c)) {
            if *v >= 1 {
                *v -= 1;
                Ok(*v)
            } else {
                Err(LexerError::MisMatchedDelimiters{symbol: *c, requires: Lexer::map_delimiters(&c)})
            }
        } else {
            Err(LexerError::MisMatchedDelimiters{symbol: *c, requires: Lexer::map_delimiters(&c)})
            
        }
    }

    fn consume_digit(&mut self, num: &String, exp_radix: u32) -> Result<char, LexerError> {
        match self.chars.next(){
            None => {
                Err(LexerError::NumericLiteralCollapse{ received: num.to_string() })
            },
            Some(c) if !c.is_digit(exp_radix) => {
                Err(LexerError::NumericLiteralCollapse{ received: num.to_string() })
            },
            Some(c) => Ok(c)
        }
    }

       /*
        * 29 => simple number, int, digits
        * .1 => floating point, starting with .
        * 1.1 => dot at some point
        *  1e+2192 => exponential
        */

    fn parse_numbers(&mut self, start: char) -> Result<TokenType, LexerError> {
       let mut seen_dot = false;
       let mut seen_exp = false;
       let mut seen_base = false;
       let mut seen_hex = false;
       let mut seen_bin = false;
       let mut seen_oct = false;
       let mut seen_b64 = false;
       let mut base_radix=10;
       let mut num = start.to_string();

       if start == '.'{
           num.push(self.consume_digit(&num, base_radix)?);
           seen_dot = true;
       }

       loop {
           match self.chars.peek() {
               Some(c) if *c == '.' && !seen_dot && !seen_exp && !seen_base => {
                  num.push(*c);
                  self.consume_space();
                  seen_dot = true;
               }, 
               Some(c) if *c == 'e' || *c == 'E' && !seen_base => {
                  num.push(*c);
                  self.consume_space();
                  seen_exp=true;

                  match self.chars.peek() {
                      Some(c) if *c == '+' || *c == '-' => {
                        num.push(*c);
                        self.consume_space();
                      },
                      _ => {}
                  }

                  num.push(self.consume_digit(&num, base_radix)?);
               },
               Some(c) if *c == 'b' && !seen_exp && !seen_dot && !seen_base => {  // explicit base declaration
                  let base = num.clone();
                  num.clear();
                  self.consume_space();

                  if base == "2" {
                      seen_base = true;
                      seen_bin = true;
                      base_radix = 2;
                  } else
                  if base == "8" {
                      seen_base = true;
                      seen_oct = true;
                      base_radix = 8;
                  } else
                  if base == "16" {
                      seen_hex = true;
                      seen_base = true;
                      base_radix = 16;
                  } else 
                  if base == "64" {
                      seen_base = true;
                      seen_b64 = true;
                  } else {
                      break Err(LexerError::InvalidBaseNumeric{ basereceived: base })
                  }
               },
               Some(c) if !c.is_digit(base_radix) && !c.is_whitespace() && !seen_b64 => {
                  num.push(*c);
                  if seen_bin {
                    break Err(LexerError::InvalidBinaryValue { bin: num });
                  } else
                  if seen_oct {
                    break Err(LexerError::InvalidOctValue { oct: num });
                  } else 
                  if seen_hex {
                    break Err(LexerError::InvalidHexValue { hex: num });
                  } else {
                    break Err(LexerError::UnknownNumericLiteral { unknown: num });
                  }
               },
               Some(c) if c.is_digit(base_radix) || seen_b64 && !c.is_whitespace() => {
                   num.push(*c);
                   self.consume_space();
               },
               Some(c) if c.is_ascii_alphabetic() && !seen_base && !c.is_digit(base_radix) => {  
                  num.push(*c);
                  break Err(LexerError::NumericLiteralCollapse{ received: num });
               }
                _ => {  //exit condition
                  if seen_base {
                    if seen_bin {
                        break Ok(TokenType::Numeric{ raw: num, base: NumericBase::Binary, kind: NumericKind::Whole });
                    }

                    if seen_hex {
                        break Ok(TokenType::Numeric{ raw: num, base: NumericBase::Hexidecimal, kind: NumericKind::Whole });
                    }

                    if seen_oct {
                        break Ok(TokenType::Numeric{ raw: num, base: NumericBase::Octal, kind: NumericKind::Whole });
                    }

                    if seen_b64 {
                        break Ok(TokenType::Numeric{ raw: num, base: NumericBase::Base64, kind: NumericKind::Whole });
                    }
                    
                    if seen_exp {
                        break Ok(TokenType::Numeric{ raw: num, base: NumericBase::Denary, kind: NumericKind::Exponential})
                    }
                  } else {
                      break Ok(TokenType::Numeric{ raw: num, base: NumericBase::Denary, kind: if seen_dot { NumericKind::Fractional } else { NumericKind::Whole }  });
                  }
               }
           }
       }

     //  Ok()
    }
    

    pub fn transform_content(&mut self, c: char) -> Result<TokenType, LexerError> {
        match c {
            // Delimiters
            ',' => Ok(TokenType::Delimiters{ raw: c, kind: DelimitersKind::Opening(self.open_delimiters(&c)) }),
            '?' => Ok(TokenType::Delimiters{ raw: c, kind: DelimitersKind::Closing(self.close_delimiters(&c)?) }),
            '|' => Ok(TokenType::Delimiters{ raw: c, kind: DelimitersKind::Opening(self.open_delimiters(&c)) }),
            '_' => Ok(TokenType::Delimiters{ raw: c, kind: DelimitersKind::Closing(self.close_delimiters(&c)?) }),
            '{' | '[' | '<' | '(' => Ok(TokenType::Delimiters{ raw: c, kind: DelimitersKind::Opening(self.open_delimiters(&c)) }),
            '}' | ']' | '>' | ')' => Ok(TokenType::Delimiters{ raw: c, kind: DelimitersKind::Closing(self.close_delimiters(&c)?) }),

            // Numbers
            '0' ..= '9' | '.' => self.parse_numbers(c),
            
            // Operators
            

            // Indentifiers
            

            _ => Err(LexerError::UnknownPokemon{ unknowns: c.to_string() })
        }
    }
    
    fn consume_space(&mut self) -> Option<char> {
        match self.chars.next() {
            Some(c) => {
                self.cur_col += 1;

                if c == '\n' {
                    self.cur_line += 1;
                    self.cur_col = 1;
                }

                self.codepoint_offset += 1;

                Some(c)
            }
            None => None
        }
    }

    fn skip_spaces(&mut self) {
        while let Some(c) = self.chars.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.consume_space();
        }
    }

    pub fn next_token(&mut self) -> Result<TokenType, LexerError> {
        self.skip_spaces();

        if let Some(c) = self.consume_space(){
            self.transform_content(c)
        } else {
            Ok(TokenType::EOF)
        }
    }
}
