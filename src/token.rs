/// Base of numeric literal encoding according to its prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Base {
    /// Literal starts with "0b".
    Binary = 2,
    /// Literal starts with "0o".
    Octal = 8,
    /// Literal doesn't contain a prefix.
    Decimal = 10,
    /// Literal starts with "0x".
    Hexadecimal = 16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Int {
        base: Base,
        empty_int: bool,
    },
    Float {
        base: Base,
        empty_exponent: bool,
    },
    Char {
        terminated: bool,
    },
    Byte {
        terminated: bool,
    },
    Str {
        terminated: bool,
    },
    ByteStr {
        terminated: bool,
    },
}


pub enum TokenKind {
    LineComment,
    BlockComment,
    Whitespace,
    Ident,
    InvalidIdent,
    RawIdent,
    UnknownPrefix,
    Literal {
        kind: LiteralKind,
        suffix_start: u32,
    },
    Semi,
    Comma,
    Dot,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    At,
    Pound,
    Tilde,
    Question,
    Colon,
    Dollar,
    Eq,
    Bang,
    Lt,
    Gt,
    Minus,
    And,
    Or,
    Plus,
    Star,
    Slash,
    Caret,
    Percent,
    Unknown,
    Eof,
}

pub struct Span {
    pub lo: u32,
    pub hi: u32,
}

pub struct Token {
    pub kind: TokenKind,
    pub span: Span
}
