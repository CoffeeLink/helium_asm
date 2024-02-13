use std::fmt::{Debug};

#[derive(Debug)]
pub enum Error {
    UnexpectedToken(String),
    UnexpectedEOF,
    ParseError(String),
    MismatchedTypes(String),
    UnknownIdentifier(String),
    UnknownDirective(String),
    SystemError(String),
    IncludeError(String),
    ConstantCollision(String),
}

/*
New error system.

 */
