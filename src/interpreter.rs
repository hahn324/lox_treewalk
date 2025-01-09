use crate::{
    environment::Environment,
    expr::{
        Assign, Binary, Call, Closure, Expr, ExprVisitor, Get, Grouping, Literal, Logical, Set,
        Super, Ternary, This, Unary, Variable,
    },
    lox_callable::LoxCallable,
    lox_class::LoxClass,
    lox_exception::{LoxException, RuntimeError},
    lox_function::LoxFunction,
    lox_object::{LoxLiteral, LoxObject},
    native_function::NativeFunction,
    stmt::{Block, Class, Expression, Function, If, Print, Return, Stmt, StmtVisitor, Var, While},
    token::Token,
    token_type::TokenType,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc, time::SystemTime};

pub struct Interpreter<'src> {
    pub globals: Rc<RefCell<Environment<'src>>>,
    pub environment: Rc<RefCell<Environment<'src>>>,
    locals: HashMap<Token<'src>, usize>,
    active_break: bool,
}

impl<'src> Interpreter<'src> {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new(None)));
        // Implement global "clock" function.
        let clock_function = |_: &mut Interpreter, _: Vec<LoxObject<'src>>| {
            LoxObject::Literal(LoxLiteral::Number(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("SystemTime should be after UNIX EPOCH in global clock function.")
                    .as_secs_f64(),
            ))
        };
        let global_clock = LoxObject::Callable(LoxCallable::NativeFun(Rc::new(
            NativeFunction::new(clock_function, 0, String::from("<native fn>")),
        )));

        globals.borrow_mut().define("clock", global_clock);

        let environment = Rc::clone(&globals);

        Interpreter {
            globals,
            environment,
            locals: HashMap::new(),
            active_break: false,
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt<'src>>) -> Result<(), LoxException<'src>> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt<'src>) -> Result<(), LoxException<'src>> {
        stmt.accept(self)
    }

    pub fn resolve(&mut self, token: Token<'src>, depth: usize) {
        self.locals.insert(token, depth);
    }

    pub fn execute_block(
        &mut self,
        statements: &Vec<Stmt<'src>>,
        environment: Rc<RefCell<Environment<'src>>>,
    ) -> Result<(), LoxException<'src>> {
        let previous_env = Rc::clone(&self.environment);
        self.environment = environment;

        for statement in statements {
            if self.active_break {
                break;
            }
            match self.execute(statement) {
                Ok(_) => (),
                Err(exception) => {
                    self.environment = previous_env;
                    return Err(exception);
                }
            }
        }

        self.environment = previous_env;
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr<'src>) -> Result<LoxObject<'src>, LoxException<'src>> {
        expr.accept(self)
    }

    fn is_truthy(&self, object: &LoxObject<'src>) -> bool {
        match &object {
            LoxObject::Literal(LoxLiteral::Nil) => false,
            LoxObject::Literal(LoxLiteral::Boolean(res)) => *res,
            _ => true,
        }
    }

    fn look_up_variable(
        &mut self,
        name: &Token<'src>,
    ) -> Result<LoxObject<'src>, LoxException<'src>> {
        match self.locals.get(name) {
            Some(&distance) => Ok(self.environment.borrow().get_at(distance, name.lexeme)),
            None => self.globals.borrow().get(name),
        }
    }
}

