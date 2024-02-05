
#[derive(Debug)]
pub enum Error {
    UnexpectedToken(String),
    UnexpectedEOF,
    ParseError(String),
    MismatchedTypes(String),
    UnknownIdentifier(String),
    UnknownDirective(String),
    SystemError(String)
}