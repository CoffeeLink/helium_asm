mod parser;

use std::collections::{BTreeSet, HashMap};
use std::fs::read_to_string;
use std::iter::Peekable;
use std::path::Path;
use std::slice::Iter;
use crate::helium::defaults::DEFAULT_CONSTS;
use crate::helium::errors::Error;
use crate::helium::errors::Error::{ConstantCollision, IncludeError, MismatchedTypes, UnexpectedEOF, UnexpectedToken, UnknownDirective, UnknownIdentifier};
use crate::helium::instructions::{Argument, AsmInstruction, Instruction};
use crate::helium::lexer::Lexer;
use crate::helium::parser::ConstantType::{Label, Unknown, Value};
use crate::helium::tokens::{Token, TokenKind, ValueKind};
use crate::helium::tokens::TokenKind::{Comma, ConstantDeclaration, Identifier, Integer, Newline, Register, SemiColon};

//Note: this file is actual spaghetti that you can eat, use the tabs as sauce.

#[derive(Copy, Clone, PartialOrd, PartialEq, Debug)]
pub enum ConstantType {
    Label,
    Value(u16),
    Unknown
}

#[derive(Debug, Default, Clone)]
pub struct ProgramTree {
    pub file_name : String,
    pub constants : HashMap<String, ConstantType>,
    pub segments : Vec<ProgramSegment>,

    pub imports : BTreeSet<String>,
    // It seems like the better choice than HashSet
    // because i dont think there will be more than ~50 (this seems like a way to high number)
    // HashSet has a big array that usually never gets fully utilized.
}
impl ProgramTree {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn include(&mut self, mut other : Self, file_name : &str) -> Result<(), Vec<Error>> {
        if self.imports.contains(&other.file_name) {
            return Ok(());
        }

        let mut errors : Vec<Error> = vec![];

        // check for constant collisions
        let mut const_collision_found = false;

        let _ = &other.constants.keys().map(|k| { // Iter through all
            if self.constants.contains_key(k) { return }
            const_collision_found = true;
            errors.push(ConstantCollision(
                format!("constant collision found in file: {}: {}", file_name,k)
            ))
        });

        if const_collision_found { return Err(errors); }

        // renames the entry point of the other. should never crash cuz the first segment is always there.
        let name = other.segments[0].name.clone();
        other.segments.get_mut(0).unwrap().name =
            format!("__{}:{}__auto_renamed", file_name, name);
        
        // take segments, constants and imports
        self.constants.extend(other.constants);
        self.segments.extend(other.segments);
        self.imports.extend(other.imports);

        // add as include.
        self.imports.insert(other.file_name);

