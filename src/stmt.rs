use crate::{
    expr::{Closure, Expr},
    token::Token,
};

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> T;
    fn visit_print_stmt(&mut self, stmt: &Print) -> T;
    fn visit_var_stmt(&mut self, stmt: &Var) -> T;
    fn visit_block_stmt(&mut self, stmt: &Block) -> T;
    fn visit_if_stmt(&mut self, stmt: &If) -> T;
    fn visit_while_stmt(&mut self, stmt: &While) -> T;
    fn visit_break_stmt(&mut self) -> T;
    fn visit_function_stmt(&mut self, stmt: &Function) -> T;
    fn visit_return_stmt(&mut self, stmt: &Return) -> T;
    fn visit_class_stmt(&mut self, stmt: &Class) -> T;
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expression),
    Print(Print),
    Var(Var),
    Block(Block),
    If(If),
    While(While),
    Break,
    Function(Function),
    Return(Return),
    Class(Class),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
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
pub struct Expression {
    pub expression: Expr,
}
impl Expression {
    pub fn new(expression: Expr) -> Self {
        Expression { expression }
    }
}

#[derive(Debug, Clone)]
pub struct Print {
    pub expression: Expr,
}
impl Print {
    pub fn new(expression: Expr) -> Self {
        Print { expression }
    }
}

#[derive(Debug, Clone)]
pub struct Var {
    pub name: Token,
    pub initializer: Option<Expr>,
}
impl Var {
    pub fn new(name: Token, initializer: Option<Expr>) -> Self {
        Var { name, initializer }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}
impl Block {
    pub fn new(statements: Vec<Stmt>) -> Self {
        Block { statements }
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}
impl If {
    pub fn new(condition: Expr, then_branch: Box<Stmt>, else_branch: Option<Box<Stmt>>) -> Self {
        If {
            condition,
            then_branch,
            else_branch,
        }
    }
}

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Expr,
    pub body: Box<Stmt>,
}
impl While {
    pub fn new(condition: Expr, body: Box<Stmt>) -> Self {
        While { condition, body }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Token,
    pub closure: Closure,
}
impl Function {
    pub fn new(name: Token, closure: Closure) -> Self {
        Function { name, closure }
    }
}
impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, Clone)]
pub struct Return {
    pub keyword: Token,
    pub value: Expr,
}
impl Return {
    pub fn new(keyword: Token, value: Expr) -> Self {
        Return { keyword, value }
    }
}

#[derive(Debug, Clone)]
pub struct Class {
    pub name: Token,
    pub methods: Vec<Stmt>,
}
impl Class {
    pub fn new(name: Token, methods: Vec<Stmt>) -> Self {
        Class { name, methods }
    }
}
