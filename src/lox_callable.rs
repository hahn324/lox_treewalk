use crate::{
    environment::Environment,
    interpreter::Interpreter,
    lox_exception::LoxException,
    lox_object::{LoxLiteral, LoxObject},
    stmt::Function,
};
use std::{cell::RefCell, fmt, rc::Rc};

#[derive(Debug, Clone, PartialEq)]
pub enum FunDeclaration {
    NativeFunction(fn(&mut Interpreter, Vec<LoxObject>) -> LoxObject),
    LoxFunction {
        declaration: Function,
        closure: Rc<RefCell<Environment>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoxCallable {
    arity: usize,
    function: FunDeclaration,
    repr: String,
}

impl LoxCallable {
    pub fn new_native_fun(
        arity: usize,
        function: fn(&mut Interpreter, Vec<LoxObject>) -> LoxObject,
        repr: String,
    ) -> Self {
        LoxCallable {
            arity,
            function: FunDeclaration::NativeFunction(function),
            repr,
        }
    }

    pub fn new_lox_fun(declaration: &Function, closure: Rc<RefCell<Environment>>) -> Self {
        let arity = declaration.params.len();
        let repr = format!("<fn {}>", declaration.name.lexeme);
        LoxCallable {
            arity,
            function: FunDeclaration::LoxFunction {
                declaration: declaration.clone(),
                closure,
            },
            repr,
        }
    }

    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<LoxObject>,
    ) -> Result<LoxObject, LoxException> {
        match &self.function {
            FunDeclaration::NativeFunction(function) => Ok(function(interpreter, arguments)),
            FunDeclaration::LoxFunction {
                declaration,
                closure,
            } => {
                let environment =
                    Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&closure)))));
                for (idx, value) in arguments.into_iter().enumerate() {
                    environment
                        .borrow_mut()
                        .define(declaration.params[idx].lexeme.clone(), value);
                }

                match interpreter.execute_block(&declaration.body, environment) {
                    Ok(_) => Ok(LoxObject::Literal(LoxLiteral::Nil)),
                    Err(exception) => match exception {
                        LoxException::RuntimeError(_) => Err(exception),
                        LoxException::Return(value) => Ok(value),
                    },
                }
            }
        }
    }

    pub fn arity(&self) -> usize {
        self.arity
    }
}

impl fmt::Display for LoxCallable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.repr)
    }
}
