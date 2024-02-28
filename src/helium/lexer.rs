#![deprecated]

use crate::helium::errors::Error;
use crate::helium::errors::Error::{ParseError, UnexpectedToken};
use crate::helium::instructions::AsmInstruction;
use crate::helium::tokens::TokenKind::{
    Comma, ConstantDeclaration, Directive, Identifier, Instruction, Integer, Label, Newline,
    Register, SemiColon,
};
use crate::helium::tokens::ValueKind::Word;
use crate::helium::tokens::{Token, ValueKind};
use std::iter::Peekable;
use std::str::{Chars, FromStr};

pub struct Lexer<'a> {
    file_name: &'a str,
    source: Peekable<Chars<'a>>,
    line_count: u32,
    current_char: u32
}

impl<'a> Lexer<'a> {
    pub fn new(name: &'a str, source: &'a str) -> Self {
        let chars: Peekable<Chars<'a>> = source.chars().peekable();
        Self {
            file_name: name,
            source: chars,
            line_count: 1,
            current_char: 0
        }
    }
    pub fn lex(&mut self) -> Result<Vec<Token>, Vec<Error>> {
        let mut tokens: Vec<Token> = vec![];
        let mut errors: Vec<Error> = vec![];

        while self.source.peek().is_some() {
            match self.next_token() {
                Ok(token) => {
                    match token {
                        None => { /* how */ }
                        Some(t) => tokens.push(t),
                    }
                }
                Err(err) => errors.push(err),
            }
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>, Error> {
        self.consume_whitespaces();

        let next_token = self.source.peek();
        if next_token.is_none() {
            return Ok(None);
        }

        let next = self.source.next().unwrap();

        self.current_char += 1;

        return Ok(Some(
            match next {
                '\n' => {
                    self.line_count += 1;
                    self.current_char = 0;
                    Token::from_kind(Newline)
                },
                ';' => Token::from_kind(SemiColon),
                ',' => Token::from_kind(Comma),
                '-' => {
                    // parse num as negative, then convert to u8 eq
                    let word = match self.parse_word(None) {
                        Ok(w) => w,
                        Err(e) => return Err(e),
                    };

                    let num = Self::parse_int(&word, true, self.line_count);
                    match num {
                        Ok(n) => Token::with_value(Integer, ValueKind::Integer(n)),
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                '/' => {
                    let after = self.source.peek();
                    if after != Some(&'/') {
                        return Err(UnexpectedToken("Unexpected '/'".to_string(), self.line_count));
                    }

                    for ch in self.source.by_ref() {
                        if ch == '\n' {
                            self.line_count += 1;
                            self.current_char = 0;
                            break;
                        }
                    }
                    if self.source.peek().is_none() {
                        return Ok(None);
                    }

                    Token::from_kind(Newline)
                }
                '$' => {
                    // Register prefix
                    let word = match self.parse_word(None) {
                        Ok(w) => w,
                        Err(e) => return Err(e),
                    };
                    // Check if it's a number
                    if word.chars().next().unwrap().is_numeric() {
                        let i_val = match Self::parse_int(&word, false, self.line_count) {
                            Ok(i) => i,
                            Err(e) => return Err(e),
                        };
                        Token::with_value(Register, ValueKind::Integer(i_val))
                    } else {
                        // it's not a number, so it's an identifier.
                        Token::with_value(Register, Word(word))
                    }
                }
                '#' => {
                    // Directive prefix.
                    let word = match self.parse_word(None) {
                        Ok(w) => w,
                        Err(e) => return Err(e),
                    };
                    // Check if it's longer than 0 chars
                    if word.is_empty() {
                        return Err(ParseError("Directive has invalid name".to_string(), self.line_count));
                    }

                    Token::with_value(Directive, Word(word))
                }
                first => {
                    let word = match self.parse_word(Some(first)) {
                        Ok(w) => w,
                        Err(e) => return Err(e),
                    };

                    if let Some(ins) = AsmInstruction::match_instruction(&word) {
                        // Instruction token
                        Token::with_value(Instruction, ValueKind::Instruction(ins))
                    } else if first.is_numeric() {
                        let num = match Self::parse_int(&word, false, self.line_count) {
                            Ok(n) => n,
                            Err(e) => {
                                return Err(e);
                            }
                        };
                        Token::with_value(Integer, ValueKind::Integer(num))
                    } else if word.ends_with(':') {
                        Token::with_value(Label, Word(word.replace(':', "")))
                    } else if word == "const" || word == "CONST" {
                        Token::from_kind(ConstantDeclaration)
                    } else {
                        Token::with_value(Identifier, Word(word))
                    }
                }
            }.set_file_name(self.file_name.to_string())
                .set_position(self.line_count, self.current_char)
        ));
    }

    fn parse_word(&mut self, start_char: Option<char>) -> Result<String, Error> {
        let mut word = String::new();
        let mut is_string_format = false;

        if start_char.is_some() {
            if start_char.unwrap() == '"' {
                is_string_format = true
            } else if !Self::is_const_compatible(&start_char.unwrap()) {
                return Err(ParseError(format!(
                    "Incompatible char: '{}'",
                    start_char.unwrap()
                ), self.line_count
                ));
            } else {
                word.push(start_char.unwrap())
            }
        }

        while let Some(ch) = self.source.peek() {
            if !is_string_format {
                if ch.is_whitespace() || ch == &';' || ch == &',' {
                    break;
                }
                if Self::is_const_compatible(ch) {
                    self.current_char += 1;
                    word.push(self.source.next().unwrap())
                } else {
                    let cha = *ch;
                    self.source.next(); // Consume so it doesnt give 2 errors
                    self.current_char += 1;
                    return Err(ParseError(format!("Incompatible char: '{}'", cha), self.line_count));
                }
            } else {
                if ch == &'"' {
                    self.source.next(); // Consume the "
                    self.current_char += 1;
                    break;
                }

                self.current_char +=1;
                word.push(self.source.next().unwrap());
            }
        }
        Ok(word)
    }

    fn consume_whitespaces(&mut self) {
        let mut next = self.source.peek();
        while next.is_some() && next.unwrap().is_whitespace() && next.unwrap() != &'\n' {
            self.source.next();
            next = self.source.peek();
        }
    }

    fn is_const_compatible(ch: &char) -> bool {
        ch.is_alphanumeric() || ch == &'_' || ch == &':'
    }

    fn parse_int(source: &str, neg: bool, line : u32) -> Result<u16, Error> {
        if source.starts_with("0x") {
            let mut val = source.replace("0x", "");
            val = val.replace('_', "");
            let res = u16::from_str_radix(&val, 16);
            match res {
                Ok(res) => Ok(res),
                Err(_) => Err(ParseError(format!("Could not parse to int: {}", val), line)),
            }
        } else if source.starts_with("0b") {
            let mut val = source.replace("0b", "");
            val = val.replace('_', "");
            let res = u16::from_str_radix(&val, 2);
            match res {
                Ok(res) => Ok(res),
                Err(_) => Err(ParseError(format!("Could not parse to int: {}", val), line)),
            }
        } else {
            let val = source.replace('_', "");
            let res = u16::from_str(&val);
            match res {
                Ok(res) => {
                    if neg {
                        Ok(!res + 1)
                    } else {
                        Ok(res)
                    }
                }
                Err(_) => Err(ParseError(format!("Could not parse to int: {}", val), line)),
            }
        }
    }
}
