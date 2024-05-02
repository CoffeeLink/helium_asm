

pub use crate::error::Error;
pub type Result<T> = core::result::Result<T, Error>; // A basic Result definition that uses the crate error by default.
