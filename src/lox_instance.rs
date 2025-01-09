use crate::{
    lox_callable::LoxCallable,
    lox_class::LoxClass,
    lox_exception::{LoxException, RuntimeError},
    lox_object::LoxObject,
    token::Token,
};
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct LoxInstance<'src> {
    klass: LoxClass<'src>,
    fields: HashMap<&'src str, LoxObject<'src>>,
}
impl<'src> LoxInstance<'src> {
    pub fn new(klass: LoxClass<'src>) -> Self {
        LoxInstance {
            klass,
            fields: HashMap::new(),
        }
    }

    pub fn get(
        &self,
        name: &Token<'src>,
        instance: Rc<RefCell<LoxInstance<'src>>>,
    ) -> Result<LoxObject<'src>, LoxException<'src>> {
        if self.fields.contains_key(name.lexeme) {
            return Ok(self.fields.get(name.lexeme).unwrap().clone());
        }

        match self.klass.find_method(name.lexeme) {
            Some(method) => Ok(LoxObject::Callable(LoxCallable::Function(Rc::new(
                method.bind(instance),
            )))),
            None => Err(LoxException::RuntimeError(RuntimeError::new(
                name.line,
                format!("Undefined property '{}'.", name.lexeme),
            ))),
        }
    }

    pub fn set(&mut self, name: &Token<'src>, value: LoxObject<'src>) -> LoxObject<'src> {
        self.fields.insert(name.lexeme, value.clone());
        value
    }
}

impl<'src> fmt::Display for LoxInstance<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} instance", self.klass.name)
    }
}
