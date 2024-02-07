use std::iter::Peekable;
use std::slice::Iter;
use crate::helium::parserv2::constant_type::ConstantType;
use crate::helium::parserv2::constant_type::ConstantType::{Unknown, Value};
use crate::helium::parserv2::error::ParserError;
use crate::helium::parserv2::error::ParserError::{ConstantCollision, UnexpectedEOF};
use crate::helium::parserv2::program_element::ProgramElement;
use crate::helium::parserv2::program_segment::ProgramSegment;
use crate::helium::parserv2::program_tree::ProgramTree;
use crate::helium::tokens::{Token, TokenKind};
use crate::helium::tokens::TokenKind::{
    ConstantDeclaration, Label,
    Identifier, Integer, Register,
    Directive, Instruction
};

struct Parser<'a> {
    file_name : String,

    tokens : Peekable<Iter<'a, Token>>,
    errors : Vec<ParserError>,
    current_segment : ProgramSegment
}

impl <'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], file_name : String) -> Self {
        Self {
            file_name,
            tokens: tokens.iter().peekable(),
            errors: vec![],
            current_segment: ProgramSegment::new("entry")
        }
    }

    pub fn parse(mut self) -> Result<ProgramTree, Vec<ParserError>> {
        let mut tree = ProgramTree::new(self.file_name.clone());

        while self.tokens.peek().is_some() {
            self.parse_next(&mut tree);
        }

        if !self.errors.is_empty() { return Err(self.errors) }
        Ok(tree)
    }

    /// Parses the next anything
    fn parse_next(&mut self, mut tree : &mut ProgramTree) {
        let next = self.tokens.next();
        if next.is_none() { return }
        let next = next.unwrap();

        match next.kind {
            ConstantDeclaration => self.parse_constant(&mut tree),
            Label => self.parse_label(&mut tree, next),

            Identifier => self.parse_identifier(&mut tree, next),
            Integer => self.parse_integer(&mut tree, next),
            Register => self.parse_register(&mut tree, next),

            Directive => self.parse_directive(&mut tree, next),
            Instruction => self.parse_instruction(&mut tree, next),

            _ => {/* consume token */}
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
            ProgramElement::Identifier(key)
        )
    }

    fn parse_integer(&mut self, mut tree : &ProgramTree, token : &Token) {

    }

    fn parse_instruction(&mut self, mut tree : &ProgramTree, token: &Token) {

    }

    fn parse_register(&mut self, mut tree : &ProgramTree, token : &Token) {

    }

    fn parse_directive(&mut self, mut tree: &ProgramTree, token: &Token) {

    }

    fn consume_whitespaces(&mut self) {

    }
}