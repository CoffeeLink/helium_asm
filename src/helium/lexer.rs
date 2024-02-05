use std::iter::Peekable;
use std::str::{Chars, FromStr};
use crate::helium::errors::Error;
use crate::helium::errors::Error::{ParseError, UnexpectedToken};
use crate::helium::instructions::AsmInstruction;
use crate::helium::tokens::{Token, ValueKind};
use crate::helium::tokens::TokenKind::{Comma, ConstantDeclaration, Directive, Identifier, Instruction, Integer, Label, Newline, Register, SemiColon};
use crate::helium::tokens::ValueKind::Word;

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>
}

impl <'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Self {
        let chars: Peekable<Chars<'a>> = source.chars().peekable();
        Self {
            source: chars
        }
    }
    pub fn lex(&mut self) -> Result<Vec<Token>, Vec<Error>> {
        let mut tokens : Vec<Token> = vec![];
        let mut errors : Vec<Error> = vec![];

        while let Some(_) = self.source.peek() {
            match self.next_token() {
                Ok(token) => {
                    match token {
                        None => { /* how */ }
                        Some(t) => tokens.push(t)
                    }
                }
                Err(err) => {
                    errors.push(err)
                }
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
        if next_token.is_none() { return Ok(None) }

        let next = self.source.next().unwrap();


        return Ok(Some(
            match next {
            '\n' => { Token::from_kind(Newline) }
            ';' => { Token::from_kind(SemiColon) }
            ',' => { Token::from_kind(Comma) }
            '-' => {
                // parse num as negative, then convert to u8 eq
                let word = match self.parse_word(None) {
                    Ok(w) => w,
                    Err(e) => {
                        return Err(e)
                    }
                };

                let num = Self::parse_int(&word, true);
                match num {
                    Ok(n) => {
                        Token::with_value(Integer, ValueKind::Integer(n))
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            '/' => {
                let after = self.source.peek();
                if after != Some(&'/') {
                    return Err(UnexpectedToken("Unexpected '/'".to_string()));
                }

                while let Some(ch) = self.source.next() {
                    if ch == '\n' { break; }
                }
                if self.source.peek() == None { return Ok(None); }

                Token::from_kind(Newline)
            }
            '$' => {
                // Register prefix
                let word = match self.parse_word(None) {
                    Ok(w) => w,
                    Err(e) => {
                        return Err(e)
                    }
                };
                // Check if its a number
                if word.chars().next().unwrap().is_numeric() {
                    let i_val = match Self::parse_int(&word, false) {
                        Ok(i) => i,
                        Err(e) => {return Err(e)}
                    };
                    Token::with_value(Register, ValueKind::Integer(i_val))
                } else { // its not a number so its an identifier.
                    Token::with_value(Register, Word(word))
                }
            }
            '#' => {
                // Directive prefix.
                let word = match self.parse_word(None) {
                    Ok(w) => w,
                    Err(e) => {
                        return Err(e)
                    }
                };
                // Check if its longer than 0 chars
                if word.len() == 0 {
                    return Err(ParseError("Directive has invalid name".to_string()))
                }

                Token::with_value(Directive, Word(word))
            }
            first => {
                let word = match self.parse_word(Some(first)) {
                    Ok(w) => w,
                    Err(e) => {
                        return Err(e)
                    }
                };

                if let Some(ins) = AsmInstruction::match_instruction(&word) {
                    // Instruction token
                    Token::with_value(Instruction, ValueKind::Instruction(ins))
                }
                else if first.is_numeric() {
                    let num = match Self::parse_int(&word, false) {
                        Ok(n) => n,
                        Err(e) => {
                            return Err(e);
                        }
                    };
                    Token::with_value(Integer, ValueKind::Integer(num))
                } else if word.ends_with(":") {
                    Token::with_value(Label, Word(word.replace(":", "")))
                } else if word == "const" || word == "CONST" {
                    Token::from_kind(ConstantDeclaration)
                } else {
                    Token::with_value(Identifier, Word(word))
                }
            }
        }
        ))
    }

    fn parse_word(&mut self, start_char : Option<char>) -> Result<String, Error> {
        let mut word = String::new();
        let mut is_string_format = false;

        if start_char.is_some() {
            if &start_char.unwrap() == &'"' {
                is_string_format = true
            } else if !Self::is_const_compatible(&start_char.unwrap()) {
                return Err(ParseError(format!("Incompatible char: '{}'", start_char.unwrap())))
            } else {
                word.push(start_char.unwrap())
            }
        }

        while let Some(ch) = self.source.peek() {
            if !is_string_format {
                if ch.is_whitespace() || ch == &';' || ch == &',' { break; }
                if Self::is_const_compatible(ch) {
                    word.push(self.source.next().unwrap())
                } else {
                    let cha = ch.clone();
                    self.source.next(); // Consume so it doesnt give 2 errors
                    return Err(ParseError(format!("Incompatible char: '{}'", cha)))
                }
            } else {
                if ch == &'"' {
                    self.source.next(); // Consume the "
                    break;
                }

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

    fn is_const_compatible(ch : &char) -> bool {
        ch.is_alphanumeric() || ch == &'_' || ch == &':'
    }

    fn parse_int(source: &str, neg : bool) -> Result<u16, Error>{
        if source.starts_with("0x") {
            let mut val = source.replace("0x", "");
            val = val.replace("_", "");
            let res = u16::from_str_radix(&val, 16);
            match res {
                Ok(res) => Ok(res),
                Err(_) => Err(ParseError(format!("Could not parse to int: {}", val)))
            }
        }
        else if source.starts_with("0b") {
            let mut val = source.replace("0b", "");
            val = val.replace("_", "");
            let res = u16::from_str_radix(&val, 2);
            match res {
                Ok(res) => Ok(res),
                Err(_) => Err(ParseError(format!("Could not parse to int: {}", val)))
            }
        } else {
            let val = source.replace("_", "");
            let res = u16::from_str(&val);
            match res {
                Ok(res) => {
                    if neg { Ok(!res + 1)} else { Ok(res) }
                },
                Err(_) => Err(ParseError(format!("Could not parse to int: {}", val)))
            }
        }
    }
}