use crate::expr::{Expr, ExprVisitor};
use crate::token::Token;

pub trait StmtVisitor: ExprVisitor {
    fn visit_expression_stmt(&mut self, stmt: &Expression);
    fn visit_print_stmt(&mut self, stmt: &Print);
    fn visit_var_stmt(&mut self, stmt: &Var);
    fn visit_block_stmt(&mut self, stmt: &Block);
}

pub trait Stmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor);
}

pub struct Expression {
    pub expression: Box<dyn Expr>,
}
impl Expression {
    pub fn new(expression: Box<dyn Expr>) -> Self {
        Expression { expression }
    }
}
impl Stmt for Expression {
    fn accept(&self, visitor: &mut dyn StmtVisitor) {
        visitor.visit_expression_stmt(self);
    }
}

pub struct Print {
    pub expression: Box<dyn Expr>,
}
impl Print {
    pub fn new(expression: Box<dyn Expr>) -> Self {
        Print { expression }
    }
}
impl Stmt for Print {
    fn accept(&self, visitor: &mut dyn StmtVisitor) {
        visitor.visit_print_stmt(self);
    }
}

pub struct Var {
    pub name: Token,
    pub initializer: Option<Box<dyn Expr>>,
}
impl Var {
    pub fn new(name: Token, initializer: Option<Box<dyn Expr>>) -> Self {
        Var { name, initializer }
    }
}
impl Stmt for Var {
    fn accept(&self, visitor: &mut dyn StmtVisitor) {
        visitor.visit_var_stmt(self);
    }
}

pub struct Block {
    pub statements: Vec<Box<dyn Stmt>>,
}
impl Block {
    pub fn new(statements: Vec<Box<dyn Stmt>>) -> Self {
        Block { statements }
    }
}
impl Stmt for Block {
    fn accept(&self, visitor: &mut dyn StmtVisitor) {
        visitor.visit_block_stmt(self);
    }
}
