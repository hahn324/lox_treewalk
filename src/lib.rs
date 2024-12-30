mod environment;
mod expr;
pub mod interpreter;
mod lox_callable;
mod lox_class;
mod lox_exception;
mod lox_function;
mod lox_instance;
mod lox_object;
mod native_function;
pub mod parser;
pub mod resolver;
pub mod scanner;
mod stmt;
mod token;
mod token_type;

pub fn report(line: usize, loc: &str, message: &str) {
    eprintln!("[line {line}] Error {loc}: {message}");
}
