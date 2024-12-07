use std::error::Error;
use std::fmt;

#[derive(Debug)]
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
        write!(f, "[line {}] {}", self.line, self.message)
    }
}

impl Error for RuntimeError {}
