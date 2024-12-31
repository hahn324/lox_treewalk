use crate::{interpreter::Interpreter, lox_exception::LoxException, lox_object::LoxObject};
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

    pub fn arity(&self) -> usize {
        self.arity
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LoxObject>,
    ) -> Result<LoxObject, LoxException> {
        Ok((self.function)(interpreter, arguments))
    }
}

impl fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.repr)
    }
}
