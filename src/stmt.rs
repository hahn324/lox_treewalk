use crate::expr::{Expr, ExprVisitor};
use crate::token::Token;

pub trait StmtVisitor: ExprVisitor {
    fn visit_expression_stmt(&mut self, stmt: &Expression);
    fn visit_print_stmt(&mut self, stmt: &Print);
    fn visit_var_stmt(&mut self, stmt: &Var);
    fn visit_block_stmt(&mut self, stmt: &Block);
    fn visit_if_stmt(&mut self, stmt: &If);
    fn visit_while_stmt(&mut self, stmt: &While);
    fn visit_break_stmt(&mut self);
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

pub struct If {
    pub condition: Box<dyn Expr>,
    pub then_branch: Box<dyn Stmt>,
    pub else_branch: Option<Box<dyn Stmt>>,
}
impl If {
    pub fn new(
        condition: Box<dyn Expr>,
        then_branch: Box<dyn Stmt>,
        else_branch: Option<Box<dyn Stmt>>,
    ) -> Self {
        If {
            condition,
            then_branch,
            else_branch,
        }
    }
}
impl Stmt for If {
    fn accept(&self, visitor: &mut dyn StmtVisitor) {
        visitor.visit_if_stmt(self);
    }
}

pub struct While {
    pub condition: Box<dyn Expr>,
    pub body: Box<dyn Stmt>,
}
impl While {
    pub fn new(condition: Box<dyn Expr>, body: Box<dyn Stmt>) -> Self {
        While { condition, body }
    }
}
impl Stmt for While {
    fn accept(&self, visitor: &mut dyn StmtVisitor) {
        visitor.visit_while_stmt(self);
    }
}

pub struct Break;
impl Stmt for Break {
    fn accept(&self, visitor: &mut dyn StmtVisitor) {
        visitor.visit_break_stmt();
    }
}
