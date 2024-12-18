use crate::expr::{
    Assign, Binary, Call, Closure, Expr, ExprVisitor, Grouping, Literal, Logical, Ternary, Unary,
    Variable,
};
use crate::stmt::{Block, Expression, Function, If, Print, Return, Stmt, StmtVisitor, Var, While};
use crate::{interpreter::Interpreter, report, token::Token};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum FunctionType {
    None,
    Function,
}

pub struct Resolver<'interpreter> {
    interpreter: &'interpreter mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    current_function: FunctionType,
    pub had_error: bool,
}
impl<'interpreter> Resolver<'interpreter> {
    pub fn new(interpreter: &'interpreter mut Interpreter) -> Self {
        Resolver {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
            had_error: false,
        }
    }

    fn resolver_error(&mut self, line: usize, loc: &str, message: &str) {
        self.had_error = true;
        report(line, loc, message);
    }

    fn get_cur_scope(&mut self) -> &mut HashMap<String, bool> {
        let cur_scope_idx = self.scopes.len() - 1;
        &mut self.scopes[cur_scope_idx]
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let scope = self.get_cur_scope();
        let already_declared = scope.contains_key(&name.lexeme);

        scope.insert(name.lexeme.clone(), false);

        if already_declared {
            self.resolver_error(
                name.line,
                &format!("at '{}'", &name.lexeme),
                "Already a variable with this name in this scope.",
            );
        }
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        let scope = self.get_cur_scope();
        scope.insert(name.lexeme.clone(), true);
    }

    fn resolve_local(&mut self, name: &Token) {
        for idx in (0..self.scopes.len()).rev() {
            if self.scopes[idx].contains_key(&name.lexeme) {
                self.interpreter
                    .resolve(name.clone(), self.scopes.len() - 1 - idx);
                return;
            }
        }
    }

    fn resolve_function(&mut self, closure: &Closure, function_type: FunctionType) {
        let enclosing_function = self.current_function;
        self.current_function = function_type;

        self.begin_scope();
        for param in closure.params.iter() {
            self.declare(param);
            self.define(param);
        }
        self.resolve_statements(&closure.body);
        self.end_scope();

        self.current_function = enclosing_function;
    }

    pub fn resolve_statements(&mut self, statements: &Vec<Stmt>) {
        for statement in statements {
            self.resolve_stmt(statement);
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        stmt.accept(self);
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        expr.accept(self);
    }
}

impl<'interpreter> ExprVisitor<()> for Resolver<'interpreter> {
    fn visit_binary_expr(&mut self, expr: &Binary) {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) {
        self.resolve_expr(&expr.expression);
    }

    fn visit_literal_expr(&mut self, _: &Literal) {}

    fn visit_unary_expr(&mut self, expr: &Unary) {
        self.resolve_expr(&expr.right);
    }

    fn visit_ternary_expr(&mut self, expr: &Ternary) {
        self.resolve_expr(&expr.condition);
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
    }

    fn visit_variable_expr(&mut self, expr: &Variable) {
        if !self.scopes.is_empty() && self.get_cur_scope().get(&expr.name.lexeme) == Some(&false) {
            self.resolver_error(
                expr.name.line,
                &format!("at '{}'", &expr.name.lexeme),
                "Can't read local variable in its own initializer.",
            );
        }

        self.resolve_local(&expr.name);
    }

    fn visit_assign_expr(&mut self, expr: &Assign) {
        self.resolve_expr(&expr.value);
        self.resolve_local(&expr.name);
    }

    fn visit_logical_expr(&mut self, expr: &Logical) {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
    }

    fn visit_call_expr(&mut self, expr: &Call) {
        self.resolve_expr(&expr.callee);
        for argument in expr.arguments.iter() {
            self.resolve_expr(argument);
        }
    }

    fn visit_closure_expr(&mut self, expr: &Closure) {
        self.resolve_function(expr, FunctionType::Function);
    }
}

impl<'interpreter> StmtVisitor<()> for Resolver<'interpreter> {
    fn visit_expression_stmt(&mut self, stmt: &Expression) {
        self.resolve_expr(&stmt.expression);
    }

    fn visit_print_stmt(&mut self, stmt: &Print) {
        self.resolve_expr(&stmt.expression);
    }

    fn visit_var_stmt(&mut self, stmt: &Var) {
        self.declare(&stmt.name);
        if let Some(ref initializer) = stmt.initializer {
            self.resolve_expr(initializer);
        }
        self.define(&stmt.name);
    }

    fn visit_block_stmt(&mut self, stmt: &Block) {
        self.begin_scope();
        self.resolve_statements(&stmt.statements);
        self.end_scope();
    }

    fn visit_if_stmt(&mut self, stmt: &If) {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.then_branch);
        if let Some(ref else_stmt) = stmt.else_branch {
            self.resolve_stmt(else_stmt);
        }
    }

    fn visit_while_stmt(&mut self, stmt: &While) {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.body);
    }

    fn visit_break_stmt(&mut self) {}

    fn visit_function_stmt(&mut self, stmt: &Function) {
        self.declare(&stmt.name);
        self.define(&stmt.name);

        self.resolve_function(&stmt.closure, FunctionType::Function);
    }

    fn visit_return_stmt(&mut self, stmt: &Return) {
        if self.current_function == FunctionType::None {
            self.resolver_error(
                stmt.keyword.line,
                "at 'return'",
                "Can't return from top-level code.",
            );
        }
        self.resolve_expr(&stmt.value);
    }
}
