use crate::{
    interpreter::Interpreter,
    lox_callable::{Callable, LoxCallable},
    lox_exception::LoxException,
    lox_instance::LoxInstance,
    lox_object::LoxObject,
};
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct LoxClass {
    pub name: String,
    pub methods: HashMap<String, Rc<Callable>>,
}
impl LoxClass {
    pub fn new(name: String, methods: HashMap<String, Rc<Callable>>) -> Self {
        LoxClass { name, methods }
    }

    pub fn find_method(&self, name: &str) -> Option<Rc<Callable>> {
        match self.methods.get(name) {
            Some(method) => Some(Rc::clone(method)),
            None => None,
        }
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
        Ok(LoxObject::Instance(Rc::new(RefCell::new(instance))))
    }
}
