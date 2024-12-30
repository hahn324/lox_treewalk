use crate::{lox_callable::Callable, lox_instance::LoxInstance};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum LoxLiteral {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl fmt::Display for LoxLiteral {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxLiteral::Number(val) => write!(f, "{val}"),
            LoxLiteral::String(ref val) => write!(f, "{val}"),
            LoxLiteral::Boolean(val) => write!(f, "{val}"),
            LoxLiteral::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoxObject {
    Literal(LoxLiteral),
    Callable(Callable),
    Instance(LoxInstance),
}

impl fmt::Display for LoxObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxObject::Literal(literal) => write!(f, "{literal}"),
            LoxObject::Callable(function) => write!(f, "{function}"),
            LoxObject::Instance(instance) => write!(f, "{instance}"),
        }
    }
}
