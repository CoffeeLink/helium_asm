use crate::helium::instructions::AsmInstruction;

#[derive(Debug)]
pub enum TokenKind {
    Newline,
    Comma,
    SemiColon,

    Identifier,
    Label,
    Constant,

    Register,
    RegisterContent,
    Integer,
}

#[derive(Debug)]
pub enum ValueKind {
    Instruction(AsmInstruction),
    Integer(u16), // u16 for future support.
    Word(String)
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    value: Option<ValueKind>,
    // For Debug/Error report purposes
    file: Option<String>,
    line: Option<u32>,
    char: Option<u32>
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