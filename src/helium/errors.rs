
#[derive(Debug)]
pub enum Error {
    UnexpectedToken(String),
    ParseError(String),
    MismatchedTypes(String),
    UnknownIdentifier(String),
}