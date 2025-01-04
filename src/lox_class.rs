use crate::{
    interpreter::Interpreter, lox_exception::LoxException, lox_function::LoxFunction,
    lox_instance::LoxInstance, lox_object::LoxObject,
};
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct LoxClass {
    pub name: String,
    pub superclass: Option<Rc<LoxClass>>,
    pub methods: HashMap<String, LoxFunction>,
}
impl LoxClass {
    pub fn new(
        name: String,
        superclass: Option<Rc<LoxClass>>,
        methods: HashMap<String, LoxFunction>,
    ) -> Self {
        LoxClass {
            name,
            superclass,
            methods,
        }
    }

    pub fn arity(&self) -> usize {
        match self.find_method("init") {
            Some(initializer) => initializer.arity(),
            None => 0,
        }
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LoxObject>,
    ) -> Result<LoxObject, LoxException> {
        let instance = Rc::new(RefCell::new(LoxInstance::new(self.clone())));
        if let Some(initializer) = self.find_method("init") {
            initializer.bind(&instance).call(interpreter, arguments)?;
        }
        Ok(LoxObject::Instance(instance))
    }

    pub fn find_method(&self, name: &str) -> Option<&LoxFunction> {
        if self.methods.contains_key(name) {
            return self.methods.get(name);
        }

        match self.superclass {
            Some(ref class) => class.find_method(name),
            None => None,
        }
    }
}

impl fmt::Display for LoxClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
