
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

pub struct Token {
    kind: TokenKind,
    value: Option<String>,
    // For Debug/Error report purposes
    file: Option<String>,
    line: Option<u32>,
    char: Option<u32>
}

impl Token {
    pub fn new(
        token_kind: TokenKind,
        value: Option<String>,
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

    pub fn with_value(token_kind: TokenKind, value: String) -> Self {
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

    pub fn set_value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }
}