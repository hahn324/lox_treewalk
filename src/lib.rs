mod ast_printer;
mod expr;
mod parser;
mod scanner;
mod token;
mod token_type;

use crate::ast_printer::AstPrinter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use std::fs;
use std::io::{self, Write};

pub fn run_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_path)?;
    if !run(&contents) {
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
    let mut had_error = false;
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens();
    let tokens = match scanner.take() {
        Ok(tokens) => tokens,
        Err(tokens) => {
            had_error = true;
            tokens
        }
    };
    let mut parser = Parser::new(tokens);
    let parse_result = parser.parse();

    match parse_result {
        Ok(expression) => {
            let mut ast_printer = AstPrinter::new();
            expression.accept(&mut ast_printer);
            println!("{}", ast_printer.output);
        }
        Err(_) => {
            had_error = true;
        }
    }

    !had_error
}

pub fn report(line: usize, loc: &str, message: &str) {
    eprintln!("[line {line} ] Error {loc}: {message}");
}
