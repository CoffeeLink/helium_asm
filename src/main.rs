#![allow(unused)] // Before v1.0

use std::fs::read_to_string;
use crate::lexer::lexer::Lexer;
use crate::lexer::token::Token;
use crate::prelude::*;

mod error;
mod prelude;
mod utils;
mod lexer;

fn main() -> Result<()> {
    let contents = read_to_string("test.hsm").unwrap();

    let mut lexer = Lexer::new("test.hsm", &contents);

    let (tokens, errors) = lexer.tokenize();
    if !errors.is_empty() {
        println!("Errors present!");
        println!("{:?}", errors);
        println!()
    }

    println!("Tokens: ");

    for token in tokens{
        println!("{:?}", token);
    }
    Ok(())
}
