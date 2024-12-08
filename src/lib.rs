mod environment;
mod expr;
pub mod interpreter;
mod lox_callable;
mod lox_exception;
mod lox_object;
pub mod parser;
pub mod scanner;
mod stmt;
mod token;
mod token_type;

pub fn report(line: usize, loc: &str, message: &str) {
    eprintln!("[line {line} ] Error {loc}: {message}");
}
