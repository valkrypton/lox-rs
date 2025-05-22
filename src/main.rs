mod error;
mod expression;
mod parser;
mod scanner;
mod token;
mod token_type;

use std::{
    cmp::Ordering,
    env,
    error::Error,
    io::{self, BufRead, Write},
    process::exit,
};

use crate::{expression::AstPrinter, parser::Parser, scanner::Scanner};

pub struct Lox {
    pub had_error: bool,
}

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let mut lox = Lox { had_error: false };
    match args.len().cmp(&2) {
        Ordering::Greater => {
            println!("Usage: rlox [script]");
            exit(64);
        }
        Ordering::Equal => run_file(&mut lox, &args[1]).expect("error while reading file"),
        _ => run_prompt(&mut lox),
    }
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
    let mut parser = Parser::new(scanner.tokens);
    while !parser.complete(){
        if let Ok(expr) = parser.parse() {
            let ast_p = AstPrinter;
            ast_p.print(expr);
        }
    }
}
