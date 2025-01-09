use crate::{
    expr::{Closure, Expr},
    token::Token,
};

pub trait StmtVisitor<'src, T> {
    fn visit_expression_stmt(&mut self, stmt: &Expression<'src>) -> T;
    fn visit_print_stmt(&mut self, stmt: &Print<'src>) -> T;
    fn visit_var_stmt(&mut self, stmt: &Var<'src>) -> T;
    fn visit_block_stmt(&mut self, stmt: &Block<'src>) -> T;
    fn visit_if_stmt(&mut self, stmt: &If<'src>) -> T;
    fn visit_while_stmt(&mut self, stmt: &While<'src>) -> T;
    fn visit_break_stmt(&mut self) -> T;
    fn visit_function_stmt(&mut self, stmt: &Function<'src>) -> T;
    fn visit_return_stmt(&mut self, stmt: &Return<'src>) -> T;
    fn visit_class_stmt(&mut self, stmt: &Class<'src>) -> T;
}

#[derive(Debug, Clone)]
pub enum Stmt<'src> {
    Expression(Expression<'src>),
    Print(Print<'src>),
    Var(Var<'src>),
    Block(Block<'src>),
    If(If<'src>),
    While(While<'src>),
    Break,
    Function(Function<'src>),
    Return(Return<'src>),
    Class(Class<'src>),
}

impl<'src> Stmt<'src> {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<'src, T>) -> T {
        match self {
            Stmt::Expression(expression) => visitor.visit_expression_stmt(expression),
            Stmt::Print(print) => visitor.visit_print_stmt(print),
            Stmt::Var(var) => visitor.visit_var_stmt(var),
            Stmt::Block(block) => visitor.visit_block_stmt(block),
            Stmt::If(if_stmt) => visitor.visit_if_stmt(if_stmt),
            Stmt::While(while_stmt) => visitor.visit_while_stmt(while_stmt),
            Stmt::Break => visitor.visit_break_stmt(),
            Stmt::Function(function) => visitor.visit_function_stmt(function),
            Stmt::Return(return_stmt) => visitor.visit_return_stmt(return_stmt),
            Stmt::Class(class) => visitor.visit_class_stmt(class),
        }
    }
}

// Statement Types
#[derive(Debug, Clone)]
pub struct Expression<'src> {
    pub expression: Expr<'src>,
}
impl<'src> Expression<'src> {
    pub fn new(expression: Expr<'src>) -> Self {
        Expression { expression }
    }
}

#[derive(Debug, Clone)]
pub struct Print<'src> {
    pub expression: Expr<'src>,
}
impl<'src> Print<'src> {
    pub fn new(expression: Expr<'src>) -> Self {
        Print { expression }
    }
}

#[derive(Debug, Clone)]
pub struct Var<'src> {
    pub name: Token<'src>,
    pub initializer: Option<Expr<'src>>,
}
impl<'src> Var<'src> {
    pub fn new(name: Token<'src>, initializer: Option<Expr<'src>>) -> Self {
        Var { name, initializer }
    }
}

#[derive(Debug, Clone)]
pub struct Block<'src> {
    pub statements: Vec<Stmt<'src>>,
}
impl<'src> Block<'src> {
    pub fn new(statements: Vec<Stmt<'src>>) -> Self {
        Block { statements }
    }
}

#[derive(Debug, Clone)]
pub struct If<'src> {
    pub condition: Expr<'src>,
    pub then_branch: Box<Stmt<'src>>,
    pub else_branch: Option<Box<Stmt<'src>>>,
}
impl<'src> If<'src> {
    pub fn new(
        condition: Expr<'src>,
        then_branch: Box<Stmt<'src>>,
        else_branch: Option<Box<Stmt<'src>>>,
    ) -> Self {
        If {
            condition,
            then_branch,
            else_branch,
        }
    }
}

#[derive(Debug, Clone)]
pub struct While<'src> {
    pub condition: Expr<'src>,
    pub body: Box<Stmt<'src>>,
}
impl<'src> While<'src> {
    pub fn new(condition: Expr<'src>, body: Box<Stmt<'src>>) -> Self {
        While { condition, body }
    }
}

#[derive(Debug, Clone)]
pub struct Function<'src> {
    pub name: Token<'src>,
    pub closure: Closure<'src>,
}
impl<'src> Function<'src> {
    pub fn new(name: Token<'src>, closure: Closure<'src>) -> Self {
        Function { name, closure }
    }
}
impl<'src> PartialEq for Function<'src> {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, Clone)]
pub struct Return<'src> {
    pub keyword: Token<'src>,
    pub value: Expr<'src>,
}
impl<'src> Return<'src> {
    pub fn new(keyword: Token<'src>, value: Expr<'src>) -> Self {
        Return { keyword, value }
    }
}

#[derive(Debug, Clone)]
pub struct Class<'src> {
    pub name: Token<'src>,
    pub superclass: Option<Box<Expr<'src>>>,
    pub methods: Vec<Stmt<'src>>,
}
impl<'src> Class<'src> {
    pub fn new(
        name: Token<'src>,
        superclass: Option<Box<Expr<'src>>>,
        methods: Vec<Stmt<'src>>,
    ) -> Self {
        Class {
            name,
            superclass,
            methods,
        }
    }
}
