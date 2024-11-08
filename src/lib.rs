mod ast_printer;
mod expr;
mod expr_interpreter;
mod scanner;
mod token;
mod token_type;

use crate::scanner::Scanner;
use crate::token::Token;
use std::fs;
use std::io::{self, Write};

pub fn run_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_path)?;
    let had_error = !run(&contents);
    if had_error {
        std::process::exit(65);
    }
    Ok(())
}

pub fn run_prompt() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    loop {
        buffer.clear();
        print!("> ");
        io::stdout().flush()?;
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                if n == 1 {
                    break;
                }
                run(&buffer.trim());
            }
            Err(error) => {
                eprintln!("Error: {error}");
                break;
            }
        }
    }
    Ok(())
}

fn run(source: &str) -> bool {
    let scanner = Scanner::new(source);
    let tokens: Vec<Token> = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }
    true
}

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, loc: &str, message: &str) {
    eprintln!("[line {line} ] Error {loc}: {message}");
    // TODO: Handle setting had_error
}
