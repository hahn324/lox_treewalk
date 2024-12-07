use crate::lox_object::LoxLiteral;
use crate::token::Token;

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, expr: &Binary) -> T;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> T;
    fn visit_literal_expr(&mut self, expr: &Literal) -> T;
    fn visit_unary_expr(&mut self, expr: &Unary) -> T;
    fn visit_ternary_expr(&mut self, expr: &Ternary) -> T;
    fn visit_variable_expr(&mut self, expr: &Variable) -> T;
    fn visit_assign_expr(&mut self, expr: &Assign) -> T;
    fn visit_logical_expr(&mut self, expr: &Logical) -> T;
    fn visit_call_expr(&mut self, expr: &Call) -> T;
}

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
    Ternary(Ternary),
    Variable(Variable),
    Assign(Assign),
    Logical(Logical),
    Call(Call),
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Binary(binary) => visitor.visit_binary_expr(binary),
            Expr::Grouping(grouping) => visitor.visit_grouping_expr(grouping),
            Expr::Literal(literal) => visitor.visit_literal_expr(literal),
            Expr::Unary(unary) => visitor.visit_unary_expr(unary),
            Expr::Ternary(ternary) => visitor.visit_ternary_expr(ternary),
            Expr::Variable(variable) => visitor.visit_variable_expr(variable),
            Expr::Assign(assign) => visitor.visit_assign_expr(assign),
            Expr::Logical(logical) => visitor.visit_logical_expr(logical),
            Expr::Call(call) => visitor.visit_call_expr(call),
        }
    }
}

// Expression Types
#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
impl Binary {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Box<Expr>,
}
impl Grouping {
    pub fn new(expression: Box<Expr>) -> Self {
        Grouping { expression }
    }
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub value: LoxLiteral,
}
impl Literal {
    pub fn new(value: LoxLiteral) -> Self {
        Literal { value }
    }
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}
impl Unary {
    pub fn new(operator: Token, right: Box<Expr>) -> Self {
        Unary { operator, right }
    }
}

#[derive(Debug, Clone)]
pub struct Ternary {
    pub condition: Box<Expr>,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}
impl Ternary {
    pub fn new(condition: Box<Expr>, left: Box<Expr>, right: Box<Expr>) -> Self {
        Ternary {
            condition,
            left,
            right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Token,
}
impl Variable {
    pub fn new(name: Token) -> Self {
        Variable { name }
    }
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: Token,
    pub value: Box<Expr>,
}
impl Assign {
    pub fn new(name: Token, value: Box<Expr>) -> Self {
        Assign { name, value }
    }
}

#[derive(Debug, Clone)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}
impl Logical {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
        Logical {
            left,
            operator,
            right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Call {
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}
impl Call {
    pub fn new(callee: Box<Expr>, paren: Token, arguments: Vec<Expr>) -> Self {
        Call {
            callee,
            paren,
            arguments,
        }
    }
}