        Ok(())
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub enum ProgramElement {
    Instruction(Instruction),
    Identifier(String),
    Immediate(u16)
}
#[derive(Debug)]
pub struct Parser<'a> {
    tokens : Peekable<Iter<'a, Token>>,
    file_name : String
}
impl <'a> Parser<'a> {
    pub fn new(tokens : &'a [Token], file_name : String) -> Self {
        Self { tokens: tokens.iter().peekable(), file_name }
    }
    pub fn parse(mut self, import_tree : Option<BTreeSet<String>>) -> Result<ProgramTree, Vec<Error>> {
        let mut tree = ProgramTree::new();
        let mut errors: Vec<Error> = vec![];
        let mut imports : BTreeSet<String> = Default::default();

        tree.file_name = self.file_name.clone();
        tree.imports.insert(self.file_name.clone());

        if let Some(t) = import_tree {
            tree.imports.extend(t);
        }

        // create root segment
        let mut current_segment = ProgramSegment::new("entry");

        //config and other stuff
        let mut last_auto_id: u32 = 0;
        let mut pre_dec_allowed = true;

        while let Some(token) = self.tokens.next() {
            match token.kind {
                Newline |
                SemiColon |
                Comma => {/* Do nothing just consume */}

                ConstantDeclaration => {
                    // the one after needs to be a word
                    let next = self.tokens.next();
                    if next.is_some() && next.unwrap().kind != Identifier {
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
                    if next.is_some() && next.unwrap().kind != Integer {
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
                Identifier => {
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
                Integer => {
                    // also raw data
                    current_segment.elements.push(
                        ProgramElement::Immediate(token.clone()
                            .value.unwrap()
                            .get_int_value().unwrap()
                        )
                    )
                }
                Register => {
                    // this isn't allowed so raise error.
                    let reg_key = token.clone().value.unwrap();
                    errors.push(match reg_key {
                        ValueKind::Instruction(i) => {UnexpectedToken(format!("Unexpected Token: Register({:?})", i))}
                        ValueKind::Integer(i) => {UnexpectedToken(format!("Unexpected Token: Register({})", i))}
                        ValueKind::Word(w) => {UnexpectedToken(format!("Unexpected Token: Register({})", w))}
                    })
                }
                TokenKind::Directive => {
                    // check directive name and if its ths skipto directive check if there is a label following
                    // if so, create that label with an origin of that skipto-s addr
                    // if not, create a new label with skipto_<skipto_id>_addr and set org
                    let directive_name = token.clone().value.unwrap().get_word_value().unwrap();
                    match directive_name.as_str() {
                        "no_predec" => {pre_dec_allowed = false;}
                        "skipto" => {
                            let addr_token = match self.tokens.next() {
                                Some(a) => a,
                                None => {
                                    errors.push(UnexpectedEOF);
                                    continue;
                                }
                            };
                            if addr_token.kind != Integer {
                                errors.push(MismatchedTypes(
                                    format!("Integer needed, but found: {:?}", addr_token.kind)
                                ));
                                continue;
                            }
                            let addr = addr_token.clone()
                                .value.unwrap().get_int_value().unwrap();

                            if self.tokens.peek().is_none() {
                                errors.push(UnexpectedEOF);
                                continue
                            }
                            // before looking for next token.
                            // eat all "whitespace tokens" before looking for the label.
                            while let Some(token) = self.tokens.peek() {
                                if token.kind != SemiColon &&
                                    token.kind != Comma &&
                                    token.kind != Newline {
                                    break;
                                }
                                self.tokens.next();
                            }

                            let next_token = self.tokens.peek().unwrap();

                            if next_token.kind == TokenKind::Label {
                                // create new segment with the labels name
                                let name = next_token.value
                                    .clone().unwrap()
                                    .get_word_value().unwrap();

                                tree.constants.insert(name.clone(), Label);

                                // replace segment.
                                tree.segments.push(current_segment);
                                current_segment = ProgramSegment::new(&name);
                                current_segment.origin = Some(u32::from(addr));

                            } else {
                                let name = format!("__auto_label_unnamed_segment_with_origin_{}:{}__", addr, last_auto_id);
                                last_auto_id += 1;
                                tree.constants.insert(name.clone(), Label);

                                tree.segments.push(current_segment);
                                current_segment = ProgramSegment::new(&name);
                                current_segment.origin = Some(u32::from(addr));
                            }
                        }

                        "include" => {
                            let file_name = self.tokens.next_if(|token|{
                                token.kind == Identifier
                            });
                            let name = match file_name {
                                None => {
                                    errors.push(MismatchedTypes("Include needs Identifier but found something else".to_string()));
                                    continue;
                                }
                                Some(n) => {n.clone().value.unwrap().get_word_value().unwrap()}
                            };
                            imports.insert(name);
                        }

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

                    match self.parse_instruction(instruction_code, &mut tree) {
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

        if pre_dec_allowed {
            tree.constants.extend(DEFAULT_CONSTS.clone());
        }

        // import everything
        for import in imports {
            self.include(&mut tree, import).unwrap_or_else(|e|{
                errors.push(e);
            });
        }

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
    fn parse_instruction(&mut self, instruction: AsmInstruction, tree : &mut ProgramTree) -> Result<Instruction, Error> {
        let mut ins = Instruction::new(instruction);
        // consume arguments until a newline or a semicolon.
        // error on invalid token types like instruction, directive etc.

        while let Some(next) = self.tokens.peek() {
            if next.kind == Newline || next.kind == SemiColon { break } // instruction parsed.
            let token = self.tokens.next().unwrap().clone();

            if token.kind == Comma { continue }

            if token.kind != Identifier && token.kind != Integer && token.kind != Register {
                return Err(UnexpectedToken(format!("Cannot pass: {:?} into {:?} as an argument", token.kind, instruction)))
            }

            match token.kind {
                Integer => {
                    let value = token.value.unwrap().get_int_value().unwrap();
                    ins.args.push(Argument::Immediate(value))
                }
                Identifier => {
                    let ident = token.value.unwrap().get_word_value().unwrap();

                    if !tree.constants.contains_key(&ident) {
                        tree.constants.insert(ident.clone(), Unknown);
                    }

                    ins.args.push(
                        Argument::ImmediateIdentifier(ident)
                    )
                }
                Register => {
                    match token.value.unwrap() {
                        ValueKind::Instruction(_) => {}
                        ValueKind::Integer(val) => {
                            ins.args.push(Argument::Register(val))
                        }
                        ValueKind::Word(val) => {
                            if !tree.constants.contains_key(&val) {
                                tree.constants.insert(val.clone(), Unknown);
                            }
                            ins.args.push(Argument::RegisterIdentifier(val))
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(ins)
    }
    fn include(&mut self, tree : &mut ProgramTree, name : String) -> Result<(), Error> {
        if tree.imports.contains(&name) {
            return Ok(());
        }

        // check if exists
        let path = Path::new(&name);
        if !path.is_file() {
            return Err(IncludeError(
                format!("Could not find file {}", &name)
            ))
        }

        // open file
        let contents = read_to_string(path);
        if contents.is_err() { return Err(IncludeError(
            format!("Unable to read file: {}", &name)
        )) }
        let contents = contents.unwrap();

        // lex file
        let tokens = Lexer::new(&contents).lex();
        if tokens.is_err() {
            return Err(IncludeError(format!("Failed to include {}, errors found.", &name)))
        }
        let tokens = tokens.unwrap();

        // Parse tokens
        let parsed = Parser::new(&tokens, name.clone()).parse(Some(tree.imports.clone()));
        if parsed.is_err() {
            return Err(IncludeError(
                format!("Parsing failed on: {}", &name)
            ));
        }
        let parsed = parsed.unwrap();

        let incl = tree.include(parsed, &name);

        if incl.is_err() {
            return Err(IncludeError(format!(
                "failed to include file: {}", &name
            )))
        }
        Ok(())
    }
}