impl<'src> ExprVisitor<'src, Result<LoxObject<'src>, LoxException<'src>>> for Interpreter<'src> {
    fn visit_binary_expr(
        &mut self,
        expr: &Binary<'src>,
    ) -> Result<LoxObject<'src>, LoxException<'src>> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Minus => match (left, right) {
                (
                    LoxObject::Literal(LoxLiteral::Number(left_val)),
                    LoxObject::Literal(LoxLiteral::Number(right_val)),
                ) => Ok(LoxObject::Literal(LoxLiteral::Number(left_val - right_val))),
                _ => Err(LoxException::RuntimeError(RuntimeError::new(
                    expr.operator.line,
                    String::from("Operands must be numbers."),
                ))),
            },
            TokenType::Slash => match (left, right) {
                (
                    LoxObject::Literal(LoxLiteral::Number(left_val)),
                    LoxObject::Literal(LoxLiteral::Number(right_val)),
                ) => match right_val == 0.0 {
                    true => Err(LoxException::RuntimeError(RuntimeError::new(
                        expr.operator.line,
                        String::from("Cannot divide by zero."),
                    ))),
                    false => Ok(LoxObject::Literal(LoxLiteral::Number(left_val / right_val))),
                },
                _ => Err(LoxException::RuntimeError(RuntimeError::new(
                    expr.operator.line,
                    String::from("Operands must be numbers."),
                ))),
            },
            TokenType::Star => match (left, right) {
                (
                    LoxObject::Literal(LoxLiteral::Number(left_val)),
                    LoxObject::Literal(LoxLiteral::Number(right_val)),
                ) => Ok(LoxObject::Literal(LoxLiteral::Number(left_val * right_val))),
                _ => Err(LoxException::RuntimeError(RuntimeError::new(
                    expr.operator.line,
                    String::from("Operands must be numbers."),
                ))),
            },
            TokenType::Plus => match (left, right) {
                (
                    LoxObject::Literal(LoxLiteral::Number(left_val)),
                    LoxObject::Literal(LoxLiteral::Number(right_val)),
                ) => Ok(LoxObject::Literal(LoxLiteral::Number(left_val + right_val))),
                (
                    LoxObject::Literal(LoxLiteral::String(left_val)),
                    LoxObject::Literal(LoxLiteral::String(right_val)),
                ) => Ok(LoxObject::Literal(LoxLiteral::String(Rc::new(format!(
                    "{left_val}{right_val}"
                ))))),
                (LoxObject::Literal(LoxLiteral::String(left_val)), right) => Ok(
                    LoxObject::Literal(LoxLiteral::String(Rc::new(format!("{left_val}{right}",)))),
                ),
                (left, LoxObject::Literal(LoxLiteral::String(right_val))) => Ok(
                    LoxObject::Literal(LoxLiteral::String(Rc::new(format!("{left}{right_val}",)))),
                ),
                _ => Err(LoxException::RuntimeError(RuntimeError::new(
                    expr.operator.line,
                    String::from("Operands must be two numbers or one must be a string."),
                ))),
            },
            TokenType::Greater => match (left, right) {
                (
                    LoxObject::Literal(LoxLiteral::Number(left_val)),
                    LoxObject::Literal(LoxLiteral::Number(right_val)),
                ) => Ok(LoxObject::Literal(LoxLiteral::Boolean(
                    left_val > right_val,
                ))),
                _ => Err(LoxException::RuntimeError(RuntimeError::new(
                    expr.operator.line,
                    String::from("Operands must be numbers."),
                ))),
            },
            TokenType::GreaterEqual => match (left, right) {
                (
                    LoxObject::Literal(LoxLiteral::Number(left_val)),
                    LoxObject::Literal(LoxLiteral::Number(right_val)),
                ) => Ok(LoxObject::Literal(LoxLiteral::Boolean(
                    left_val >= right_val,
                ))),
                _ => Err(LoxException::RuntimeError(RuntimeError::new(
                    expr.operator.line,
                    String::from("Operands must be numbers."),
                ))),
            },
            TokenType::Less => match (left, right) {
                (
                    LoxObject::Literal(LoxLiteral::Number(left_val)),
                    LoxObject::Literal(LoxLiteral::Number(right_val)),
                ) => Ok(LoxObject::Literal(LoxLiteral::Boolean(
                    left_val < right_val,
                ))),
                _ => Err(LoxException::RuntimeError(RuntimeError::new(
                    expr.operator.line,
                    String::from("Operands must be numbers."),
                ))),
            },
            TokenType::LessEqual => match (left, right) {
                (
                    LoxObject::Literal(LoxLiteral::Number(left_val)),
                    LoxObject::Literal(LoxLiteral::Number(right_val)),
                ) => Ok(LoxObject::Literal(LoxLiteral::Boolean(
                    left_val <= right_val,
                ))),
                _ => Err(LoxException::RuntimeError(RuntimeError::new(
                    expr.operator.line,
                    String::from("Operands must be numbers."),
                ))),
            },
            TokenType::BangEqual => Ok(LoxObject::Literal(LoxLiteral::Boolean(left != right))),
            TokenType::EqualEqual => Ok(LoxObject::Literal(LoxLiteral::Boolean(left == right))),
            TokenType::Comma => Ok(right),
            _ => unreachable!("All valid Binary operators are accounted for in above arms."),
        }
    }

    fn visit_grouping_expr(
        &mut self,
        expr: &Grouping<'src>,
    ) -> Result<LoxObject<'src>, LoxException<'src>> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(
        &mut self,
        expr: &Literal,
    ) -> Result<LoxObject<'src>, LoxException<'src>> {
        Ok(LoxObject::Literal(expr.value.clone()))
    }

    fn visit_unary_expr(
        &mut self,
        expr: &Unary<'src>,
    ) -> Result<LoxObject<'src>, LoxException<'src>> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Minus => match right {
                LoxObject::Literal(LoxLiteral::Number(val)) => {
                    Ok(LoxObject::Literal(LoxLiteral::Number(-val)))
                }
                _ => Err(LoxException::RuntimeError(RuntimeError::new(
                    expr.operator.line,
                    String::from("Operand must be a number."),
                ))),
            },
            TokenType::Bang => Ok(LoxObject::Literal(LoxLiteral::Boolean(
                !self.is_truthy(&right),
            ))),
            _ => unreachable!("All valid Unary operators are accounted for in above arms."),
        }
    }

    fn visit_ternary_expr(
        &mut self,
        expr: &Ternary<'src>,
    ) -> Result<LoxObject<'src>, LoxException<'src>> {
        let condition = self.evaluate(&expr.condition)?;
        match self.is_truthy(&condition) {
            true => self.evaluate(&expr.left),
            false => self.evaluate(&expr.right),
        }
    }

    fn visit_variable_expr(
        &mut self,
        expr: &Variable<'src>,
    ) -> Result<LoxObject<'src>, LoxException<'src>> {
        self.look_up_variable(&expr.name)
    }

    fn visit_assign_expr(
        &mut self,
        expr: &Assign<'src>,
    ) -> Result<LoxObject<'src>, LoxException<'src>> {
        let value = self.evaluate(&expr.value)?;
        match self.locals.get(&expr.name) {
            Some(&distance) => Ok(self
                .environment
                .borrow_mut()
                .assign_at(distance, &expr.name, value)),
            None => self.globals.borrow_mut().assign(&expr.name, value),
        }
    }

    fn visit_logical_expr(
        &mut self,
        expr: &Logical<'src>,
    ) -> Result<LoxObject<'src>, LoxException<'src>> {
        let left = self.evaluate(&expr.left)?;

        match expr.operator.token_type {
            TokenType::Or if self.is_truthy(&left) => Ok(left),
            TokenType::And if !self.is_truthy(&left) => Ok(left),
            _ => self.evaluate(&expr.right),
        }
    }

    fn visit_call_expr(
        &mut self,
        expr: &Call<'src>,
    ) -> Result<LoxObject<'src>, LoxException<'src>> {
        let callee = self.evaluate(&expr.callee)?;

        let mut arguments = Vec::new();
        for argument in expr.arguments.iter() {
            arguments.push(self.evaluate(argument)?);
        }

        match callee {
            LoxObject::Callable(callable) => {
                if arguments.len() != callable.arity() {
                    return Err(LoxException::RuntimeError(RuntimeError::new(
                        expr.paren.line,
                        format!(
                            "Expected {} arguments but got {}.",
                            callable.arity(),
                            arguments.len()
                        ),
                    )));
                }
                callable.call(self, arguments)
            }
            _ => Err(LoxException::RuntimeError(RuntimeError::new(
                expr.paren.line,
                String::from("Can only call functions and classes."),
            ))),
        }
    }

    fn visit_get_expr(&mut self, expr: &Get<'src>) -> Result<LoxObject<'src>, LoxException<'src>> {
        let object = self.evaluate(&expr.object)?;
        match object {
            LoxObject::Instance(instance) => {
                instance.borrow().get(&expr.name, Rc::clone(&instance))
            }
            _ => Err(LoxException::RuntimeError(RuntimeError::new(
                expr.name.line,
                String::from("Only instances have properties."),
            ))),
        }
    }

    fn visit_set_expr(&mut self, expr: &Set<'src>) -> Result<LoxObject<'src>, LoxException<'src>> {
        let object = self.evaluate(&expr.object)?;
        match object {
            LoxObject::Instance(instance) => {
                let value = self.evaluate(&expr.value)?;
                Ok(instance.borrow_mut().set(&expr.name, value.clone()))
            }
            _ => Err(LoxException::RuntimeError(RuntimeError::new(
                expr.name.line,
                String::from("Only instances have fields."),
            ))),
        }
    }

    fn visit_this_expr(
        &mut self,
        expr: &This<'src>,
    ) -> Result<LoxObject<'src>, LoxException<'src>> {
        self.look_up_variable(&expr.keyword)
    }

    fn visit_super_expr(&mut self, expr: &Super) -> Result<LoxObject<'src>, LoxException<'src>> {
        let distance = self
            .locals
            .get(&expr.keyword)
            .expect("Expected super local to resolve.");
        let superclass = self.environment.borrow().get_at(*distance, "super");

        let object = self.environment.borrow().get_at(*distance - 1, "this");
        let instance = match object {
            LoxObject::Instance(instance) => instance,
            _ => unreachable!(),
        };

        let method = match superclass {
            LoxObject::Callable(LoxCallable::Class(ref class)) => {
                class.find_method(expr.method.lexeme)
            }
            _ => unreachable!(),
        };
        match method {
            Some(function) => Ok(LoxObject::Callable(LoxCallable::Function(Rc::new(
                function.bind(instance),
            )))),
            None => Err(LoxException::RuntimeError(RuntimeError::new(
                expr.method.line,
                format!("Undefined property '{}'.", expr.method.lexeme),
            ))),
        }
    }

    fn visit_closure_expr(
        &mut self,
        expr: &Closure<'src>,
    ) -> Result<LoxObject<'src>, LoxException<'src>> {
        let closure = LoxFunction::new(expr, Rc::clone(&self.environment), None, false);
        Ok(LoxObject::Callable(LoxCallable::Function(Rc::new(closure))))
    }
}

