use crate::{
    expr::{
        Assign, Binary, Call, Closure, Expr, ExprVisitor, Get, Grouping, Literal, Logical, Set,
        Super, Ternary, This, Unary, Variable,
    },
    interpreter::Interpreter,
    lox_object::LoxLiteral,
    report,
    stmt::{Block, Class, Expression, Function, If, Print, Return, Stmt, StmtVisitor, Var, While},
    token::Token,
};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
enum FunctionType {
    None,
    Function,
    Method,
    Initializer,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum ClassType {
    None,
    Class,
    Subclass,
}

pub struct Resolver<'interpreter, 'src> {
    interpreter: &'interpreter mut Interpreter<'src>,
    scopes: Vec<HashMap<&'src str, bool>>,
    current_function: FunctionType,
    current_class: ClassType,
    pub had_error: bool,
}
impl<'interpreter, 'src> Resolver<'interpreter, 'src> {
    pub fn new(interpreter: &'interpreter mut Interpreter<'src>) -> Self {
        Resolver {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
            current_class: ClassType::None,
            had_error: false,
        }
    }

    fn resolver_error(&mut self, line: usize, loc: &str, message: &str) {
        self.had_error = true;
        report(line, loc, message);
    }

    fn get_cur_scope(&mut self) -> &mut HashMap<&'src str, bool> {
        let cur_scope_idx = self.scopes.len() - 1;
        &mut self.scopes[cur_scope_idx]
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token<'src>) {
        if self.scopes.is_empty() {
            return;
        }

        let scope = self.get_cur_scope();
        let already_declared = scope.contains_key(name.lexeme);

        scope.insert(name.lexeme, false);

        if already_declared {
            self.resolver_error(
                name.line,
                &format!("at '{}'", &name.lexeme),
                "Already a variable with this name in this scope.",
            );
        }
    }

    fn define(&mut self, name: &Token<'src>) {
        if self.scopes.is_empty() {
            return;
        }

        self.get_cur_scope().insert(name.lexeme, true);
    }

    fn resolve_local(&mut self, name: &Token<'src>) {
        for idx in (0..self.scopes.len()).rev() {
            if self.scopes[idx].contains_key(name.lexeme) {
                self.interpreter
                    .resolve(name.clone(), self.scopes.len() - 1 - idx);
                return;
            }
        }
    }

    fn resolve_function(&mut self, closure: &Closure<'src>, function_type: FunctionType) {
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

    pub fn resolve_statements(&mut self, statements: &Vec<Stmt<'src>>) {
        for statement in statements {
            self.resolve_stmt(statement);
        }
    }

    fn resolve_stmt(&mut self, stmt: &Stmt<'src>) {
        stmt.accept(self);
    }

    fn resolve_expr(&mut self, expr: &Expr<'src>) {
        expr.accept(self);
    }
}

impl<'interpreter, 'src> ExprVisitor<'src, ()> for Resolver<'interpreter, 'src> {
    fn visit_binary_expr(&mut self, expr: &Binary<'src>) {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping<'src>) {
        self.resolve_expr(&expr.expression);
    }

    fn visit_literal_expr(&mut self, _: &Literal) {}

    fn visit_unary_expr(&mut self, expr: &Unary<'src>) {
        self.resolve_expr(&expr.right);
    }

    fn visit_ternary_expr(&mut self, expr: &Ternary<'src>) {
        self.resolve_expr(&expr.condition);
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
    }

    fn visit_variable_expr(&mut self, expr: &Variable<'src>) {
        if !self.scopes.is_empty() && self.get_cur_scope().get(&expr.name.lexeme) == Some(&false) {
            self.resolver_error(
                expr.name.line,
                &format!("at '{}'", &expr.name.lexeme),
                "Can't read local variable in its own initializer.",
            );
        }

        self.resolve_local(&expr.name);
    }

    fn visit_assign_expr(&mut self, expr: &Assign<'src>) {
        self.resolve_expr(&expr.value);
        self.resolve_local(&expr.name);
    }

    fn visit_logical_expr(&mut self, expr: &Logical<'src>) {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
    }

    fn visit_call_expr(&mut self, expr: &Call<'src>) {
        self.resolve_expr(&expr.callee);
        for argument in expr.arguments.iter() {
            self.resolve_expr(argument);
        }
    }

    fn visit_get_expr(&mut self, expr: &Get<'src>) {
        self.resolve_expr(&expr.object);
    }

