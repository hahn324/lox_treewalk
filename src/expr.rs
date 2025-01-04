use crate::{lox_object::LoxLiteral, stmt::Stmt, token::Token};

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
    fn visit_closure_expr(&mut self, expr: &Closure) -> T;
    fn visit_get_expr(&mut self, expr: &Get) -> T;
    fn visit_set_expr(&mut self, expr: &Set) -> T;
    fn visit_this_expr(&mut self, expr: &This) -> T;
    fn visit_super_expr(&mut self, expr: &Super) -> T;
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
    Closure(Closure),
    Get(Get),
    Set(Set),
    This(This),
    Super(Super),
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
            Expr::Closure(closure) => visitor.visit_closure_expr(closure),
            Expr::Get(get) => visitor.visit_get_expr(get),
            Expr::Set(set) => visitor.visit_set_expr(set),
            Expr::This(this) => visitor.visit_this_expr(this),
            Expr::Super(super_expr) => visitor.visit_super_expr(super_expr),
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

#[derive(Debug, Clone)]
pub struct Closure {
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}
impl Closure {
    pub fn new(params: Vec<Token>, body: Vec<Stmt>) -> Self {
        Closure { params, body }
    }
}
impl PartialEq for Closure {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub struct Get {
    pub object: Box<Expr>,
    pub name: Token,
}
impl Get {
    pub fn new(object: Box<Expr>, name: Token) -> Self {
        Get { object, name }
    }
}

#[derive(Debug, Clone)]
pub struct Set {
    pub object: Box<Expr>,
    pub name: Token,
    pub value: Box<Expr>,
}
impl Set {
    pub fn new(object: Box<Expr>, name: Token, value: Box<Expr>) -> Self {
        Set {
            object,
            name,
            value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct This {
    pub keyword: Token,
}
impl This {
    pub fn new(keyword: Token) -> Self {
        This { keyword }
    }
}

#[derive(Debug, Clone)]
pub struct Super {
    pub keyword: Token,
    pub method: Token,
}
impl Super {
    pub fn new(keyword: Token, method: Token) -> Self {
        Super { keyword, method }
    }
}
