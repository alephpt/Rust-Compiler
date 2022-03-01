
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

macro_rules! ingest {
    ($self:ident, $($inner:tt),*) => {
        if let Some(c) = $self.chars.peek() {
            if ingest!(impl c, $($inner),*) {
                let temp = *c;
                $self.consume_space();
                Some(temp)
            } else {
                None
            }
        } else {
            None
        }
    };

    (impl , ) => (false);
    (impl $c:ident, $item:tt) => (*$c == $item);
    (impl $c:ident, $item:tt, $($rest:tt), +) => (ingest!(impl $c, $item) || ingest!(impl $c, $($rest),+));
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

    fn map_base_to_num(n: &NumericBase) -> u32 {
        match n {
            NumericBase::Binary => 2,
            NumericBase::Octal => 8,
            NumericBase::Decimal => 10,
            NumericBase::Hexadecimal => 16,
            NumericBase::Base64 => 64,
            _ => panic!("Invalid Base To Number Mapping")
        }
    }

    fn map_num2base(n: &u32) -> NumericBase {
         match n {
            2 => NumericBase::Binary,
            8 => NumericBase::Octal,
            10 => NumericBase::Decimal,
            16 => NumericBase::Hexadecimal,
            64 => NumericBase::Base64,
            _ => panic!("Invalid Number to Base Mapping")
        }   
    }

    fn map_num_to_base(n: &str) -> Result<NumericBase, LexerError> {
        match n {
            "2" => Ok(NumericBase::Binary),
            "8" => Ok(NumericBase::Octal),
            "10" => Ok(NumericBase::Decimal),
            "16" => Ok(NumericBase::Hexadecimal),
            "64" => Ok(NumericBase::Base64),
            _ => Err(LexerError::InvalidNumericBase { base: n.to_string() })
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

    fn digest_digit(&mut self, exp_radix: u32, fractional: &mut bool, b64: bool, empty: bool) -> Result<String, LexerError> {
        let mut raw = String::new();
        loop{
            match self.chars.peek(){
                None => {
                    break if empty || raw.len() > 0 {
                        Ok(raw)
                    } else {
                        Err(LexerError::NumericLiteralCollapse{ 
                            received: TokenType::EOF,
                            expected: Numeric {
                                raw: "<int>".to_string(),
                                base: NumericBase::Any,
                                kind: NumericKind::Any,
                            }
                        })
                    }
                },
                Some(c) if c.is_whitespace() || (*c == 'b' || *c == 'B' || *c == 'e' || *c == 'E' || *c == '.') && !*fractional => break Ok(raw),
                Some(c) if (c.is_digit(exp_radix) || b64) || !c.is_whitespace() => { 
                    raw.push(*c); 
                    self.consume_space(); 
                },
                Some(c) => {
                    break Err(LexerError::UnknownNumericLiteral{ raw, received: *c })
                },
            }
        }
    }

       /*
        * 29 => simple number, int, digits
        * .1 => floating point, starting with .
        * 1.1 => dot at some point
        *  1e+2192 => exponential
        */

    fn parse_numbers(&mut self, start: char) -> Result<TokenType, LexerError> {
        let mut seen_b64 = false;
        let mut seen_dec = false;
        let mut base_radix = 10;
        let mut raw = start.to_string();
        let mut kind = NumericKind::Whole;
        let mut base = NumericBase::Decimal;

        // parse whole numbers and fractions
        if start == '.'{
            seen_dec = true;
            raw += &self.digest_digit(base_radix, &mut seen_dec, seen_b64, false)?;
            kind = NumericKind::Fractional;
        } else if start.is_digit(base_radix){
            raw += &self.digest_digit(base_radix, &mut seen_dec, seen_b64, true)?;
            // println!("Passing Base");
            if let Some(c) = ingest!(self, '.') {
                raw.push(c);
                raw += &self.digest_digit(base_radix, &mut seen_dec, seen_b64, false)?;
                seen_dec = true;
                kind = NumericKind::Fractional;
            }

        // parse Exponentials
        if let Some(c) = ingest!(self, 'e', 'E') {
            kind = NumericKind::Exponential;
            base = NumericBase::Decimal;
            raw.push(c);
            
            if let Some(c) = ingest!(self, '+', '-') {
                raw.push(c);
            }

            raw += &self.digest_digit(base_radix, &mut seen_dec, seen_b64, false)?;
        }
        
        // parse base components
        if let Some(_c) = ingest!(self, 'b', 'B') {  // explicit base declaration
            let raw_base = raw.clone();
            raw.clear();

            base = Lexer::<'a>::map_num_to_base(&raw_base)?;

            if base == NumericBase::Base64 { 
                seen_b64 = true; 
                base_radix = 10; }
            else {       
                base_radix = Lexer::<'a>::map_base_to_num(&base); 
            }

            raw += &self.digest_digit(base_radix, &mut seen_dec, seen_b64, false)?;
        }
        


        } else {
            println!("Compiler Bug .. Not sure how we had a number that isn't a number?!");
            return Err(LexerError::InvalidNumericLiteral {
                base,
                raw,
                received: start.to_string(),
            });
        }
        Ok(TokenType::Numeric{ raw, base, kind })
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
