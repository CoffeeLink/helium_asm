// main errors

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic {0}")]
    Generic(String), // this is for debug and pre 1.0v dev stuff.

    #[error("{0}")]
    LexicalError(String), // For errors while lexing

    #[error(transparent)]
    IO(#[from] std::io::Error),

}