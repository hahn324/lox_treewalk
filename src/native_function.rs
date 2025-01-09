use crate::{interpreter::Interpreter, lox_exception::LoxException, lox_object::LoxObject};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct NativeFunction<'src> {
    function: fn(&mut Interpreter<'src>, Vec<LoxObject<'src>>) -> LoxObject<'src>,
    arity: usize,
    repr: String,
}
impl<'src> NativeFunction<'src> {
    pub fn new(
        function: fn(&mut Interpreter<'src>, Vec<LoxObject<'src>>) -> LoxObject<'src>,
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
        interpreter: &mut Interpreter<'src>,
        arguments: Vec<LoxObject<'src>>,
    ) -> Result<LoxObject<'src>, LoxException<'src>> {
        Ok((self.function)(interpreter, arguments))
    }
}

impl<'src> fmt::Display for NativeFunction<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.repr)
    }
}
