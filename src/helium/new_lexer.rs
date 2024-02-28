use std::iter::Peekable;
use std::num::ParseIntError;
use std::str::{Chars, FromStr};
use crate::helium::errors::HeliumError;
use crate::helium::instructions::AsmInstruction;
use crate::helium::tokens::{Token, TokenKind, ValueKind};
use crate::helium::tokens::TokenKind::{Comma, ConstantDeclaration, Directive, Identifier, Instruction, Integer, Label, Newline, Register, SemiColon};
use crate::helium::tokens::ValueKind::Word;

// TODO: Line by line token analysis, check if a line causes an error, if so
//  return the error but not the token stream

/// The new class for tokenizing the file.
pub struct Lexer<'a> {
    file_name: &'a str,

    source: Peekable<Chars<'a>>,
    pub tokens: Vec<Token>,
    pub errors: Vec<HeliumError>,

    line: u32,
    char: u32
}
impl <'a> Lexer <'a> {
    pub fn new(name: &'a str, file_content: &'a str) -> Self {
        Self {
            file_name: name,
            source: file_content
                .chars()
                .peekable(),
            tokens: vec![],
            errors: vec![],

            line: 0,
            char: 0
        }
    }
    pub fn tokenize(&mut self) -> Result<Vec<Token>, Vec<HeliumError>> {
        while self.source.peek().is_some() {
            self.next_token();
        }

        if !self.errors.is_empty() {
            return Err(self.errors.clone())
        }

        Ok(self.tokens.clone())
    }

    fn next_token(&mut self) {
        self.consume_whitespaces();

        if self.source.peek().is_none() {
            return;
        }

        let next = self.source.next().unwrap();
        self.char += 1;

        match next {
            '\n' => {
                self.line += 1;
                self.char = 0;
                self.new_token(Newline);
            }
            ';' => self.new_token(SemiColon),
            ',' => self.new_token(Comma),
            '-' => self.parse_negative(),
            '/' => self.parse_comment(),
            '$' => self.parse_register(),
            '#' => self.parse_directive(),

            f_char => self.parse_any(f_char)
        }
    }

