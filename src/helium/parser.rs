use std::collections::HashMap;
use crate::helium::errors::Error;
use crate::helium::tokens::Token;

pub enum ConstantType {
    Label(String),
    Value(u16)
}
pub struct ProgramTree {
    constants : HashMap<String, Option<ConstantType>>,
    segments : Vec<ProgramSegment>
}

pub struct ProgramSegment {
    name : String,
    origin : u32,
    elements : ProgramElement
}

pub enum ProgramElement {
    Instruction(),
    Identifier(String),
    Immediate(u16)
}

pub struct Parser {
    tokens : Vec<Token>
}

impl Parser {
    pub fn new(tokens : Vec<Token>) -> Self {
        Self {
            tokens
        }
    }

    pub fn parse(mut self) -> Result<ProgramTree, Vec<Error>> {
        todo!()
    }
    fn parse_instruction(&mut self) -> Result<(), Error> {
        todo!()
    }
}