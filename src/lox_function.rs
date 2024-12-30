use crate::{
    environment::Environment,
    expr::Closure,
    interpreter::Interpreter,
    lox_callable::LoxCallable,
    lox_exception::LoxException,
    lox_object::{LoxLiteral, LoxObject},
};
use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub struct LoxFunction {
    declaration: Closure,
    context: Rc<RefCell<Environment>>,
    arity: usize,
    repr: String,
}

impl LoxFunction {
    pub fn new(
        declaration: &Closure,
        context: Rc<RefCell<Environment>>,
        name: Option<String>,
    ) -> Self {
        let arity = declaration.params.len();
        let repr = match name {
            Some(lexeme) => format!("<fn {}>", lexeme),
            None => String::from("<fn>"),
        };
        LoxFunction {
            declaration: declaration.clone(),
            context,
            arity,
            repr,
        }
    }
}

impl fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.repr)
    }
}

impl LoxCallable for LoxFunction {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(
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
            Ok(_) => Ok(LoxObject::Literal(LoxLiteral::Nil)),
            Err(exception) => match exception {
                LoxException::RuntimeError(_) => Err(exception),
                LoxException::Return(value) => Ok(value),
            },
        }
    }
}
