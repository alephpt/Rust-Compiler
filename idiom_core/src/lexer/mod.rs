extern crate thiserror;

pub mod lexer;
pub use lexer::*;

pub mod macros;
pub use macros::*;

use std::io;
use core::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("File IO Error")]
    FileIO(#[from] io::Error),

    #[error("Expected Symbol {expected:?} Missing!\nFound {found:?}")]
    ImproperUsage { expected: TokenType, found: Token },

    #[error("Improper Parameterization with {symbol:?}")]
    MisMatchedDelimiters { symbol: char, requires: char },

    #[error("Invalid Numeric Character for {base:?} Number: {raw:?} Fails. {received:?} is invalid.")]
    InvalidNumericLiteral { base: NumericBase, raw: String, received: String }, // can we add expected behaviour?

    #[error("Invalid Fraction: {raw:?}.")]
    InvalidFraction { raw: String },
    
    #[error("Invalid Base Number. {base:?} is Not a Valid Base implimented in Idiom_Core.")]
    InvalidNumericBase { base: String },

    #[error("Invalid Binary Value: {bin:?}")]
    InvalidBinaryValue { bin: String },

    #[error("Invalid Hexadecimal Value: {hex:?}")]
    InvalidHexValue { hex: String },

    #[error("Invalid Octal Value: {oct:?}")]
    InvalidOctValue { oct: String },

    #[error("Numerical Literal Collapsed. Found: {received:?}, Expected: {expected:?}")]
    NumericLiteralCollapse{ received: TokenType, expected: Numeric },

    #[error("Unexpected Numeric Digest: {raw:?}, Received: {received:?}")]
    UnknownNumericLiteral{ raw: String, received: char },

    #[error("Unidentified Token - {unknowns:?}")]
    UnknownPokemon { unknowns: String }
}

pub type Token = TokenType;

pub struct Delimiters {
    pub raw: char,
    pub kind: DelimitersKind,
}

#[derive(Debug)]
pub struct Numeric {
    pub raw: String,
    pub base: NumericBase,
    pub kind: NumericKind,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    /* End of Token Stream */
    EOF,

    /* Indicators like ',', '(', '[', etc .. */
    Delimiters{raw: char, kind: DelimitersKind},

    /* Operators like '*', '<-', etc.. */
    Operators(String),

    /* Sequence of Characters */
    Identifiers(String),

    /* A single Character */
    Character(char),

    Numeric{raw: String, base: NumericBase, kind: NumericKind},

 //   Magic{raw: String, kind: MagicKind, form: MagicForm, component: MagicComponent}
}

type ParameterDepthType = i32;

#[derive(Debug, PartialEq)]
pub enum DelimitersKind {
    Opening(ParameterDepthType),
    Closing(ParameterDepthType),
    Seperator,
}

#[derive(Debug, PartialEq)]
pub enum NumericKind {
    Any,
    Whole,
    Fractional,
    Exponential,
    Bool,
}

#[derive(Debug, PartialEq)]
pub enum NumericBase {
    Any,
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
    Base64,
}
