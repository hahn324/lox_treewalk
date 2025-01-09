use crate::{
    interpreter::Interpreter, lox_exception::LoxException, lox_function::LoxFunction,
    lox_instance::LoxInstance, lox_object::LoxObject,
};
use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct LoxClass<'src> {
    pub name: &'src str,
    pub superclass: Option<Rc<LoxClass<'src>>>,
    pub methods: HashMap<&'src str, LoxFunction<'src>>,
}
impl<'src> LoxClass<'src> {
    pub fn new(
        name: &'src str,
        superclass: Option<Rc<LoxClass<'src>>>,
        methods: HashMap<&'src str, LoxFunction<'src>>,
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
        interpreter: &mut Interpreter<'src>,
        arguments: Vec<LoxObject<'src>>,
    ) -> Result<LoxObject<'src>, LoxException<'src>> {
        let instance = Rc::new(RefCell::new(LoxInstance::new(self.clone())));
        if let Some(initializer) = self.find_method("init") {
            initializer
                .bind(Rc::clone(&instance))
                .call(interpreter, arguments)?;
        }
        Ok(LoxObject::Instance(instance))
    }

    pub fn find_method(&self, name: &str) -> Option<&LoxFunction<'src>> {
        if self.methods.contains_key(name) {
            return self.methods.get(name);
        }

        match self.superclass {
            Some(ref class) => class.find_method(name),
            None => None,
        }
    }
}

impl<'src> fmt::Display for LoxClass<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
