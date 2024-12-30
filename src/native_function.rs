use crate::{
    interpreter::Interpreter, lox_callable::LoxCallable, lox_exception::LoxException,
    lox_object::LoxObject,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct NativeFunction {
    function: fn(&mut Interpreter, Vec<LoxObject>) -> LoxObject,
    arity: usize,
    repr: String,
}
impl NativeFunction {
    pub fn new(
        function: fn(&mut Interpreter, Vec<LoxObject>) -> LoxObject,
        arity: usize,
        repr: String,
    ) -> Self {
        NativeFunction {
            function,
            arity,
            repr,
        }
    }
}

impl fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.repr)
    }
}

impl LoxCallable for NativeFunction {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LoxObject>,
    ) -> Result<LoxObject, LoxException> {
        Ok((self.function)(interpreter, arguments))
    }
}
