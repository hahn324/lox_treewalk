use crate::{lox_callable::LoxCallable, lox_instance::LoxInstance};
use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub enum LoxLiteral {
    Number(f64),
    String(Rc<String>),
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
pub enum LoxObject<'src> {
    Literal(LoxLiteral),
    Callable(LoxCallable<'src>),
    Instance(Rc<RefCell<LoxInstance<'src>>>),
}

impl fmt::Display for LoxObject<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxObject::Literal(literal) => write!(f, "{literal}"),
            LoxObject::Callable(function) => write!(f, "{function}"),
            LoxObject::Instance(instance) => write!(f, "{}", instance.borrow()),
        }
    }
}
