mod scanner;
mod token;
mod token_type;
use crate::scanner::Scanner;
use std::cmp::Ordering;
use std::env;
use std::error::Error;
use std::io;
use std::io::{BufRead, Write};
use std::process::exit;

pub struct Lox {
    pub had_error: bool,
}

impl Lox {
    fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error {}: {}", line, location, message);
        self.had_error = true;
    }
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

    for token in scanner.tokens {
        println!("{}", token);
    }
}
