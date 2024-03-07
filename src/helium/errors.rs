use std::fmt::Debug;
use crate::helium::position::Position;


#[derive(Debug)]
pub enum Error {
    UnexpectedToken(String, u32),
    UnexpectedEOF(u32),
    ParseError(String, u32),
    MismatchedTypes(String, u32),
    UnknownIdentifier(String, u32),
    UnknownDirective(String, u32),
    SystemError(String, u32),
    IncludeError(String, u32),
    ConstantCollision(String, u32),
}
impl Error {
    pub fn get_line(&self) -> u32 {
        match self {
            Error::UnexpectedToken(_, l) => *l,
            Error::UnexpectedEOF(l) => *l,
            Error::ParseError(_, l) => *l,
            Error::MismatchedTypes(_, l) => *l,
            Error::UnknownIdentifier(_, l) => *l,
            Error::UnknownDirective(_, l) => *l,
            Error::SystemError(_, l) => *l,
            Error::IncludeError(_, l) => *l,
            Error::ConstantCollision(_, l) => *l
        }
    }
    pub fn get_err(&self) -> &str {
        match self {
            Error::UnexpectedToken(a, _) => a,
            Error::UnexpectedEOF(_) => "Eof",
            Error::ParseError(s, _) => s,
            Error::MismatchedTypes(s, _) => s,
            Error::UnknownIdentifier(s, _) => s,
            Error::UnknownDirective(s, _) => s,
            Error::SystemError(s, _) => s,
            Error::IncludeError(s, _) => s,
            Error::ConstantCollision(s, _) => s
        }
    }
}

// new error system.
#[derive(Clone, Debug)]
pub struct HeliumError { // Todo: refactor name to 'Error' when ready for refactoring.
    pub pos: Position,
    pub message : String // this will do for now.
}

impl HeliumError {
    pub fn new(message: String, pos: Position) -> Self {
        Self {
            message,
            pos,
        }
    }
}