use crate::{token::Token, token_type::TokenType};

pub enum Error {
    ParserError,
}

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn parser_error(token: &Token, message: &str) {
    if token.r#type == TokenType::Eof {
        report(token.line, " at end", message)
    } else {
        let location = format!(" at '{}'", token.lexeme);
        report(token.line, &location, message)
    }
}

pub fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
}
