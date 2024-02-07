use std::error::Error;
use std::fmt::{Display, Formatter};
use crate::helium::tokens::TokenKind;

#[derive(Debug)]
pub enum  ParserError {
    UnexpectedEOF,
    MismatchedTypes{
        expected : TokenKind,
        got : TokenKind
    },
    ConstantCollision {
        file : String,
        name : String
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ParserError::UnexpectedEOF => {"Unexpected EOF".into()}
            ParserError::MismatchedTypes { expected, got } => {
                format!("Mismatched Types. expected: {}, got: {}", expected, got)
            }
            ParserError::ConstantCollision { file, name } => {
                format!("Constant collision: {}:{}", file, name)
            }
        };
        write!(f, "{}", str)
    }
}