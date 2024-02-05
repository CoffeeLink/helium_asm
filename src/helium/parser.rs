use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
use crate::helium::errors::Error;
use crate::helium::errors::Error::MismatchedTypes;
use crate::helium::instructions::Instruction;
use crate::helium::parser::ConstantType::Value;
use crate::helium::tokens::{Token, TokenKind};
use crate::helium::tokens::TokenKind::{Identifier, Integer};

pub enum ConstantType {
    Label(String),
    Value(u16)
}
pub struct ProgramTree {
    pub constants : HashMap<String, Option<ConstantType>>,
    pub segments : Vec<ProgramSegment>
}
impl ProgramTree {
    pub fn new() -> Self {
        Self { constants: Default::default(), segments: vec![] }
    }
}
pub struct ProgramSegment {
    name : String,
    origin : Option<u32>,
    elements : Vec<ProgramElement>
}

impl ProgramSegment {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            origin: None,
            elements: vec![],
        }
    }
    pub fn with_origin(name : &str, origin : u32) -> Self {
        Self {
            name: name.to_string(),
            origin: Some(origin),
            elements: vec![],
        }
    }
}
pub enum ProgramElement {
    Instruction(Instruction),
    Identifier(String),
    Immediate(u16)
}
pub struct Parser<'a> {
    tokens : Peekable<Iter<'a, Token>>
}
impl <'a> Parser<'a> {
    pub fn new(tokens : &'a Vec<Token>) -> Self {
        Self { tokens: tokens.iter().peekable() }
    }
    pub fn parse(mut self) -> Result<ProgramTree, Vec<Error>> {
        let mut tree = ProgramTree{ constants: Default::default(), segments: vec![] };
        let mut errors: Vec<Error> = vec![];
        // create root segment
        let mut current_segment = ProgramSegment::new("entry");

        while let Some(token) = self.tokens.next() {
            match token.kind {
                TokenKind::Newline |
                TokenKind::SemiColon |
                TokenKind::Comma => {/* Do nothing just consume */}

                TokenKind::ConstantDeclaration => {
                    // the one after needs to be a word
                    let next = self.tokens.next();
                    if next.is_some() && &next.unwrap().kind != &Identifier {
                        errors.push(MismatchedTypes(format!("Identifier expected, found: {:?}", next.unwrap().kind)));
                        continue;
                    }
                    // Correct type.
                    let name = next
                        .unwrap()
                        .value.clone()
                        .unwrap()
                        .get_word_value()
                        .unwrap();

                    // check for immediate.
                    let next = self.tokens.next();
                    if next.is_some() && &next.unwrap().kind != &Integer {
                        errors.push(MismatchedTypes(format!("Integer expected, found: {:?}", next.unwrap().kind)));
                        continue;
                    }
                    let value = next.unwrap().
                        value.clone().
                        unwrap()
                        .get_int_value()
                        .unwrap();

                    tree.constants.insert(name, Some(Value(value)));
                }

                _ => {}
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(tree)
    }
    fn parse_instruction(&mut self) -> Result<(), Error> {
        todo!()
    }
}