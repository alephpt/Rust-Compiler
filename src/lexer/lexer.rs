extern crate thiserror;

use std::io;
use core::fmt::Debug;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("File IO Error")]
    FileIO(#[from] io::Error),

    #[error("Expected Symbol {expected:?} Missing!\nFound {found:?}")]
    MissingChildren {
        expected: TokenType,
        found: Token,
    },

    #[error("Improper Parameterization with {symbol:?}")]
    ImbalancedImmuneSystem {
        symbol: char,
        requires: char
    },

    #[error("Unidentified Token - {unknowns:?}")]
    UnknownPokemon {
        unknowns: String,
    }
}

pub type Token = TokenType;

pub struct Delimitters {
    pub raw: char,
    pub kind: DelimittersKind,
}

#[derive(Debug)]
pub enum TokenType {
    /* End of Token Stream */
    EOF,

    /* Indicators like ',', '(', '[', etc .. */
    Delimitters{raw: char, kind: DelimittersKind},

    /* Operators like '*', '<-', etc.. */
    Operators(String),

    /* Sequence of Characters */
    Identifiers(String),

    /* A single Character */
    Character(char),

    Numeric{raw: String /*, base: NumericBaseKind, postfix: NumberPostfixKind, form: NumericForm*/},

 //   Magic{raw: String, kind: MagicKind, form: MagicForm, component: MagicComponent} 
}

type ParameterDepthType = i32;

#[derive(Debug)]
pub enum DelimittersKind {
    Open(ParameterDepthType),
    Close(ParameterDepthType),
    Seperator,
}


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

    fn map_delimitters(c: &char) -> char {
        match c {
            '|' => '_',
            '_' => '|',
            ',' => '?',
            '?' => ',',

            _ => panic!("How are you going to Map Delimitters that don't doesn't Map?")
        }
    }

    fn open_parameters(&mut self, c: &char) -> ParameterDepthType {
        if let Some(v) = self.parameter_state.get_mut(&c) {
            *v += 1;
            *v - 1
        } else {
            self.parameter_state.insert(*c, 1);
            0
        }
    }

    fn close_parameters(&mut self, c: &char) -> Result<ParameterDepthType, LexerError> {
        if let Some(v) = self.parameter_state.get_mut(&Lexer::map_delimitters(&c)) {
            if *v >= 1 {
                *v -= 1;
                Ok(*v)
            } else {
                Err(LexerError::ImbalancedImmuneSystem{symbol: *c, requires: Lexer::map_delimitters(&c)})
            }
        } else {
            Err(LexerError::ImbalancedImmuneSystem{symbol: *c, requires: Lexer::map_delimitters(&c)})
            
        }
    }

    pub fn parse_parameters(&mut self, c: char) -> Result<TokenType, LexerError> {
        match c {
            ',' => Ok(TokenType::Delimitters{ raw: c, kind: DelimittersKind::Open(self.open_parameters(&c)) }),
            '?' => Ok(TokenType::Delimitters{ raw: c, kind: DelimittersKind::Close(self.close_parameters(&c)?) }),
            '|' => Ok(TokenType::Delimitters{ raw: c, kind: DelimittersKind::Open(self.open_parameters(&c)) }),
            '_' => Ok(TokenType::Delimitters{ raw: c, kind: DelimittersKind::Close(self.close_parameters(&c)?) }),
//            '.' => Ok(TokenType::Delimitters{ raw: c, kind: DelimittersKind::Close(self.close_parameters(&c)?) }),
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
            self.parse_parameters(c)
        } else {
            Ok(TokenType::EOF)
        }
    }
}
