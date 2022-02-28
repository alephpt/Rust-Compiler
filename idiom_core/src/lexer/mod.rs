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

    #[error("Invalid Numeric Character. {received:?} Literal Fails")]
    NumericLiteralCollapse { received: String },

    #[error("Invalid Base Component. Base {basereceived:?} is Not a Valid Type.")]
    InvalidBaseNumeric { basereceived: String },

    #[error("Invalid Binary Value: {bin:?}")]
    InvalidBinaryValue { bin: String },

    #[error("Invalid Hexadecimal Value: {hex:?}")]
    InvalidHexValue { hex: String },

    #[error("Invalid Octal Value: {oct:?}")]
    InvalidOctValue { oct: String },

    #[error("Unknown Numerical Literal: {unknown:?}")]
    UnknownNumericLiteral{ unknown: String },

    #[error("Unidentified Token - {unknowns:?}")]
    UnknownPokemon { unknowns: String }
}

pub type Token = TokenType;

pub struct Delimiters {
    pub raw: char,
    pub kind: DelimitersKind,
}

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
    Whole,
    Fractional,
    Exponential,
    Bool,
}

#[derive(Debug, PartialEq)]
pub enum NumericBase {
    Binary,
    Octal,
    Denary,
    Hexadecimal,
    Base64,
}