impl<'src> StmtVisitor<'src, Result<(), LoxException<'src>>> for Interpreter<'src> {
    fn visit_expression_stmt(&mut self, stmt: &Expression<'src>) -> Result<(), LoxException<'src>> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Print<'src>) -> Result<(), LoxException<'src>> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{value}");
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &Var<'src>) -> Result<(), LoxException<'src>> {
        let value = match stmt.initializer {
            Some(ref expr) => self.evaluate(expr)?,
            None => LoxObject::Literal(LoxLiteral::Nil),
        };

        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme, value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &Block<'src>) -> Result<(), LoxException<'src>> {
        let environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &self.environment,
        )))));
        self.execute_block(&stmt.statements, environment)
    }

    fn visit_if_stmt(&mut self, stmt: &If<'src>) -> Result<(), LoxException<'src>> {
        let condition_value = self.evaluate(&stmt.condition)?;

        if self.is_truthy(&condition_value) {
            self.execute(&stmt.then_branch)?;
        } else if let Some(ref else_branch) = stmt.else_branch {
            self.execute(else_branch)?;
        }

        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &While<'src>) -> Result<(), LoxException<'src>> {
        loop {
            let condition_value = self.evaluate(&stmt.condition)?;
            if !self.is_truthy(&condition_value) {
                break;
            }
            self.execute(&stmt.body)?;
            if self.active_break {
                break;
            }
        }
        self.active_break = false;
        Ok(())
    }

    fn visit_break_stmt(&mut self) -> Result<(), LoxException<'src>> {
        self.active_break = true;
        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: &Function<'src>) -> Result<(), LoxException<'src>> {
        let function_name = stmt.name.lexeme;
        let function = LoxFunction::new(
            &stmt.closure,
            Rc::clone(&self.environment),
            Some(function_name),
            false,
        );
        self.environment.borrow_mut().define(
            function_name,
            LoxObject::Callable(LoxCallable::Function(Rc::new(function))),
        );
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &Return<'src>) -> Result<(), LoxException<'src>> {
        let value = self.evaluate(&stmt.value)?;
        Err(LoxException::Return(value))
    }

    fn visit_class_stmt(&mut self, stmt: &Class<'src>) -> Result<(), LoxException<'src>> {
        let mut superclass = None;
        if let Some(ref superclass_expr) = stmt.superclass {
            let superclass_err = LoxException::RuntimeError(RuntimeError::new(
                stmt.name.line,
                String::from("Superclass must be a class."),
            ));

            let superclass_obj = self.evaluate(superclass_expr)?;
            if let LoxObject::Callable(ref callable) = superclass_obj {
                match callable {
                    LoxCallable::Class(class) => {
                        superclass = Some(Rc::clone(class));
                    }
                    _ => {
                        return Err(superclass_err);
                    }
                }
            } else {
                return Err(superclass_err);
            }
        }

        let class_name = stmt.name.lexeme;
        self.environment
            .borrow_mut()
            .define(class_name, LoxObject::Literal(LoxLiteral::Nil));

        if superclass.is_some() {
            self.environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
                &self.environment,
            )))));
            self.environment.borrow_mut().define(
                "super",
                LoxObject::Callable(LoxCallable::Class(Rc::clone(superclass.as_ref().unwrap()))),
            );
        }

        let mut methods = HashMap::new();
        for method in stmt.methods.iter() {
            if let Stmt::Function(function) = method {
                let method_name = function.name.lexeme;
                let lox_fun = LoxFunction::new(
                    &function.closure,
                    Rc::clone(&self.environment),
                    Some(method_name),
                    method_name == "init",
                );
                methods.insert(method_name, lox_fun);
            }
        }

        if superclass.is_some() {
            let enclosing = self.environment.borrow_mut().enclosing.take().unwrap();
            self.environment = enclosing;
        }

        let klass = LoxClass::new(class_name, superclass, methods);

        self.environment.borrow_mut().assign(
            &stmt.name,
            LoxObject::Callable(LoxCallable::Class(Rc::new(klass))),
        )?;
        Ok(())
    }
}
