#[derive(Debug, PartialEq)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greather,
    GreatherEqual,
    Less,
    LessEqual,

    // Literals
    Identifiter,
    String(String),
    Number(f64),

    Keyword(Keyword),

    Error(Error),
}

#[derive(PartialEq, Debug)]
pub enum Keyword {
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(PartialEq, Debug)]
pub enum Error {
    UnexpectedCharacter,
    UnterminatedString,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: u32,
}

impl Token {
    pub fn new(lexeme: String, line: u32, kind: TokenKind) -> Self {
        Self { kind, lexeme, line }
    }
}
