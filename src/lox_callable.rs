use crate::{
    interpreter::Interpreter, lox_class::LoxClass, lox_exception::LoxException,
    lox_function::LoxFunction, lox_object::LoxObject, native_function::NativeFunction,
};
use std::fmt;

pub trait LoxCallable: fmt::Display + PartialEq {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LoxObject>,
    ) -> Result<LoxObject, LoxException>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Callable {
    Function(LoxFunction),
    NativeFun(NativeFunction),
    Class(LoxClass),
}

impl fmt::Display for Callable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Callable::Function(function) => write!(f, "{function}"),
            Callable::NativeFun(native_fun) => write!(f, "{native_fun}"),
            Callable::Class(class) => write!(f, "{class}"),
        }
    }
}

impl LoxCallable for Callable {
    fn arity(&self) -> usize {
        match self {
            Callable::Function(function) => function.arity(),
            Callable::NativeFun(native_fun) => native_fun.arity(),
            Callable::Class(class) => class.arity(),
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LoxObject>,
    ) -> Result<LoxObject, LoxException> {
        match self {
            Callable::Function(function) => function.call(interpreter, arguments),
            Callable::NativeFun(native_fun) => native_fun.call(interpreter, arguments),
            Callable::Class(class) => class.call(interpreter, arguments),
        }
    }
}
