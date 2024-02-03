
#[derive(Debug)]
pub enum Error {
    UnexpectedToken,
    ParseError(String)
}