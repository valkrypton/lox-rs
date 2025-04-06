use std::fmt::{Display, Formatter};

use crate::token_type::TokenType;
#[derive(Default, Clone)]
pub struct Token {
    pub r#type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, line: usize) -> Self {
        Token {
            r#type,
            lexeme,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {} ", self.r#type, self.lexeme)
    }
}
