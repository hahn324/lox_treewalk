use crate::{
    lox_exception::{LoxException, RuntimeError},
    lox_object::LoxObject,
    token::Token,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, PartialEq)]
pub struct Environment {
    values: HashMap<String, LoxObject>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
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

    pub fn get(&self, name: &Token) -> Result<LoxObject, LoxException> {
        match self.values.contains_key(&name.lexeme) {
            true => Ok(self.values.get(&name.lexeme).unwrap().clone()),
            false if self.enclosing.is_some() => {
                self.enclosing.as_deref().unwrap().borrow().get(name)
            }
            false => Err(LoxException::RuntimeError(RuntimeError::new(
                name.line,
                format!("Undefined variable '{}'.", &name.lexeme),
            ))),
        }
    }

    pub fn get_at(&self, distance: usize, name: &str) -> LoxObject {
        let expect_msg = format!(
            "Expect to find variable '{name}' at distance {distance} due to semantic analysis in Resolver."
        );
        if distance == 0 {
            self.values.get(name).expect(&expect_msg).clone()
        } else {
            self.ancestor(distance)
                .borrow()
                .values
                .get(name)
                .expect(&expect_msg)
                .clone()
        }
    }

    fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment>> {
        let expect_msg = "Expect number of enclosing environments to match value from Resolver.";
        let mut environment = Rc::clone(self.enclosing.as_ref().expect(expect_msg));
        for _ in 1..distance {
            let enclosing = Rc::clone(environment.borrow().enclosing.as_ref().expect(expect_msg));
            environment = enclosing;
        }
        environment
    }

    pub fn assign(&mut self, name: &Token, value: LoxObject) -> Result<LoxObject, LoxException> {
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
                return Err(LoxException::RuntimeError(RuntimeError::new(
                    name.line,
                    format!("Undefined variable '{}'.", &name.lexeme),
                )))
            }
        }
        Ok(value)
    }

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: LoxObject) -> LoxObject {
        if distance == 0 {
            self.values.insert(name.lexeme.clone(), value.clone());
        } else {
            self.ancestor(distance)
                .borrow_mut()
                .values
                .insert(name.lexeme.clone(), value.clone());
        }
        value
    }
}
