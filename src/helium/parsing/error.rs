use crate::helium::errors::{HeliumError};
use crate::helium::tokens::TokenKind;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParserError {
    UnexpectedEOF,
    MismatchedTypes { expected: TokenKind, got: TokenKind },
    ConstantCollision { file: String, name: String },
    UnexpectedToken { kind: TokenKind },
    Named { error: String },
    UnknownDirective { name: String },
    UnknownIdentifier { name: String },
    FileNotFound(String),
    FileError(String),
    IncludeLexError(HeliumError),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ParserError::UnexpectedEOF => "Unexpected EOF".into(),
            ParserError::MismatchedTypes { expected, got } => {
                format!("Mismatched Types. expected: {}, got: {}", expected, got)
            }
            ParserError::ConstantCollision { file, name } => {
                format!("Constant collision: {}:{}", file, name)
            }
            ParserError::UnexpectedToken { kind } => {
                format!("Unexpected Token: {}", kind)
            }
            ParserError::Named { error } => error.to_string(),
            ParserError::UnknownDirective { name } => {
                format!("Unknown Directive: {}", name)
            }
            ParserError::UnknownIdentifier { name } => {
                format!("Unknown Identifier: '{}'", name)
            }
            ParserError::FileNotFound(name) => {
                format!("File Not Found: {}", name)
            }
            ParserError::FileError(name) => {
                format!("Could Not Open File: {}", name)
            }
            ParserError::IncludeLexError(err) => {
                format!("{:?}", err)
            }
        };
        write!(f, "{}", str)
    }
}
