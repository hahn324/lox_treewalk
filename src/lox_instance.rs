use crate::{
    lox_callable::LoxCallable,
    lox_class::LoxClass,
    lox_exception::{LoxException, RuntimeError},
    lox_object::LoxObject,
    token::Token,
};
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct LoxInstance {
    klass: LoxClass,
    fields: HashMap<String, LoxObject>,
}
impl LoxInstance {
    pub fn new(klass: LoxClass) -> Self {
        LoxInstance {
            klass,
            fields: HashMap::new(),
        }
    }

    pub fn get(
        &self,
        name: &Token,
        instance: &Rc<RefCell<LoxInstance>>,
    ) -> Result<LoxObject, LoxException> {
        if self.fields.contains_key(&name.lexeme) {
            return Ok(self.fields.get(&name.lexeme).unwrap().clone());
        }

        match self.klass.find_method(&name.lexeme) {
            Some(method) => Ok(LoxObject::Callable(Rc::new(LoxCallable::Function(
                method.bind(instance),
            )))),
            None => Err(LoxException::RuntimeError(RuntimeError::new(
                name.line,
                format!("Undefined property '{}'.", name.lexeme),
            ))),
        }
    }

    pub fn set(&mut self, name: &Token, value: LoxObject) -> LoxObject {
        self.fields.insert(name.lexeme.clone(), value.clone());
        value
    }
}

impl fmt::Display for LoxInstance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} instance", self.klass.name)
    }
}
