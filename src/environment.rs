use crate::{
    lox_exception::{LoxException, RuntimeError},
    lox_object::LoxObject,
    token::Token,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, PartialEq)]
pub struct Environment<'src> {
    values: HashMap<&'src str, LoxObject<'src>>,
    pub enclosing: Option<Rc<RefCell<Environment<'src>>>>,
}

impl<'src> Environment<'src> {
    pub fn new(enclosing: Option<Rc<RefCell<Environment<'src>>>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing,
        }
    }

    pub fn define(&mut self, name: &'src str, value: LoxObject<'src>) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<LoxObject<'src>, LoxException<'src>> {
        match self.values.contains_key(name.lexeme) {
            true => Ok(self.values.get(name.lexeme).unwrap().clone()),
            false if self.enclosing.is_some() => {
                self.enclosing.as_deref().unwrap().borrow().get(name)
            }
            false => Err(LoxException::RuntimeError(RuntimeError::new(
                name.line,
                format!("Undefined variable '{}'.", name.lexeme),
            ))),
        }
    }

    pub fn get_at(&self, distance: usize, name: &str) -> LoxObject<'src> {
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

    fn ancestor(&self, distance: usize) -> Rc<RefCell<Environment<'src>>> {
        let expect_msg = "Expect number of enclosing environments to match value from Resolver.";
        let mut environment = Rc::clone(self.enclosing.as_ref().expect(expect_msg));
        for _ in 1..distance {
            let enclosing = Rc::clone(environment.borrow().enclosing.as_ref().expect(expect_msg));
            environment = enclosing;
        }
        environment
    }

    pub fn assign(
        &mut self,
        name: &Token<'src>,
        value: LoxObject<'src>,
    ) -> Result<LoxObject<'src>, LoxException<'src>> {
        match self.values.contains_key(&name.lexeme) {
            true => {
                self.values.insert(name.lexeme, value.clone());
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

    pub fn assign_at(
        &mut self,
        distance: usize,
        name: &Token<'src>,
        value: LoxObject<'src>,
    ) -> LoxObject<'src> {
        if distance == 0 {
            self.values.insert(name.lexeme, value.clone());
        } else {
            self.ancestor(distance)
                .borrow_mut()
                .values
                .insert(name.lexeme, value.clone());
        }
        value
    }
}
