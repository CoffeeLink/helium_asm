use crate::prelude::*;

use std::iter::Peekable;
use std::str::Chars;
use crate::lexer::token::{Token, TokenKind};
use crate::lexer::token::TokenKind::{Minus, Star, Plus, Modulo, Equals, Comma, SemiColon, OpenBraces, CloseBraces, SquareBracesOpen, SquareBracesClose, Divide, Identifier, Directive, Label, PointerTypeMarker, RegisterMarker, AddressMarker};

pub struct Lexer<'a> {
    file_name: &'a str,

    source: Peekable<Chars<'a>>,
    tokens: Vec<Token>,

    current_line: usize,
    current_char: usize,
}

impl <'a> Lexer<'a> {
    pub fn new(file_name: &'a str, source: &'a str) -> Self {
        Self {
            file_name,
            source: source.chars().peekable(),

            tokens: vec![],

            current_line: 0,
            current_char: 0,
        }
    }

    /// Tokenizes the given source and returns the tokens/errors(if errors are detected)
    pub fn tokenize(mut self) -> Vec<Token> {
        while self.source.peek().is_some() {
            self.next_token()
        }

        self.tokens
    }

    fn get_next(&mut self) -> Option<char> {
        let ch = self.source.next()?;

        if ch == '\n' {
            self.current_line += 1;
            self.current_char = 0;
        } else {
            self.current_char += 1;
        }

        Some(ch)
    }

    fn next_token(&mut self) {
        // match next token to rules.
        let next = self.get_next().unwrap();
        match next {
            '\n' => { /* nothing */ },

            '$' => self.new_token_with_kind(RegisterMarker),
            '@' => self.new_token_with_kind(AddressMarker),

            '+' => self.new_token_with_kind(Plus),
            '-' => self.new_token_with_kind(Minus),
            '*' => self.new_token_with_kind(Star),
            '%' => self.new_token_with_kind(Modulo),

            '=' => self.new_token_with_kind(Equals),

            ',' => self.new_token_with_kind(Comma),
            ';' => self.new_token_with_kind(SemiColon),

            '(' => self.new_token_with_kind(OpenBraces),
            ')' => self.new_token_with_kind(CloseBraces),

            '[' => self.new_token_with_kind(SquareBracesOpen),
            ']' => self.new_token_with_kind(SquareBracesClose),

            '/' => {
                // if another "/" is following than it's a comment.
                let peek = self.source.peek();
                if peek.is_some() && *peek.unwrap() == '/' {
                    // eat comment
                    self.get_next(); // eats the '/'
                    self.consume_comment()
                } else {
                    self.new_token_with_kind(Divide);
                }
            }

            other if other.is_whitespace() => { /* nothing */ },

            start => {
                let mut word = self.parse_string(start);

                // handle directives.

                if word == "ptr" || word == "PTR" {
                    self.new_token_with_kind(PointerTypeMarker);

                } else if word.starts_with('#') && !word.ends_with(':') {
                    word.remove(0);
                    self.new_token_with_value(Directive, word)

                } else if word.ends_with(':') {
                    word.remove(word.len()-1);
                    self.new_token_with_value(Label, word)

                } else {
                    self.new_token_with_value(Identifier, word)
                }
            }
        }
    }

    fn parse_string(&mut self, first_char: char) -> String {
        let mut out : String;
        let str_mode = first_char == '"';

        let mut escaped = false; // if true: create escape chars or just escape symbols.

        out = match str_mode {
            true => String::new(),
            false => String::from(first_char)
        };

        loop {
            let ahead = self.source.peek();

            if let None = ahead { break; } // if string ends than its over.
            let ahead = ahead.unwrap();

            if !Self::word_compatibility_check(ahead) && (!str_mode && !escaped) {
                break;
            }
            if *ahead == '\\' && !escaped {
                escaped = true;
                self.get_next();
                continue;

            } else if *ahead == '"' && !escaped {
                self.get_next(); // consume the char
                break; // end of string
            }

            out.push(self.get_next().unwrap());
            escaped = false;
        }
        out
    }

    fn word_compatibility_check(ch: &char) -> bool {
        ch.is_alphabetic() || ch.is_numeric() || *ch == '_' || *ch == ':' || *ch == '#'
    }
    fn consume_comment(&mut self) {
        // eat everything until a '\n' is eaten.
        loop {
            if self.source.peek().is_none() || *self.source.peek().unwrap() == '\n' {
                break;
            }
            self.get_next();
        }
    }

    fn new_token_with_kind(&mut self, kind: TokenKind) {
        // we store the last chars position rather than the first cuz the size of the token is unknown.
        let mut token = Token::with_kind(kind, self.current_line, self.current_char);
        token.set_file(self.file_name.into());

        self.tokens.push(token)
    }

    fn new_token_with_value(&mut self, kind: TokenKind, value: String) {
        let mut token = Token::with_value(kind, value, self.current_line, self.current_char);
        token.set_file(self.file_name.into());

        self.tokens.push(token)
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::lexer::Lexer;

    #[test]
    /// This test should verify that the position counting is still valid.
    /// Which is that lines are off by -1 and chars are accurate (so to index chrs[chr_count-1]) would be the last.
    fn test_position_counting() {
        let src = "This is a Line, AAA\n New Line!!!, AA \n 3rd line?";
        let mut lexer = Lexer::new("epic test", src);

        let mut lines_last = 0;
        let mut chars_last = 0;

        while lexer.get_next().is_some() {
            if lines_last != lexer.current_line {
                println!("Chars on line {}: {}", lines_last, chars_last);
                lines_last = lexer.current_line;
                chars_last = lexer.current_char;
            } else {
                chars_last = lexer.current_char;
            }
        }

        println!("Num lines: {}", lexer.current_line);
        println!("num chars on last: {}", lexer.current_char);

        assert_eq!(lexer.current_line, 2);
        assert_eq!(lexer.current_char, 10)
    }
}