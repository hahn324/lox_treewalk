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
pub struct LoxFunction<'src> {
    declaration: Closure<'src>,
    context: Rc<RefCell<Environment<'src>>>,
    arity: usize,
    name: Option<&'src str>,
    repr: String,
    is_initializer: bool,
}

impl<'src> LoxFunction<'src> {
    pub fn new(
        declaration: &Closure<'src>,
        context: Rc<RefCell<Environment<'src>>>,
        name: Option<&'src str>,
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
        interpreter: &mut Interpreter<'src>,
        arguments: Vec<LoxObject<'src>>,
    ) -> Result<LoxObject<'src>, LoxException<'src>> {
        let environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &self.context,
        )))));
        for (idx, value) in arguments.into_iter().enumerate() {
            environment
                .borrow_mut()
                .define(self.declaration.params[idx].lexeme, value);
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

    pub fn bind(&self, instance: Rc<RefCell<LoxInstance<'src>>>) -> LoxFunction<'src> {
        let mut environment = Environment::new(Some(Rc::clone(&self.context)));
        environment.define("this", LoxObject::Instance(instance));
        LoxFunction::new(
            &self.declaration,
            Rc::new(RefCell::new(environment)),
            self.name,
            self.is_initializer,
        )
    }
}

impl<'src> fmt::Display for LoxFunction<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.repr)
    }
}
