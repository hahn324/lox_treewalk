mod ast_printer;
mod expr;
mod interpreter;
mod parser;
mod scanner;
mod token;
mod token_type;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use std::fs;
use std::io::{self, Write};

pub fn run_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_path)?;
    let mut interpreter = Interpreter::new();
    let exit_code = run(&contents, &mut interpreter);
    if exit_code != 0 {
        std::process::exit(exit_code);
    }
    Ok(())
}

pub fn run_prompt() -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = String::new();
    let mut interpreter = Interpreter::new();
    loop {
        buffer.clear();
        print!("> ");
        io::stdout().flush()?;
        match io::stdin().read_line(&mut buffer) {
            Ok(n) => {
                if n == 1 {
                    break;
                }
                run(&buffer.trim(), &mut interpreter);
            }
            Err(error) => {
                eprintln!("Error: {error}");
                break;
            }
        }
    }
    Ok(())
}

fn run(source: &str, interpreter: &mut Interpreter) -> i32 {
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
        Ok(expression) => interpreter.interpret(&expression),
        Err(_) => {
            had_error = true;
        }
    }

    if had_error {
        65
    } else if interpreter.had_runtime_error {
        interpreter.had_runtime_error = false;
        70
    } else {
        0
    }
}

pub fn report(line: usize, loc: &str, message: &str) {
    eprintln!("[line {line} ] Error {loc}: {message}");
}
