use crate::{
    environment::Environment,
    expr::Closure,
    interpreter::Interpreter,
    lox_exception::LoxException,
    lox_instance::LoxInstance,
    lox_object::{LoxLiteral, LoxObject},
};
use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct LoxFunction {
    declaration: Closure,
    context: Rc<RefCell<Environment>>,
    arity: usize,
    name: Option<String>,
    repr: String,
    is_initializer: bool,
}

impl LoxFunction {
    pub fn new(
        declaration: &Closure,
        context: Rc<RefCell<Environment>>,
        name: Option<String>,
        is_initializer: bool,
    ) -> Self {
        let arity = declaration.params.len();
        let repr = match name {
            Some(ref lexeme) => format!("<fn {}>", lexeme),
            None => String::from("<fn>"),
        };
        LoxFunction {
            declaration: declaration.clone(),
            context,
            arity,
            name,
            repr,
            is_initializer,
        }
    }

    pub fn arity(&self) -> usize {
        self.arity
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LoxObject>,
    ) -> Result<LoxObject, LoxException> {
        let environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &self.context,
        )))));
        for (idx, value) in arguments.into_iter().enumerate() {
            environment
                .borrow_mut()
                .define(self.declaration.params[idx].lexeme.clone(), value);
        }

        match interpreter.execute_block(&self.declaration.body, environment) {
            Ok(_) if self.is_initializer => Ok(self.context.borrow().get_at(0, "this")),
            Ok(_) => Ok(LoxObject::Literal(LoxLiteral::Nil)),
            Err(exception) => match exception {
                LoxException::RuntimeError(_) => Err(exception),
                LoxException::Return(_) if self.is_initializer => {
                    Ok(self.context.borrow().get_at(0, "this"))
                }
                LoxException::Return(value) => Ok(value),
            },
        }
    }

    pub fn bind(&self, instance: &Rc<RefCell<LoxInstance>>) -> LoxFunction {
        let mut environment = Environment::new(Some(Rc::clone(&self.context)));
        environment.define(
            String::from("this"),
            LoxObject::Instance(Rc::clone(instance)),
        );
        LoxFunction::new(
            &self.declaration,
            Rc::new(RefCell::new(environment)),
            self.name.clone(),
            self.is_initializer,
        )
    }
}

impl fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.repr)
    }
}
