use crate::expr::{
    Assign, Binary, Call, Closure, Expr, ExprVisitor, Grouping, Literal, Logical, Ternary, Unary,
    Variable,
};
use crate::stmt::{Block, Expression, Function, If, Print, Return, Stmt, StmtVisitor, Var, While};
use crate::{
    environment::Environment,
    lox_callable::LoxCallable,
    lox_exception::{LoxException, RuntimeError},
    lox_object::{LoxLiteral, LoxObject},
    token_type::TokenType,
};
use std::{cell::RefCell, rc::Rc, time::SystemTime};

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
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
        let global_clock = LoxObject::Callable(LoxCallable::new_native_fun(
            0,
            clock_function,
            String::from("<native fn>"),
        ));

        globals
            .borrow_mut()
            .define(String::from("clock"), global_clock);

        let environment = Rc::clone(&globals);

        Interpreter {
            globals,
            environment,
            active_break: false,
        }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<(), LoxException> {
        for statement in statements.iter() {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxException> {
        stmt.accept(self)
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
                ) => Ok(LoxObject::Literal(LoxLiteral::String(format!(
                    "{left_val}{right_val}"
                )))),
                (LoxObject::Literal(LoxLiteral::String(left_val)), right) => Ok(
                    LoxObject::Literal(LoxLiteral::String(format!("{left_val}{right}",))),
                ),
                (left, LoxObject::Literal(LoxLiteral::String(right_val))) => Ok(
                    LoxObject::Literal(LoxLiteral::String(format!("{left}{right_val}",))),
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
        self.environment.borrow().get(&expr.name)
    }

    fn visit_assign_expr(&mut self, expr: &Assign) -> Result<LoxObject, LoxException> {
        let value = self.evaluate(&expr.value)?;
        self.environment.borrow_mut().assign(&expr.name, value)
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
            LoxObject::Callable(function) => {
                if arguments.len() != function.arity() {
                    return Err(LoxException::RuntimeError(RuntimeError::new(
                        expr.paren.line,
                        format!(
                            "Expected {} arguments but got {}.",
                            function.arity(),
                            arguments.len()
                        ),
                    )));
                }
                function.call(self, arguments)
            }
            _ => Err(LoxException::RuntimeError(RuntimeError::new(
                expr.paren.line,
                String::from("Can only call functions and classes."),
            ))),
        }
    }

    fn visit_closure_expr(&mut self, expr: &Closure) -> Result<LoxObject, LoxException> {
        let closure = LoxCallable::new_lox_closure(expr, Rc::clone(&self.environment));
        Ok(LoxObject::Callable(closure))
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
        let function = LoxCallable::new_lox_fun(stmt, Rc::clone(&self.environment));
        self.environment
            .borrow_mut()
            .define(function_name, LoxObject::Callable(function));
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &Return) -> Result<(), LoxException> {
        let value = self.evaluate(&stmt.value)?;
        Err(LoxException::Return(value))
    }
}
