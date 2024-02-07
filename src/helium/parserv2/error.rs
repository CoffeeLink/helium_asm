use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Default, Debug)]
struct ParserError(String);

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Parse Err: {}", self.0)
    }
}

impl Error for ParserError {
    fn description(&self) -> &str {
        &self.0
    }
}