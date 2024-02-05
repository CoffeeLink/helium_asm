use std::collections::HashMap;
use std::iter::Peekable;
use std::slice::Iter;
use crate::helium::errors::Error;
use crate::helium::errors::Error::{MismatchedTypes, UnexpectedEOF, UnexpectedToken, UnknownDirective, UnknownIdentifier};
use crate::helium::instructions::{Argument, AsmInstruction, Instruction};
use crate::helium::parser::ConstantType::{Label, Unknown, Value};
use crate::helium::tokens::{Token, TokenKind, ValueKind};
use crate::helium::tokens::TokenKind::{Comma, ConstantDeclaration, Identifier, Integer, Newline, Register, SemiColon};

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub enum ConstantType {
    Label,
    Value(u16),
    Unknown
}

#[derive(Debug)]
pub struct ProgramTree {
    pub constants : HashMap<String, ConstantType>,
    pub segments : Vec<ProgramSegment>
}
impl ProgramTree {
    pub fn new() -> Self {
        Self { constants: Default::default(), segments: vec![] }
    }
}
#[derive(Debug)]
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
#[derive(Debug)]
pub enum ProgramElement {
    Instruction(Instruction),
    Identifier(String),
    Immediate(u16)
}
#[derive(Debug)]
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
                TokenKind::Register => {
                    // this isn't allowed so raise error.
                    let reg_key = token.clone().value.unwrap();
                    errors.push(match reg_key {
                        ValueKind::Instruction(i) => {UnexpectedToken(format!("Unexpected Token: Register({:?})", i))}
                        ValueKind::Integer(i) => {UnexpectedToken(format!("Unexpected Token: Register({})", i))}
                        ValueKind::Word(w) => {UnexpectedToken(format!("Unexpected Token: Register({})", w))}
                    })
                }
                TokenKind::Directive => {
                    // check directive name and if its ths skipto diretive check if there is a label following
                    // if so, create that label with an origin of that skipto-s addr
                    // if not, create a new label with skipto_<skipto_id>_addr and set org
                    let directive_name = token.clone().value.unwrap().get_word_value().unwrap();
                    match directive_name.as_str() {
                        "entry" => {}
                        "no_predec" => {}
                        "skipto" => {
                            // get the addr(can only be Imm Int) than check if there is a label after
                            // if there is a label, create the new segment and set its origin to the addr.
                            // if there is no label than create a new segment with the origin.

                            /*
                            let addr_token = match self.tokens.next() {
                                Some(a) => a,
                                None => {
                                    errors.push(UnexpectedEOF)
                                }
                            };
                            */
                        }
                        "pre_load" => {}
                        "include" => {}

                        _ => { // unknown directive
                            errors.push(UnknownDirective(
                                format!("Unknown Directive: {}", directive_name)
                            ))
                        }
                    }
                },
                TokenKind::Instruction => {
                    let instruction_code = token.clone()
                        .value.unwrap()
                        .get_instruction_code().unwrap();

                    match self.parse_instruction(instruction_code) {
                        Ok(i) => current_segment.elements.push(ProgramElement::Instruction(i)),
                        Err(e) => {
                            errors.push(e);
                        }
                    }
                }
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
    fn parse_instruction(&mut self, instruction: AsmInstruction) -> Result<Instruction, Error> {
        let mut ins = Instruction::new(instruction);
        // consume arguments until a newline or a semicolon.
        // error on invalid token types like instruction, directive etc.

        while let Some(next) = self.tokens.peek() {
            if next.kind == Newline || next.kind == SemiColon { break } // instruction parsed.
            let token = self.tokens.next().unwrap().clone();

            if token.kind == Comma { continue }

            if token.kind != Identifier && token.kind != Integer && token.kind != Register {
                return Err(UnexpectedToken(
                    format!("Cannot pass: {:?} into {:?} as an argument", token.kind, instruction)
                ))
            }

            match token.kind {
                Integer => {
                    let value = token.value.unwrap().get_int_value().unwrap();
                    ins.args.push(
                        Argument::Immediate(value)
                    )
                }
                Identifier => {
                    let ident = token.value.unwrap().get_word_value().unwrap();
                    ins.args.push(
                        Argument::ImmediateIdentifier(ident)
                    )
                }
                Register => {
                    match token.value.unwrap() {
                        ValueKind::Instruction(_) => {}
                        ValueKind::Integer(val) => {
                            ins.args.push(
                                Argument::Register(val)
                            )
                        }
                        ValueKind::Word(val) => {
                            ins.args.push(
                                Argument::RegisterIdentifier(val)
                            )
                        }
                    }
                }
                _ => {}
            }

        }

        Ok(ins)
    }
}