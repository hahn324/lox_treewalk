use crate::{lox_object::LoxLiteral, stmt::Stmt, token::Token};

pub trait ExprVisitor<'src, T> {
    fn visit_binary_expr(&mut self, expr: &Binary<'src>) -> T;
    fn visit_grouping_expr(&mut self, expr: &Grouping<'src>) -> T;
    fn visit_literal_expr(&mut self, expr: &Literal) -> T;
    fn visit_unary_expr(&mut self, expr: &Unary<'src>) -> T;
    fn visit_ternary_expr(&mut self, expr: &Ternary<'src>) -> T;
    fn visit_variable_expr(&mut self, expr: &Variable<'src>) -> T;
    fn visit_assign_expr(&mut self, expr: &Assign<'src>) -> T;
    fn visit_logical_expr(&mut self, expr: &Logical<'src>) -> T;
    fn visit_call_expr(&mut self, expr: &Call<'src>) -> T;
    fn visit_closure_expr(&mut self, expr: &Closure<'src>) -> T;
    fn visit_get_expr(&mut self, expr: &Get<'src>) -> T;
    fn visit_set_expr(&mut self, expr: &Set<'src>) -> T;
    fn visit_this_expr(&mut self, expr: &This<'src>) -> T;
    fn visit_super_expr(&mut self, expr: &Super<'src>) -> T;
}

#[derive(Debug, Clone)]
pub enum Expr<'src> {
    Binary(Binary<'src>),
    Grouping(Grouping<'src>),
    Literal(Literal),
    Unary(Unary<'src>),
    Ternary(Ternary<'src>),
    Variable(Variable<'src>),
    Assign(Assign<'src>),
    Logical(Logical<'src>),
    Call(Call<'src>),
    Closure(Closure<'src>),
    Get(Get<'src>),
    Set(Set<'src>),
    This(This<'src>),
    Super(Super<'src>),
}

impl<'src> Expr<'src> {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<'src, T>) -> T {
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
pub struct Binary<'src> {
    pub left: Box<Expr<'src>>,
    pub operator: Token<'src>,
    pub right: Box<Expr<'src>>,
}
impl<'src> Binary<'src> {
    pub fn new(left: Box<Expr<'src>>, operator: Token<'src>, right: Box<Expr<'src>>) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grouping<'src> {
    pub expression: Box<Expr<'src>>,
}
impl<'src> Grouping<'src> {
    pub fn new(expression: Box<Expr<'src>>) -> Self {
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
pub struct Unary<'src> {
    pub operator: Token<'src>,
    pub right: Box<Expr<'src>>,
}
impl<'src> Unary<'src> {
    pub fn new(operator: Token<'src>, right: Box<Expr<'src>>) -> Self {
        Unary { operator, right }
    }
}

#[derive(Debug, Clone)]
pub struct Ternary<'src> {
    pub condition: Box<Expr<'src>>,
    pub left: Box<Expr<'src>>,
    pub right: Box<Expr<'src>>,
}
impl<'src> Ternary<'src> {
    pub fn new(condition: Box<Expr<'src>>, left: Box<Expr<'src>>, right: Box<Expr<'src>>) -> Self {
        Ternary {
            condition,
            left,
            right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Variable<'src> {
    pub name: Token<'src>,
}
impl<'src> Variable<'src> {
    pub fn new(name: Token<'src>) -> Self {
        Variable { name }
    }
}

#[derive(Debug, Clone)]
pub struct Assign<'src> {
    pub name: Token<'src>,
    pub value: Box<Expr<'src>>,
}
impl<'src> Assign<'src> {
    pub fn new(name: Token<'src>, value: Box<Expr<'src>>) -> Self {
        Assign { name, value }
    }
}

#[derive(Debug, Clone)]
pub struct Logical<'src> {
    pub left: Box<Expr<'src>>,
    pub operator: Token<'src>,
    pub right: Box<Expr<'src>>,
}
impl<'src> Logical<'src> {
    pub fn new(left: Box<Expr<'src>>, operator: Token<'src>, right: Box<Expr<'src>>) -> Self {
        Logical {
            left,
            operator,
            right,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Call<'src> {
    pub callee: Box<Expr<'src>>,
    pub paren: Token<'src>,
    pub arguments: Vec<Expr<'src>>,
}
impl<'src> Call<'src> {
    pub fn new(callee: Box<Expr<'src>>, paren: Token<'src>, arguments: Vec<Expr<'src>>) -> Self {
        Call {
            callee,
            paren,
            arguments,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Closure<'src> {
    pub params: Vec<Token<'src>>,
    pub body: Vec<Stmt<'src>>,
}
impl<'src> Closure<'src> {
    pub fn new(params: Vec<Token<'src>>, body: Vec<Stmt<'src>>) -> Self {
        Closure { params, body }
    }
}
impl<'src> PartialEq for Closure<'src> {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub struct Get<'src> {
    pub object: Box<Expr<'src>>,
    pub name: Token<'src>,
}
impl<'src> Get<'src> {
    pub fn new(object: Box<Expr<'src>>, name: Token<'src>) -> Self {
        Get { object, name }
    }
}

#[derive(Debug, Clone)]
pub struct Set<'src> {
    pub object: Box<Expr<'src>>,
    pub name: Token<'src>,
    pub value: Box<Expr<'src>>,
}
impl<'src> Set<'src> {
    pub fn new(object: Box<Expr<'src>>, name: Token<'src>, value: Box<Expr<'src>>) -> Self {
        Set {
            object,
            name,
            value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct This<'src> {
    pub keyword: Token<'src>,
}
impl<'src> This<'src> {
    pub fn new(keyword: Token<'src>) -> Self {
        This { keyword }
    }
}

#[derive(Debug, Clone)]
pub struct Super<'src> {
    pub keyword: Token<'src>,
    pub method: Token<'src>,
}
impl<'src> Super<'src> {
    pub fn new(keyword: Token<'src>, method: Token<'src>) -> Self {
        Super { keyword, method }
    }
}
