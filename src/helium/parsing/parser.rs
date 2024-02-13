use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::iter::Peekable;
use std::path::Path;
use std::slice::Iter;
use crate::helium::instructions;
use crate::helium::instructions::Argument;
use crate::helium::instructions::AsmInstruction::Halt;
use crate::helium::lexer::Lexer;
use crate::helium::parsing::constant_type::ConstantType;
use crate::helium::parsing::constant_type::ConstantType::{Unknown, Value};
use crate::helium::parsing::error::ParserError;
use crate::helium::parsing::error::ParserError::{ConstantCollision, Named, UnexpectedEOF, UnknownDirective};
use crate::helium::parsing::ParserError::{FileError, FileNotFound, IncludeLexError, UnknownIdentifier};
use crate::helium::parsing::program_element::ProgramElement;
use crate::helium::parsing::program_segment::ProgramSegment;
use crate::helium::parsing::program_tree::ProgramTree;
use crate::helium::tokens::{Token, TokenKind, ValueKind};
use crate::helium::tokens::TokenKind::{ConstantDeclaration, Label, Identifier, Integer, Register, Directive, Instruction, SemiColon, Newline, Comma};

pub struct Parser<'a> {
    file_name : String,

    tokens : Peekable<Iter<'a, Token>>,
    errors : Vec<ParserError>,
    current_segment : ProgramSegment,

    /// All the To-Be included files found in the main file.
    imports: BTreeSet<String>,
}

