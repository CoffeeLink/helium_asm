use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
use crate::helium::errors::Error;
use crate::helium::errors::Error::{MismatchedTypes, UnknownIdentifier};
use crate::helium::instructions::Instruction;
use crate::helium::parser::ConstantType::{Label, Unknown, Value};
use crate::helium::tokens::{Token, TokenKind};
use crate::helium::tokens::TokenKind::{ConstantDeclaration, Identifier, Integer};

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub enum ConstantType {
    Label,
    Value(u16),
    Unknown
}
pub struct ProgramTree {
    pub constants : HashMap<String, ConstantType>,
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

                ConstantDeclaration => {
                    // the one after needs to be a word
                    let next = self.tokens.next();
                    if next.is_some() && &next.unwrap().kind != &Identifier {
                        errors.push(MismatchedTypes(
                            format!("Identifier expected, found: {:?}", next.unwrap().kind)
                        ));
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
                        errors.push(MismatchedTypes(
                            format!("Integer expected, found: {:?}", next.unwrap().kind)
                        ));
                        continue;
                    }
                    let value = next.unwrap().
                        value.clone().
                        unwrap()
                        .get_int_value()
                        .unwrap();

                    tree.constants.insert(name, Value(value));
                }
                TokenKind::Label => {
                    let name = token.clone()
                        .value.unwrap()
                        .get_word_value().unwrap();

                    tree.constants.insert(name.clone(), Label);
                    // replace segment.
                    tree.segments.push(current_segment);
                    current_segment = ProgramSegment::new(&name);
                }
                TokenKind::Identifier => {
                    // put into consts as notfound and also put into curr seg
                    let key = token.clone()
                        .value.unwrap()
                        .get_word_value().unwrap();
                    if !tree.constants.contains_key(&key) {
                        tree.constants.insert(key.clone(), Unknown);
                    }
                    current_segment.elements.push(
                        ProgramElement::Identifier(key)
                    )
                }
                TokenKind::Integer => {
                    // also raw data
                    current_segment.elements.push(
                        ProgramElement::Immediate(token.clone()
                            .value.unwrap()
                            .get_int_value().unwrap()
                        )
                    )
                }

                _ => {}
            }
        }
        // add final segment
        tree.segments.push(current_segment);

        // check for any missing constant definitions.
        for (key, value) in tree.constants.clone() {
            if value == Unknown {
                errors.push(UnknownIdentifier(
                    format!("Unknown Identifier: {}", key)
                ));
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