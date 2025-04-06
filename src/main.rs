mod expression;
mod parser;
mod scanner;
mod token;
mod token_type;
mod error;

use std::{
    error::Error,
    io::{self, BufRead, Write},
    process::exit,
};

use crate::{
    expression::{AstPrinter, Expr, LiteralValue},
    scanner::Scanner,
    token::Token,
    token_type::TokenType,
};

pub struct Lox {
    pub had_error: bool,
}

fn main() {
    // let args = env::args().collect::<Vec<_>>();
    // let mut lox = Lox { had_error: false };
    // match args.len().cmp(&2) {
    //     Ordering::Greater => {
    //         println!("Usage: rlox [script]");
    //         exit(64);
    //     }
    //     Ordering::Equal => run_file(&mut lox, &args[1]).expect("error while reading file"),
    //     _ => run_prompt(&mut lox),
    // }
    let expression = Expr::Binary {
        left: Box::new(Expr::Unary {
            operator: Token::new(TokenType::Minus, "-".to_owned(), 1),
            right: Box::new(Expr::Literal {
                value: LiteralValue::Number(123f64),
            }),
        }),
        operator: Token::new(TokenType::Star, "*".to_owned(), 1),
        right: Box::new(Expr::Grouping {
            expression: Box::new(Expr::Literal {
                value: LiteralValue::Number(45.67),
            }),
        }),
    };
    let ast_printer = AstPrinter;
    println!("{}", ast_printer.print(expression));
}

pub fn run_file(lox: &mut Lox, path: &str) -> Result<(), Box<dyn Error>> {
    let code = String::from_utf8(std::fs::read(path)?)?;
    run(&code, lox);
    if lox.had_error {
        exit(65);
    }
    Ok(())
}

pub fn run_prompt(lox: &mut Lox) {
    let mut scanner = io::stdin().lock();
    let mut buffer = String::new();
    loop {
        print!(">");
        io::stdout().flush().unwrap();
        if scanner.read_line(&mut buffer).is_ok() {
            run(&buffer, lox);
            lox.had_error = false;
        }
    }
}

fn run(source: &str, lox: &mut Lox) {
    let mut scanner = Scanner::new(source.to_string());
    scanner.scan_tokens(lox);

    for token in scanner.tokens {
        println!("{}", token);
    }
}
