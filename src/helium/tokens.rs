use crate::helium::instructions::AsmInstruction;
use crate::helium::tokens::ValueKind::Word;
use std::fmt::{Display, Formatter};
use crate::helium::position::Position;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum TokenKind {
    // These tokens will just be eaten by the parsing, so it's only for syntax enforcing
    Newline,
    Comma,
    SemiColon,

    Instruction,
    Identifier,
    Label,
    ConstantDeclaration,
    Directive,

    Register,
    Integer,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name: &str = match self {
            TokenKind::Newline => "\\n",
            TokenKind::Comma => ",",
            TokenKind::SemiColon => ";",
            TokenKind::Instruction => "Instruction",
            TokenKind::Identifier => "Identifier",
            TokenKind::Label => "Label",
            TokenKind::ConstantDeclaration => "const",
            TokenKind::Directive => "Directive",
            TokenKind::Register => "Register",
            TokenKind::Integer => "Integer",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone)]
pub enum ValueKind {
    Instruction(AsmInstruction),
    Integer(u16), // u16 for future support.
    Word(String),
}
impl ValueKind {
    pub fn get_word_value(self) -> Option<String> {
        match self {
            Word(w) => Some(w),
            _ => None,
        }
    }
    pub fn get_int_value(self) -> Option<u16> {
        match self {
            ValueKind::Integer(i) => Some(i),
            _ => None,
        }
    }
    pub fn get_instruction_code(self) -> Option<AsmInstruction> {
        match self {
            ValueKind::Instruction(i) => Some(i),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<ValueKind>,
    // For Debug/Error report purposes
    pub file: Option<String>,
    pub position : Option<Position>
}

impl Token {
    pub fn new(
        token_kind: TokenKind,
        value: Option<ValueKind>,
        file: Option<String>,
        position: Option<Position>
    ) -> Self {
        Self {
            kind: token_kind,
            value,

            file,
            position
        }
    }
    pub fn from_kind(token_kind: TokenKind) -> Self {
        Self {
            kind: token_kind,
            value: None,

            file: None,
            position: None
        }
    }

    pub fn with_value(token_kind: TokenKind, value: ValueKind) -> Self {
        Self {
            kind: token_kind,
            value: Some(value),

            file: None,
            position: None
        }
    }

    pub fn set_file_name(mut self, name: String) -> Self {
        self.file = Some(name);
        self
    }

    pub fn set_position(mut self, line: usize, char: usize, len: usize) -> Self {
        self.position = Some(Position::new(line, char, len));
        self
    }

    pub fn set_value(mut self, value: ValueKind) -> Self {
        self.value = Some(value);
        self
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.value.is_some() {
            write!(f, "{:?}: {:?}", self.kind, self.value.as_ref().unwrap())
        } else {
            write!(f, "{:?}", self.kind)
        }
    }
}
