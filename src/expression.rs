use std::fmt::{Display, Formatter};

use crate::token::Token;

pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: LiteralValue,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

#[derive(Clone, Default)]
pub enum LiteralValue {
    String(String),
    Number(f64),
    Bool(bool),
    #[default]
    Null,
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralValue::String(str) => {
                write!(f, "{str}")
            }
            LiteralValue::Number(num) => {
                write!(f, "{num}")
            }
            LiteralValue::Bool(bool) => {
                write!(f, "{bool}")
            }
            LiteralValue::Null => {
                write!(f, "null")
            }
        }
    }
}

impl Expr {
    pub fn accept<T>(&self, visitor: &dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => visitor.visit_binary_expr(left, operator, right),
            Expr::Grouping { expression } => visitor.visit_grouping_expr(expression),
            Expr::Literal { value } => visitor.visit_literal_expr(value),
            Expr::Unary { operator, right } => visitor.visit_unary_expr(operator, right),
        }
    }
}

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> T;
    fn visit_grouping_expr(&self, expression: &Expr) -> T;
    fn visit_literal_expr(&self, value: &LiteralValue) -> T;
    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> T;
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: Expr) -> String {
        expr.accept(self)
    }

    pub fn parenthesize(&self, name: &str, expressions: &[&Expr]) -> String {
        let mut string = String::new();
        string.push('(');
        string.push_str(name);
        for expression in expressions {
            string.push(' ');
            string.push_str(&expression.accept(self));
        }
        string.push(')');
        string
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&self, left: &Expr, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[left, right])
    }

    fn visit_grouping_expr(&self, expression: &Expr) -> String {
        self.parenthesize("grouping", &[expression])
    }

    fn visit_literal_expr(&self, value: &LiteralValue) -> String {
        value.to_string()
    }

    fn visit_unary_expr(&self, operator: &Token, right: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[right])
    }
}
