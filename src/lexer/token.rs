
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum TokenKind {
    Label, // <label name>:
    Directive, // #<DirectiveName> <Value>, ... ,<Value>;

    Identifier, // Any string that doesn't match other rules.
    String, // Any token marked by a "<contents>"
    RegisterMarker, // $ <Str>
    AddressMarker, // @ <Str>

    PointerTypeMarker, // "ptr" / "PTR"

    // these will become an 'Address' if a value is iven inside, like LD 10, [23]; // this will load 10 at 23, (this is an example)
    // but things like LD 10, [$A, $B] works too if allowed.
    SquareBracesOpen, //  [
    SquareBracesClose, // ]

    Plus, //         +
    Minus, //        -
    Star,//          *
    Divide, //      '/'
    Modulo, //       %

    OpenBraces, //   (
    CloseBraces, //  )

    SemiColon, //    ;
    Comma, //        ,

    Equals, //       =
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>,

    pub line: usize,
    pub char: usize,

    pub file: Option<String>
}

impl Token {
    #[inline]
    pub fn new(kind: TokenKind, value: Option<String>, line: usize, char: usize) -> Self {
        Self { kind, value, line, char, file: None }
    }

    #[inline]
    pub fn with_kind(token_kind: TokenKind, line: usize, char: usize ) -> Self {
        Self { kind: token_kind, line, char, value: None, file: None}
    }

    #[inline]
    pub fn with_value(kind: TokenKind, value: String, line: usize, char: usize) -> Self {
        Self { kind, value: Some(value), line, char, file: None }
    }

    #[inline]
    pub fn set_file(&mut self, file: String) {
        self.file = Some(file);
    }
}