#[derive(Debug, Default, Clone)]
pub enum TokenType {
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

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    #[default]
    Identifier,
    String {
        literal: String,
    },
    Number {
        number: f64,
    },

    Var,
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
    While,
    Super,
    True,
    This,

    Eof,
}

impl Default for &TokenType {
    fn default() -> Self {
        &TokenType::Identifier
    }
}
