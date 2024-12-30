use crate::{
    interpreter::Interpreter, lox_callable::LoxCallable, lox_exception::LoxException,
    lox_instance::LoxInstance, lox_object::LoxObject,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct LoxClass {
    pub name: String,
}
impl LoxClass {
    pub fn new(name: String) -> Self {
        LoxClass { name }
    }
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl LoxCallable for LoxClass {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _: &mut Interpreter, _: Vec<LoxObject>) -> Result<LoxObject, LoxException> {
        let instance = LoxInstance::new(self.clone());
        Ok(LoxObject::Instance(instance))
    }
}