    fn visit_set_expr(&mut self, expr: &Set<'src>) -> () {
        self.resolve_expr(&expr.value);
        self.resolve_expr(&expr.object);
    }

    fn visit_this_expr(&mut self, expr: &This<'src>) {
        if self.current_class == ClassType::None {
            self.resolver_error(
                expr.keyword.line,
                "at 'this'",
                "Can't use 'this' outside of a class.",
            );
        }

        self.resolve_local(&expr.keyword);
    }

    fn visit_super_expr(&mut self, expr: &Super<'src>) {
        match self.current_class {
            ClassType::None => self.resolver_error(
                expr.keyword.line,
                "at 'super'",
                "Can't use 'super' outside of a class.",
            ),
            ClassType::Class => self.resolver_error(
                expr.keyword.line,
                "at 'super'",
                "Can't use 'super' in a class with no superclass.",
            ),
            ClassType::Subclass => self.resolve_local(&expr.keyword),
        }
    }

    fn visit_closure_expr(&mut self, expr: &Closure<'src>) {
        self.resolve_function(expr, FunctionType::Function);
    }
}

impl<'interpreter, 'src> StmtVisitor<'src, ()> for Resolver<'interpreter, 'src> {
    fn visit_expression_stmt(&mut self, stmt: &Expression<'src>) {
        self.resolve_expr(&stmt.expression);
    }

    fn visit_print_stmt(&mut self, stmt: &Print<'src>) {
        self.resolve_expr(&stmt.expression);
    }

    fn visit_var_stmt(&mut self, stmt: &Var<'src>) {
        self.declare(&stmt.name);
        if let Some(ref initializer) = stmt.initializer {
            self.resolve_expr(initializer);
        }
        self.define(&stmt.name);
    }

    fn visit_block_stmt(&mut self, stmt: &Block<'src>) {
        self.begin_scope();
        self.resolve_statements(&stmt.statements);
        self.end_scope();
    }

    fn visit_if_stmt(&mut self, stmt: &If<'src>) {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.then_branch);
        if let Some(ref else_stmt) = stmt.else_branch {
            self.resolve_stmt(else_stmt);
        }
    }

    fn visit_while_stmt(&mut self, stmt: &While<'src>) {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.body);
    }

    fn visit_break_stmt(&mut self) {}

    fn visit_function_stmt(&mut self, stmt: &Function<'src>) {
        self.declare(&stmt.name);
        self.define(&stmt.name);

        self.resolve_function(&stmt.closure, FunctionType::Function);
    }

    fn visit_return_stmt(&mut self, stmt: &Return<'src>) {
        if self.current_function == FunctionType::None {
            self.resolver_error(
                stmt.keyword.line,
                "at 'return'",
                "Can't return from top-level code.",
            );
        }
        match &stmt.value {
            Expr::Literal(literal) if literal.value == LoxLiteral::Nil => (),
            _ if self.current_function == FunctionType::Initializer => {
                self.resolver_error(
                    stmt.keyword.line,
                    "at 'return",
                    "Can't return a value from an initializer.",
                );
            }
            _ => {
                self.resolve_expr(&stmt.value);
            }
        }
    }

    fn visit_class_stmt(&mut self, stmt: &Class<'src>) {
        let enclosing_class = self.current_class;
        self.current_class = ClassType::Class;

        self.declare(&stmt.name);
        self.define(&stmt.name);

        if let Some(ref superclass) = stmt.superclass {
            if let Expr::Variable(superclass_var) = superclass.as_ref() {
                if stmt.name.lexeme == superclass_var.name.lexeme {
                    self.resolver_error(
                        superclass_var.name.line,
                        &format!("at '{}'", superclass_var.name.lexeme),
                        "A class can't inherit from itself",
                    );
                }
            }
            self.current_class = ClassType::Subclass;
            self.resolve_expr(superclass);
            self.begin_scope();
            self.get_cur_scope().insert("super", true);
        }

        self.begin_scope();
        self.get_cur_scope().insert("this", true);

        for method in stmt.methods.iter() {
            if let Stmt::Function(function) = method {
                let declaration = match function.name.lexeme == "init" {
                    true => FunctionType::Initializer,
                    false => FunctionType::Method,
                };
                self.resolve_function(&function.closure, declaration);
            }
        }

        self.end_scope();

        if stmt.superclass.is_some() {
            self.end_scope();
        }

        self.current_class = enclosing_class;
    }
}
