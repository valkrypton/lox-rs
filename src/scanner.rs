use crate::token::Token;
use crate::token_type::TokenType;
use crate::token_type::TokenType::{
    And, Bang, BangEqual, Class, Comma, Dot, Else, Equal, EqualEqual, False, For, Fun, Greater,
    GreaterEqual, If, LeftBrace, LeftParen, Less, LessEqual, Minus, Nil, Number, Or, Plus, Print,
    Return, RightBrace, RightParen, Semicolon, Slash, Star, Super, This, True, Var, While,
};
use crate::Lox;
use std::collections::HashMap;
use std::sync::LazyLock;

static KEYWORDS: LazyLock<HashMap<&'static str, TokenType>> = LazyLock::new(|| {
    HashMap::from([
        ("and", And),
        ("or", Or),
        ("class", Class),
        ("else", Else),
        ("false", False),
        ("true", True),
        ("for", For),
        ("fun", Fun),
        ("if", If),
        ("nil", Nil),
        ("print", Print),
        ("return", Return),
        ("this", This),
        ("super", Super),
        ("var", Var),
        ("while", While),
    ])
});

#[derive(Default)]
pub struct Scanner {
    pub source: String,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            line: 1,
            tokens: vec![],
            ..Default::default()
        }
    }

    pub fn scan_tokens(&mut self, lox: &mut Lox) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(lox);
        }
        self.tokens.push(Token {
            r#type: TokenType::Eof,
            lexeme: "".to_string(),
            line: self.line,
        });
    }

    fn scan_token(&mut self, lox: &mut Lox) {
        let c = self.advance();
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            '*' => self.add_token(Star),
            ';' => self.add_token(Semicolon),
            '!' => {
                let token = if self.r#match('=') { BangEqual } else { Bang };
                self.add_token(token);
            }
            '=' => {
                let token = if self.r#match('=') { EqualEqual } else { Equal };
                self.add_token(token);
            }
            '>' => {
                let token = if self.r#match('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.add_token(token);
            }
            '<' => {
                let token = if self.r#match('=') { LessEqual } else { Less };
                self.add_token(token);
            }
            '/' => {
                if self.r#match('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.r#match('*') {
                    if self.is_at_end() {
                        lox.error(self.line, "Unterminated block comment");
                    } else {
                        self.handle_block_comment();
                        self.advance();
                    }
                } else {
                    self.add_token(Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(lox),
            c => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_ascii_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    lox.error(self.line, "Unexpected character");
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn r#match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn add_token(&mut self, r#type: TokenType) {
        let text = unsafe { self.source.get_unchecked(self.start..self.current) };
        self.tokens
            .push(Token::new(r#type, text.to_string(), self.line))
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(&mut self, lox: &mut Lox) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            lox.error(self.line, "Unterminated string");
            return;
        }
        self.advance();
        let value = unsafe { self.source.get_unchecked(self.start + 1..self.current - 1) };
        self.add_token(TokenType::String {
            literal: String::from(value),
        })
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }
        let number = self.source.get(self.start..self.current).unwrap();

        self.add_token(Number {
            number: number.parse::<f64>().unwrap(),
        })
    }

    fn identifier(&mut self) {
        loop {
            match self.peek() {
                c if c.is_ascii_alphanumeric() || c == '_' => {
                    self.advance();
                }
                _ => {
                    let ident = self.source.get(self.start..self.current).unwrap();
                    let token = KEYWORDS.get(ident).unwrap_or_default().clone();
                    self.add_token(token);
                    break;
                }
            }
        }
    }

    fn handle_block_comment(&mut self) {
        let mut nesting = 1;
        while nesting > 0 {
            if self.peek() == '*' && self.peek_next() == '/' {
                nesting -= 1;
            }
            if self.peek() == '/' && self.peek_next() == '*' {
                nesting += 1;
            }
            self.advance();
        }
    }
}
