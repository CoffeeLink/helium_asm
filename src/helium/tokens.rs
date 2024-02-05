use std::fmt::{Display, Formatter};
use crate::helium::instructions::AsmInstruction;
use crate::helium::tokens::ValueKind::Word;

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone)]
pub enum TokenKind {
    // These tokens will just be eaten by the parser so its only for syntax enforcing
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

#[derive(Debug, Clone)]
pub enum ValueKind {
    Instruction(AsmInstruction),
    Integer(u16), // u16 for future support.
    Word(String)
}
impl ValueKind {
    pub fn get_word_value(self) -> Option<String>{
        match self {
            Word(w) => {Some(w)}
            _ => None
        }
    }
    pub fn get_int_value(self) -> Option<u16> {
        match self {
            ValueKind::Integer(i) => Some(i),
            _ => None
        }
    }
    pub fn get_instruction_code(self) -> Option<AsmInstruction> {
        match self {
            ValueKind::Instruction(i) => Some(i),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<ValueKind>,
    // For Debug/Error report purposes
    pub file: Option<String>,
    pub line: Option<u32>,
    pub char: Option<u32>
}

impl Token {
    pub fn new(
        token_kind: TokenKind,
        value: Option<ValueKind>,
        file: Option<String>,
        line: Option<u32>,
        char: Option<u32>
    ) -> Self {
        Self{
            kind: token_kind,
            value,

            file,
            line,
            char
        }
    }
    pub fn from_kind(token_kind: TokenKind) -> Self {
        Self {
            kind: token_kind,
            value: None,

            file: None,
            line: None,
            char: None,
        }
    }

    pub fn with_value(token_kind: TokenKind, value: ValueKind) -> Self {
        Self {
            kind: token_kind,
            value: Some(value),

            file: None,
            line: None,
            char: None
        }
    }

    pub fn set_file_name(mut self, name : String) -> Self {
        self.file = Some(name);
        self
    }

    pub fn set_position(mut self, line: u32, char: u32) -> Self {
        self.line = Some(line);
        self.char = Some(char);
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