    /// parses a word into a token by checking the value.
    fn parse_any(&mut self, first_char : char) {
        let mut word = match self.parse_word(Some(first_char)) {
            Ok(w) => w,
            Err(e) => {
                self.errors.push(e);
                return;
            }
        };

        if let Some(ins) = AsmInstruction::match_instruction(&word) {
            self.new_token_with_value(Instruction, ValueKind::Instruction(ins))
        } else if first_char.is_numeric() {
            let num = match self.parse_int(&word, false) {
                Ok(n) => n,
                Err(_) => {
                    self.errors.push(HeliumError::new("Man wtf".to_string(), self.line, self.char));
                    return;
                }
            };
            self.new_token_with_value(Integer, ValueKind::Integer(num));
        } else if word.ends_with(':') {
            word.remove(word.len() - 1);
            self.new_token_with_value(Label, Word(word));
        } else if word == "const" || word == "CONST" {
            self.new_token(ConstantDeclaration)
        } else {
            self.new_token_with_value(Identifier, Word(word));
        }
    }
    fn parse_directive(&mut self) {
        let word = match self.parse_word(None) {
            Ok(w) => w,
            Err(err) => {
                self.errors.push(err);
                return;
            }
        };
        if word.is_empty() {
            self.errors.push(HeliumError::new(
                "Invalid directive name: empty prefix".to_string(),
                self.line,
                self.char
            ));
            return;
        }
        self.new_token_with_value(Directive, Word(word))
    }
    fn parse_register(&mut self) {
        let word = match self.parse_word(None) {
            Ok(w) => w,
            Err(e) => {
                self.errors.push(e);
                return;
            }
        };

        if word.chars().next().unwrap().is_numeric() {
            let int = match self.parse_int(&word, false) {
                Ok(i) => i,
                Err(_) => {
                    self.errors.push(
                        HeliumError::new(
                            "bruh, epic int fail.".to_string(),
                            self.line,
                            self.char
                        )
                    );
                    return;
                }
            };
            self.new_token_with_value(Register, ValueKind::Integer(int));
        } else {
            self.new_token_with_value(Register, Word(word));
        }
    }
    fn parse_comment(&mut self) {
        // check for '/' then consume until newline. (don't consume the newline)
        let next = self.source.peek();
        if next != Some(&'/') {
            self.errors.push(HeliumError::new("Epic comment fail.".to_string(), self.line, self.char))
        }
        self.source.next();

        for ch in self.source.by_ref() {
            if ch == '\n' {
                self.line += 1;
                self.char = 0;
                break;
            }
        }
        self.new_token(Newline);
    }
    fn parse_negative(&mut self) {
        // parse the following number into a token.
        let word_out = self.parse_word(None);

        if let Err(e) = word_out {
            self.errors.push(e);
            return;
        }

        let word = word_out.unwrap();
        let num_out = self.parse_int(&word, true);

        if num_out.is_err() {
            self.errors.push(HeliumError::new("ParseInt Error".to_string(), self.line, self.char));
            return;
        }

        let num = num_out.unwrap();

        self.new_token_with_value(Integer, ValueKind::Integer(num));
    }
    fn new_token(&mut self, kind: TokenKind) {
        self.tokens.push(
            Token::from_kind(kind)
            .set_file_name(self.file_name.to_string())
            .set_position(self.line, self.char)
        )
    }
    fn new_token_with_value(&mut self, kind: TokenKind, value: ValueKind) {
        self.tokens.push(
            Token::with_value(kind, value)
            .set_file_name(self.file_name.to_string())
            .set_position(self.line, self.char)
        )
    }
    fn parse_word(&mut self, start_char: Option<char>) -> Result<String, HeliumError> {
        let mut word = String::new();
        let mut is_string = false;

        if start_char.is_some() {
            if start_char.unwrap() == '"' { is_string = true }
            else if !Self::is_const_compatible(&start_char.unwrap()) {
                return Err(HeliumError::new(
                    format!("Incompatible char: {}", start_char.unwrap()),
                    self.line,
                    self.char
                ))
            } else {
                word.push(start_char.unwrap())
            }
        }

        while let Some(ch) = self.source.peek() {
            if !is_string {
                if ch.is_whitespace() || *ch == ';' || *ch == ',' { break; }
                if Self::is_const_compatible(ch) {
                    self.char += 1;
                    word.push(self.source.next().unwrap());
                } else {
                    let ch = self.source.next().unwrap();
                    self.char += 1;
                    return Err(HeliumError::new(
                        format!("Incompatible char: {}", ch),
                        self.line,
                        self.char
                    ))
                }
            } else {
                if *ch == '"' {
                    self.source.next();
                    self.char += 1;
                    break;
                }

                if *ch == '\n' {
                    self.line += 1;
                    self.char = 0;
                }

                self.char += 1;
                word.push(self.source.next().unwrap());
            }
        }

        Ok(word)
    }
    fn parse_int(&mut self, word : &str, negative: bool) -> Result<u16, ParseIntError> {
        if word.starts_with("0x") {
            u16::from_str_radix(&word.replace("0x", "")
                .replace('_', ""), 16)
        } else if word.starts_with("0b") {
            u16::from_str_radix(&word.replace("0b", "")
                .replace('_', ""), 2)
        } else {
            let value = u16::from_str(&word.replace('_', ""));
            if let Ok(v) = value {
                if negative {
                    Ok(!v + 1)
                } else {
                    Ok(v)
                }
            } else {
                Err(value.unwrap_err())
            }
        }
    }
    fn consume_whitespaces(&mut self) {
        let mut next = self.source.peek();
        while next.is_some() && next.unwrap().is_whitespace() && next.unwrap() != &'\n' {
            self.source.next();
            self.char += 1;

            next = self.source.peek();
        }
    }
    fn is_const_compatible(ch: &char) -> bool {
        ch.is_alphanumeric() || *ch == '_' || *ch == ':'
    }
}

#[cfg(test)]
mod tests {
    use crate::helium::new_lexer::Lexer;
    use crate::helium::tokens::TokenKind::{ConstantDeclaration, Identifier, Integer, Newline};

    #[test]
    fn whitespace_consumption() {
        let input = String::from("      \n   \n");
        let mut lexer = Lexer::new("Test input", &input);
        let out = lexer.tokenize();
        assert!(out.is_ok());

        let tokens = out.unwrap();

        assert_eq!(tokens.len(), 2);

        assert_eq!(tokens[0].kind, Newline);
        assert_eq!(tokens[1].kind, Newline);
    }

    #[test]
    fn empty_file() {
        let input = String::from("");
        let mut lexer = Lexer::new("Test input", &input);
        let out = lexer.tokenize();
        assert!(out.is_ok());

        let tokens = out.unwrap();

        assert!(tokens.is_empty());
    }
    #[test]
    fn consume_comment() {
        let input = "//hello";
        let mut lex = Lexer::new("Test input", input);
        let out = lex.tokenize();

        assert!(out.is_ok());
        let tokens = out.unwrap();

        assert_eq!(tokens.len(), 1)
    }

    #[test]
    fn parse_negative_int() {
        let input = "-100";
        let mut lex = Lexer::new("Test input", input);
        let out = lex.tokenize();

        assert!(out.is_ok());
        let tokens = out.unwrap();

        assert_eq!(tokens[0].kind, Integer)
    }

    #[test]
    fn test_const_declarations() {
        let input = "const Const CONST"; // should only return 2 constant tokens and 1 identifier.

        let mut lexer = Lexer::new("Test input", input);
        let out = lexer.tokenize();
        assert!(out.is_ok());

        let tokens = out.unwrap();

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, ConstantDeclaration);
        assert_eq!(tokens[1].kind, Identifier);
        assert_eq!(tokens[2].kind, ConstantDeclaration);
    }
}