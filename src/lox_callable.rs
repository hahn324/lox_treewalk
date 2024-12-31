use crate::{
    interpreter::Interpreter, lox_class::LoxClass, lox_exception::LoxException,
    lox_function::LoxFunction, lox_object::LoxObject, native_function::NativeFunction,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum LoxCallable {
    Function(LoxFunction),
    NativeFun(NativeFunction),
    Class(LoxClass),
}

impl LoxCallable {
    pub fn arity(&self) -> usize {
        match self {
            LoxCallable::Function(function) => function.arity(),
            LoxCallable::NativeFun(native_fun) => native_fun.arity(),
            LoxCallable::Class(class) => class.arity(),
        }
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LoxObject>,
    ) -> Result<LoxObject, LoxException> {
        match self {
            LoxCallable::Function(function) => function.call(interpreter, arguments),
            LoxCallable::NativeFun(native_fun) => native_fun.call(interpreter, arguments),
            LoxCallable::Class(class) => class.call(interpreter, arguments),
        }
    }
}

impl fmt::Display for LoxCallable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxCallable::Function(function) => write!(f, "{function}"),
            LoxCallable::NativeFun(native_fun) => write!(f, "{native_fun}"),
            LoxCallable::Class(class) => write!(f, "{class}"),
        }
    }
}