impl <'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], file_name : String) -> Self {
        Self {
            file_name,
            tokens: tokens.iter().peekable(),
            errors: vec![],
            current_segment: ProgramSegment::new("entry"),
            imports: Default::default(),
        }
    }

    pub fn parse(mut self, root: Option<&ProgramTree>) -> Result<ProgramTree, Vec<ParserError>> {
        let mut tree = ProgramTree::new(self.file_name.clone());
        if root.is_some() {
            self.current_segment.name = format!("__include_entry_{}__", self.file_name);
            tree = root.unwrap().clone();
            tree.add_include(self.file_name.clone());
        }
        self.consume_whitespaces();

        while self.tokens.peek().is_some() {
            self.parse_next(&mut tree);
        }


        // halt at the end of main, cuz there is no need to accidentally go into the next block.
        if root.is_none() {
            self.current_segment.elements.push(
                ProgramElement::Instruction(instructions::Instruction::new(Halt))
            )
        }

        // Add final segment.
        tree.segments.push(self.current_segment.clone());

        // Include all To-be-included files.
        for import in self.imports.clone() {
            tree = self.include(&mut tree, import).unwrap_or(tree);
        }

        if root.is_none() {
            tree.complete();
        }

        //Check for unresolved references
        let check = tree.check_all_resolved(root.is_some());
        if check.is_err() {
            for e in check.err().unwrap() {
                self.errors.push(
                    UnknownIdentifier{name: e}
                )
            }
        }

        if !self.errors.is_empty() { return Err(self.errors) }
        Ok(tree)
    }

    fn include(&mut self, tree : &mut ProgramTree, file : String) -> Option<ProgramTree>{
        let path = Path::new(&file);
        if !path.is_file() {
            self.errors.push(FileNotFound(file));
            return None;
        }

        let contents = read_to_string(path);
        let contents = if contents.is_err() {
            self.errors.push(FileError(file));
            return None;
        } else if let Ok(c) = contents {
            c
        } else {
            unreachable!()
        };
        // Lex file.
        let tokens = Lexer::new(&contents).lex();
        if let Err(e) = tokens {
            for err in e {
                self.errors.push(IncludeLexError(err));
            }
            return None;
        }
        let tokens = tokens.unwrap();
        // parse
        let new_tree = Parser::new(&tokens, file).parse(Some(tree));
        if let Err(e) = new_tree {
            for err in e {
                self.errors.push(err);
            }
            return None;
        }
        Some(new_tree.unwrap())
    }

    /// Parses the next anything
    fn parse_next(&mut self, tree : &mut ProgramTree) {
        let next = self.tokens.next();
        if next.is_none() { return }
        let next = next.unwrap();

        match next.kind {
            ConstantDeclaration => self.parse_constant(tree),
            Label => self.parse_label(tree, next),

            Identifier => self.parse_identifier(tree, next),
            Integer => self.parse_integer(next),
            Register => self.parse_register(next),

            Directive => self.parse_directive(tree, next),
            Instruction => self.parse_instruction(tree, next),

            _ => { /* consume token */ }
        }
    }

    fn parse_of_type(
        &mut self,
        expected: TokenKind
    ) -> Result<&Token, ParserError> {
        let next = self.tokens.next_if(|t|{
            t.kind == expected
        });

        if next.is_none() && self.tokens.peek().is_some() {
            return Err(
                ParserError::MismatchedTypes {
                    expected,
                    got: self.tokens.peek().unwrap().kind,
                }
            )
        } else if next.is_none() {
            return Err(UnexpectedEOF)
        }

        Ok(next.unwrap())
    }
    fn parse_constant(&mut self, tree : &mut ProgramTree) {
        let name = match self.parse_of_type(Identifier) {
            Ok(n) => {n.value.clone().unwrap().get_word_value().unwrap()}
            Err(e) => { self.errors.push(e); return; }
        };

        let value = match self.parse_of_type(Integer) {
            Ok(n) => {n.value.clone().unwrap().get_int_value().unwrap()}
            Err(e) => { self.errors.push(e); return; }
        };

        // Check collision
        if tree.has_const(&name) {
            self.errors.push(ConstantCollision {
                file: self.file_name.clone(),
                name,
            });
            return;
        }

        tree.add_const(name, Value(value))
    }
    fn parse_label(&mut self, tree: &mut ProgramTree, token: &Token) {
        let name = token.clone().value.unwrap().get_word_value().unwrap();
        if tree.has_const(&name) {
            self.errors.push(ConstantCollision {
                file: self.file_name.clone(),
                name,
            });
            return;
        }
        tree.add_const(name.clone(), ConstantType::Label);

        tree.segments.push(self.current_segment.clone());
        self.current_segment = ProgramSegment::new(&name);
    }
    fn parse_identifier(&mut self, tree : &mut ProgramTree, token: &Token) {
        let key = token.value.clone().unwrap().get_word_value().unwrap();

        if !tree.has_const(&key) {
            tree.add_const(key.clone(), Unknown)
        }
        self.current_segment.elements.push(
            ProgramElement::Identifier(key.clone())
        );
    }

    fn parse_integer(&mut self, token : &Token) {
        let value = token.value.clone().unwrap().get_int_value().unwrap();
        self.current_segment.elements.push(ProgramElement::Immediate(value));
    }
    fn parse_instruction(&mut self, tree : &mut ProgramTree, token: &Token) {
        let asm_code = token.value.clone().unwrap()
            .get_instruction_code().unwrap();
        let mut ins = instructions::Instruction::new(asm_code);

        while let Some(next) = self.tokens.peek() {
            if next.kind == Newline || next.kind == SemiColon { break }
            let token = self.tokens.next().unwrap().clone();

            if token.kind == Comma { continue }

            if token.kind != Identifier &&
                token.kind != Integer &&
                token.kind != Register {
                self.errors.push(
                    Named {
                        error: format!("Mismatched Types. expected: Identifier, Register Or Integer; found: {}", token.kind),
                    });
                continue;
            }

            if token.kind == Integer {
                let val = token.value.unwrap().get_int_value().unwrap();
                ins.args.push(Argument::Immediate(val));
            } else if token.kind == Identifier {
                let ident = token.value.unwrap().get_word_value().unwrap();

                if !tree.has_const(&ident) {
                    tree.add_const(ident.clone(), Unknown)
                }

                ins.args.push(
                    Argument::ImmediateIdentifier(ident)
                )
            } else {
                // Register case
                let value = token.value.unwrap();

                match value {
                    ValueKind::Instruction(_) => {}
                    ValueKind::Integer(i) => {
                        ins.args.push(Argument::Register(i))
                    }
                    ValueKind::Word(w) => {
                        if !tree.has_const(&w) {
                            tree.add_const(w.clone(), Unknown);
                        }
                        ins.args.push(Argument::RegisterIdentifier(w))
                    }
                }
            }
        }
        self.current_segment.elements.push(ProgramElement::Instruction(ins));

    }
    fn parse_register(&mut self, token : &Token) {
    // this isn't allowed so raise error.
        let reg_key = token.clone().value.unwrap();
        self.errors.push(match reg_key {
            ValueKind::Instruction(i) => {
                Named{ error: format!("Unexpected Token: Register({:?})", i) }}
            ValueKind::Integer(i) => {
                Named{ error: format!("Unexpected Token: Register({})", i) }}
            ValueKind::Word(w) => {
                Named{ error: format!("Unexpected Token: Register({})", w) }}
        })
    }
    fn parse_directive(&mut self, tree: &mut ProgramTree, token: &Token) {
        let directive_name = token.clone().value.unwrap().get_word_value().unwrap();
        match directive_name.as_str() {
            "no_predec" => tree.allow_defaults = false,
            "skipto" => {self.parse_skipto_directive(tree)},
            "include" => {self.parse_include_directive(tree)}
            _=> {
                self.errors.push(UnknownDirective {name: directive_name});
                self.consume_expression(); // Arguments
            }
        }
    }
    ///Consumes every token until a newline, SemiColon or EOF.
    fn consume_expression(&mut self) {
        for next in self.tokens.by_ref() {
            if next.kind == SemiColon || next.kind == Newline { break }
        }
    }
    fn parse_skipto_directive(&mut self, tree : &mut ProgramTree) {
        let addr = if let Ok(t) = self.parse_of_type(Integer) {
            t
        } else if let Err(e) = self.parse_of_type(Integer) {
            self.errors.push(e);
            return;
        } else {
            unreachable!()
        }.value
            .clone()
            .unwrap()
            .get_int_value()
            .unwrap();

        //remove anything after so the next will be an actual token.
        self.consume_whitespaces();

        let next = self.tokens.peek();
        if next.is_none() {
            self.errors.push(UnexpectedEOF);
        } else if next.is_some() && next.unwrap().kind != Label {
            // create new segment and set origin.
            let name = format!("__auto_label_{}_segment_{}__", &self.file_name, &tree.auto_label_id);
            tree.auto_label_id += 1;

            tree.segments.push(self.current_segment.clone());
            self.current_segment = ProgramSegment::with_origin(&name, addr);

        } else {
            // label.

            let name = match self.parse_of_type(Label) {
                Ok(t) => t,
                Err(e) => {
                    self.errors.push(e);
                    return;
                }
            }.value.clone().unwrap().get_word_value().unwrap();

            if tree.has_const(&name) {
                self.errors.push(ConstantCollision {
                    file: self.file_name.clone(),
                    name,
                });
                return;
            }
            tree.add_const(name.clone(), ConstantType::Label);

            tree.segments.push(self.current_segment.clone());
            self.current_segment = ProgramSegment::with_origin(&name, addr);
        }
    }
    fn parse_include_directive(&mut self, tree: &mut ProgramTree) {
        let possible_token = self.parse_of_type(Identifier);
        let include_name = if let Ok(t) = possible_token {
            t
        } else if let Err(e) = possible_token {
            self.errors.push(e);
            return;
        } else { unreachable!() }.clone().value
            .unwrap()
            .get_word_value()
            .unwrap();

        // check if includes contain this import.
        if tree.has_include(&include_name) { return; }
        self.imports.insert(include_name);

    }
    fn consume_whitespaces(&mut self) {
        while let Some(token) = self.tokens.peek() {
            if token.kind != Newline && token.kind != Comma && token.kind != SemiColon { break; }
            self.tokens.next();
        }
    }
}