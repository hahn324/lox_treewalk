use crate::environment::Environment;
use crate::expr::{Assign, Binary, Expr, ExprVisitor, Grouping, Literal, Ternary, Unary, Variable};
use crate::stmt::{Block, Expression, Print, Stmt, StmtVisitor, Var};
use crate::token::{LoxLiteral, Token};
use crate::token_type::TokenType;

pub struct Interpreter {
    environment: Environment,
    pub had_runtime_error: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
            had_runtime_error: false,
        }
    }

    pub fn interpret(&mut self, statements: Vec<Box<dyn Stmt>>) {
        for statement in statements {
            if self.had_runtime_error {
                break;
            }
            self.execute(&statement);
        }
    }

    fn execute(&mut self, stmt: &Box<dyn Stmt>) {
        stmt.accept(self);
    }

    fn execute_block(&mut self, statements: &Vec<Box<dyn Stmt>>) {
        // Create new environment for block.
        self.environment.create_environment();

        for statement in statements {
            self.execute(statement);
        }

        // Clenup environment for block.
        self.environment.delete_environment();
    }

    fn evaluate(&mut self, expr: &Box<dyn Expr>) -> LoxLiteral {
        if self.had_runtime_error {
            return LoxLiteral::Nil;
        }

        expr.accept(self)
    }

    fn is_truthy(&self, object: LoxLiteral) -> bool {
        match object {
            LoxLiteral::Nil => false,
            LoxLiteral::Boolean(res) => res,
            _ => true,
        }
    }

    fn set_runtime_error(&mut self, token: &Token, message: &str) -> LoxLiteral {
        self.had_runtime_error = true;
        eprintln!("[line {}] {}", token.line, message);
        LoxLiteral::Nil
    }
}

impl ExprVisitor for Interpreter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> LoxLiteral {
        let left = self.evaluate(&expr.left);
        let right = self.evaluate(&expr.right);

        if self.had_runtime_error {
            return LoxLiteral::Nil;
        }
        match expr.operator.token_type {
            TokenType::Minus => match (left, right) {
                (LoxLiteral::Number(left_val), LoxLiteral::Number(right_val)) => {
                    LoxLiteral::Number(left_val - right_val)
                }
                _ => self.set_runtime_error(&expr.operator, "Operands must be numbers."),
            },
            TokenType::Slash => match (left, right) {
                (LoxLiteral::Number(left_val), LoxLiteral::Number(right_val)) => {
                    match right_val == 0.0 {
                        true => self.set_runtime_error(&expr.operator, "Cannot divide by zero."),
                        false => LoxLiteral::Number(left_val / right_val),
                    }
                }
                _ => self.set_runtime_error(&expr.operator, "Operands must be numbers."),
            },
            TokenType::Star => match (left, right) {
                (LoxLiteral::Number(left_val), LoxLiteral::Number(right_val)) => {
                    LoxLiteral::Number(left_val * right_val)
                }
                _ => self.set_runtime_error(&expr.operator, "Operands must be numbers."),
            },
            TokenType::Plus => match (left, right) {
                (LoxLiteral::Number(left_val), LoxLiteral::Number(right_val)) => {
                    LoxLiteral::Number(left_val + right_val)
                }
                (LoxLiteral::String(left_val), LoxLiteral::String(right_val)) => {
                    LoxLiteral::String(format!("{left_val}{right_val}"))
                }
                (LoxLiteral::String(left_val), right) => {
                    LoxLiteral::String(format!("{left_val}{}", right.stringify()))
                }
                (left, LoxLiteral::String(right_val)) => {
                    LoxLiteral::String(format!("{}{right_val}", left.stringify()))
                }
                _ => self.set_runtime_error(
                    &expr.operator,
                    "Operands must be two numbers or one must be a string.",
                ),
            },
            TokenType::Greater => match (left, right) {
                (LoxLiteral::Number(left_val), LoxLiteral::Number(right_val)) => {
                    LoxLiteral::Boolean(left_val > right_val)
                }
                _ => self.set_runtime_error(&expr.operator, "Operands must be numbers."),
            },
            TokenType::GreaterEqual => match (left, right) {
                (LoxLiteral::Number(left_val), LoxLiteral::Number(right_val)) => {
                    LoxLiteral::Boolean(left_val >= right_val)
                }
                _ => self.set_runtime_error(&expr.operator, "Operands must be numbers."),
            },
            TokenType::Less => match (left, right) {
                (LoxLiteral::Number(left_val), LoxLiteral::Number(right_val)) => {
                    LoxLiteral::Boolean(left_val < right_val)
                }
                _ => self.set_runtime_error(&expr.operator, "Operands must be numbers."),
            },
            TokenType::LessEqual => match (left, right) {
                (LoxLiteral::Number(left_val), LoxLiteral::Number(right_val)) => {
                    LoxLiteral::Boolean(left_val <= right_val)
                }
                _ => self.set_runtime_error(&expr.operator, "Operands must be numbers."),
            },
            TokenType::BangEqual => LoxLiteral::Boolean(left != right),
            TokenType::EqualEqual => LoxLiteral::Boolean(left == right),
            TokenType::Comma => right,
            _ => unreachable!("All valid Binary operators are accounted for in above arms."),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> LoxLiteral {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> LoxLiteral {
        expr.value.clone()
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> LoxLiteral {
        let right = self.evaluate(&expr.right);

        if self.had_runtime_error {
            return LoxLiteral::Nil;
        }
        match expr.operator.token_type {
            TokenType::Minus => match right {
                LoxLiteral::Number(val) => LoxLiteral::Number(-val),
                _ => self.set_runtime_error(&expr.operator, "Operand must be a number."),
            },
            TokenType::Bang => LoxLiteral::Boolean(!self.is_truthy(right)),
            _ => unreachable!("All valid Unary operators are accounted for in above arms."),
        }
    }

    fn visit_ternary_expr(&mut self, expr: &Ternary) -> LoxLiteral {
        let condition = self.evaluate(&expr.condition);
        match self.is_truthy(condition) {
            true => self.evaluate(&expr.left),
            false => self.evaluate(&expr.right),
        }
    }

    fn visit_variable_expr(&mut self, expr: &Variable) -> LoxLiteral {
        match self.environment.get(&expr.name) {
            Some(val) => val.clone(),
            None => self.set_runtime_error(
                &expr.name,
                &format!("Undefined variable '{}'.", &expr.name.lexeme),
            ),
        }
    }

    fn visit_assign_expr(&mut self, expr: &Assign) -> LoxLiteral {
        let value = self.evaluate(&expr.value);
        match self
            .environment
            .assign(expr.name.lexeme.clone(), value.clone())
        {
            Some(_) => value,
            None => {
                self.set_runtime_error(
                    &expr.name,
                    &format!("Undefined variable '{}'.", &expr.name.lexeme),
                );
                LoxLiteral::Nil
            }
        }
    }
}

impl StmtVisitor for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &Expression) {
        self.evaluate(&stmt.expression);
    }

    fn visit_print_stmt(&mut self, stmt: &Print) {
        let value = self.evaluate(&stmt.expression);
        if !self.had_runtime_error {
            println!("{}", value.stringify());
        }
    }

    fn visit_var_stmt(&mut self, stmt: &Var) {
        let value = match stmt.initializer {
            Some(ref expr) => self.evaluate(expr),
            None => LoxLiteral::Nil,
        };

        self.environment.define(stmt.name.lexeme.clone(), value);
    }

    fn visit_block_stmt(&mut self, stmt: &Block) {
        self.execute_block(&stmt.statements)
    }
}
