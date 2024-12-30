use crate::lox_object::LoxObject;
use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub enum LoxException {
    RuntimeError(RuntimeError),
    Return(LoxObject),
}

impl fmt::Display for LoxException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxException::RuntimeError(error) => write!(f, "{error}"),
            LoxException::Return(value) => write!(f, "{value}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub line: usize,
    pub message: String,
}

impl RuntimeError {
    pub fn new(line: usize, message: String) -> Self {
        RuntimeError { line, message }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[line {}] RuntimeError: {}", self.line, self.message)
    }
}

impl Error for RuntimeError {}
