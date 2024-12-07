use crate::{lox_object::LoxObject, runtime_error::RuntimeError, token::Token};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Environment {
    values: HashMap<String, LoxObject>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(enclosing: Option<Rc<RefCell<Environment>>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: String, value: LoxObject) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<LoxObject, RuntimeError> {
        match self.values.contains_key(&name.lexeme) {
            true => Ok(self.values.get(&name.lexeme).unwrap().clone()),
            false if self.enclosing.is_some() => {
                self.enclosing.as_deref().unwrap().borrow().get(name)
            }
            false => Err(RuntimeError::new(
                name.line,
                format!("Undefined variable '{}'.", &name.lexeme),
            )),
        }
    }

    pub fn assign(&mut self, name: &Token, value: LoxObject) -> Result<LoxObject, RuntimeError> {
        match self.values.contains_key(&name.lexeme) {
            true => {
                self.values.insert(name.lexeme.clone(), value.clone());
            }
            false if self.enclosing.is_some() => {
                self.enclosing
                    .as_deref()
                    .unwrap()
                    .borrow_mut()
                    .assign(name, value.clone())?;
            }
            false => {
                return Err(RuntimeError::new(
                    name.line,
                    format!("Undefined variable '{}'.", &name.lexeme),
                ))
            }
        }
        Ok(value)
    }
}
