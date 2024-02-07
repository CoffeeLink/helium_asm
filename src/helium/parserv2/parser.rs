use std::iter::Peekable;
use std::slice::Iter;
use std::string::ParseError;
use crate::helium::parserv2::program_tree::ProgramTree;
use crate::helium::tokens::Token;

struct Parser<'a> {
    file_name : String,

    tokens : Peekable<Iter<'a, Token>>,
    errors : Vec<ParseError>
}

impl <'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], file_name : String) -> Self {
        Self {
            file_name,
            tokens: tokens.iter().peekable(),
            errors: vec![]
        }
    }

    pub fn parse(mut self) -> Result<ProgramTree, Vec<ParseError>> {
        let mut tree = ProgramTree::new(self.file_name.clone());

        while self.tokens.peek().is_some() {
            self.parse_next(&mut tree);
        }

        if !self.errors.is_empty() { return Err(self.errors) }
        Ok(tree)
    }

    /// Parses the next anything
    fn parse_next(&mut self, tree : &mut ProgramTree) {

    }

    fn parse_instruction(&mut self, tree : &mut ProgramTree) {

    }

    fn parse_directive(&mut self, tree: &mut ProgramTree) {

    }

    fn consume_whitespaces(&mut self) {

    }
}