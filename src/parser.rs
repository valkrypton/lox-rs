use crate::{
    error::{Error, Error::ParserError},
    expression::{Expr, LiteralValue},
    token::Token,
    token_type::{
        TokenType,
        TokenType::{
            Bang, BangEqual, EqualEqual, False, Greater, GreaterEqual, LeftParen, Less, LessEqual,
            Minus, Nil, Number, Plus, RightParen, Slash, Star, String, True,
        },
    },
};

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) ->Result<Expr,Error>{
        self.expression()
    }

    pub fn complete(&self)-> bool{
        self.is_at_end()
    }

    fn expression(&mut self) -> Result<Expr, Error> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, Error> {
        let mut expr = self.comparison()?;
        while self.r#match(&[BangEqual, EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, Error> {
        let mut expr = self.term()?;
        while self.r#match(&[Greater, GreaterEqual, Less, LessEqual]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, Error> {
        let mut expr = self.factor()?;
        while self.r#match(&[Plus, Minus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, Error> {
        let mut expr = self.unary()?;
        while self.r#match(&[Star, Slash]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, Error> {
        if self.r#match(&[Bang, Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary {
                operator,
                right: Box::new(right),
            });
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, Error> {
        match &self.peek().r#type {
            True => Ok(Expr::Literal {
                value: LiteralValue::Bool(true),
            }),
            False => Ok(Expr::Literal {
                value: LiteralValue::Bool(false),
            }),
            Nil => Ok(Expr::Literal {
                value: LiteralValue::Null,
            }),
            Number { number } => Ok(Expr::Literal {
                value: LiteralValue::Number(*number),
            }),
            String { literal } => Ok(Expr::Literal {
                value: LiteralValue::String(literal.clone()),
            }),
            LeftParen => {
                let expr = self.expression()?;
                self.consume(&RightParen, "Expect ) after expression")?;
                Ok(Expr::Grouping {
                    expression: Box::new(expr),
                })
            }
            _ => {
                Err(ParserError("expected expression".to_string()))
            }
        }
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token, Error> {
        if self.check(token_type) {
            return Ok(self.advance());
        }
        Err(ParserError(message.to_owned()))
    }

    fn r#match(&mut self, tokens_types: &[TokenType]) -> bool {
        for t_type in tokens_types {
            if self.check(t_type) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().r#type == *token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    fn is_at_end(&self) -> bool {
        self.peek().r#type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).expect("token must exist")
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).expect("token must exist")
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().r#type == TokenType::Semicolon {
                return;
            }
            match self.peek().r#type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {}
            }
            self.advance();
        }
    }
}
