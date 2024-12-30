use crate::{
    environment::Environment,
    expr::{
        Assign, Binary, Call, Closure, Expr, ExprVisitor, Get, Grouping, Literal, Logical, Set,
        Ternary, Unary, Variable,
    },
    lox_callable::{Callable, LoxCallable},
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

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
    locals: HashMap<Token, usize>,
    active_break: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new(None)));
        // Implement global "clock" function.
        let clock_function = |_: &mut Interpreter, _: Vec<LoxObject>| {
            LoxObject::Literal(LoxLiteral::Number(
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("SystemTime should be after UNIX EPOCH in global clock function.")
                    .as_secs_f64(),
            ))
        };
        let global_clock = LoxObject::Callable(Rc::new(Callable::NativeFun(NativeFunction::new(
            clock_function,
            0,
            String::from("<native fn>"),
        ))));

        globals
            .borrow_mut()
            .define(String::from("clock"), global_clock);

        let environment = Rc::clone(&globals);

        Interpreter {
            globals,
            environment,
            locals: HashMap::new(),
            active_break: false,
        }
    }

    pub fn interpret(&mut self, statements: &Vec<Stmt>) -> Result<(), LoxException> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxException> {
        stmt.accept(self)
    }

    pub fn resolve(&mut self, token: Token, depth: usize) {
        self.locals.insert(token, depth);
    }

    pub fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        environment: Rc<RefCell<Environment>>,
    ) -> Result<(), LoxException> {
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

    fn evaluate(&mut self, expr: &Expr) -> Result<LoxObject, LoxException> {
        expr.accept(self)
    }

    fn is_truthy(&self, object: &LoxObject) -> bool {
        match &object {
            LoxObject::Literal(LoxLiteral::Nil) => false,
            LoxObject::Literal(LoxLiteral::Boolean(res)) => *res,
            _ => true,
        }
    }

    fn look_up_variable(&mut self, name: &Token) -> Result<LoxObject, LoxException> {
        match self.locals.get(name) {
            Some(&distance) => Ok(self.environment.borrow().get_at(distance, &name.lexeme)),
            None => self.globals.borrow().get(name),
        }
    }
}

impl ExprVisitor<Result<LoxObject, LoxException>> for Interpreter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> Result<LoxObject, LoxException> {
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

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> Result<LoxObject, LoxException> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> Result<LoxObject, LoxException> {
        Ok(LoxObject::Literal(expr.value.clone()))
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> Result<LoxObject, LoxException> {
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

    fn visit_ternary_expr(&mut self, expr: &Ternary) -> Result<LoxObject, LoxException> {
        let condition = self.evaluate(&expr.condition)?;
        match self.is_truthy(&condition) {
            true => self.evaluate(&expr.left),
            false => self.evaluate(&expr.right),
        }
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> Result<LoxObject, LoxException> {
        self.look_up_variable(&expr.name)
    }

    fn visit_assign_expr(&mut self, expr: &Assign) -> Result<LoxObject, LoxException> {
        let value = self.evaluate(&expr.value)?;
        match self.locals.get(&expr.name) {
            Some(&distance) => Ok(self
                .environment
                .borrow_mut()
                .assign_at(distance, &expr.name, value)),
            None => self.globals.borrow_mut().assign(&expr.name, value),
        }
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> Result<LoxObject, LoxException> {
        let left = self.evaluate(&expr.left)?;

        match expr.operator.token_type {
            TokenType::Or if self.is_truthy(&left) => Ok(left),
            TokenType::And if !self.is_truthy(&left) => Ok(left),
            _ => self.evaluate(&expr.right),
        }
    }

    fn visit_call_expr(&mut self, expr: &Call) -> Result<LoxObject, LoxException> {
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

    fn visit_get_expr(&mut self, expr: &Get) -> Result<LoxObject, LoxException> {
        let object = self.evaluate(&expr.object)?;
        match object {
            LoxObject::Instance(instance) => instance.borrow().get(&expr.name),
            _ => Err(LoxException::RuntimeError(RuntimeError::new(
                expr.name.line,
                String::from("Only instances have properties."),
            ))),
        }
    }

    fn visit_set_expr(&mut self, expr: &Set) -> Result<LoxObject, LoxException> {
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

    fn visit_closure_expr(&mut self, expr: &Closure) -> Result<LoxObject, LoxException> {
        let closure = LoxFunction::new(expr, Rc::clone(&self.environment), None);
        Ok(LoxObject::Callable(Rc::new(Callable::Function(closure))))
    }
}

impl StmtVisitor<Result<(), LoxException>> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> Result<(), LoxException> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &Print) -> Result<(), LoxException> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{value}");
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &Var) -> Result<(), LoxException> {
        let value = match stmt.initializer {
            Some(ref expr) => self.evaluate(expr)?,
            None => LoxObject::Literal(LoxLiteral::Nil),
        };

        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), value);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &Block) -> Result<(), LoxException> {
        let environment = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &self.environment,
        )))));
        self.execute_block(&stmt.statements, environment)
    }

    fn visit_if_stmt(&mut self, stmt: &If) -> Result<(), LoxException> {
        let condition_value = self.evaluate(&stmt.condition)?;

        if self.is_truthy(&condition_value) {
            self.execute(&stmt.then_branch)?;
        } else if let Some(ref else_branch) = stmt.else_branch {
            self.execute(else_branch)?;
        }

        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &While) -> Result<(), LoxException> {
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

    fn visit_break_stmt(&mut self) -> Result<(), LoxException> {
        self.active_break = true;
        Ok(())
    }

    fn visit_function_stmt(&mut self, stmt: &Function) -> Result<(), LoxException> {
        let function_name = stmt.name.lexeme.clone();
        let function = LoxFunction::new(
            &stmt.closure,
            Rc::clone(&self.environment),
            Some(function_name.clone()),
        );
        self.environment.borrow_mut().define(
            function_name,
            LoxObject::Callable(Rc::new(Callable::Function(function))),
        );
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &Return) -> Result<(), LoxException> {
        let value = self.evaluate(&stmt.value)?;
        Err(LoxException::Return(value))
    }

    fn visit_class_stmt(&mut self, stmt: &Class) -> Result<(), LoxException> {
        let class_name = stmt.name.lexeme.clone();
        self.environment
            .borrow_mut()
            .define(class_name.clone(), LoxObject::Literal(LoxLiteral::Nil));
        let klass = LoxClass::new(class_name);
        self.environment.borrow_mut().assign(
            &stmt.name,
            LoxObject::Callable(Rc::new(Callable::Class(klass))),
        )?;
        Ok(())
    }
}
