use crate::token_type::TokenType;
use std::fmt::{ Display, Formatter};
#[derive(Default)]
pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    fn new(r#type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            r#type,
            lexeme,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.r#type, self.lexeme)
    }
}
