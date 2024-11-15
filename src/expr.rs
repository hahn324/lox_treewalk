use crate::token::{LoxLiteral, Token};

pub trait Visitor {
    fn visit_binary_expr(&mut self, expr: &Binary);
    fn visit_grouping_expr(&mut self, expr: &Grouping);
    fn visit_literal_expr(&mut self, expr: &Literal);
    fn visit_unary_expr(&mut self, expr: &Unary);
    fn visit_ternary_expr(&mut self, expr: &Ternary);
}

pub trait Expr {
    fn accept(&self, visitor: &mut dyn Visitor);
}

pub struct Binary {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}
impl Binary {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }
}
impl Expr for Binary {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_binary_expr(self);
    }
}

pub struct Grouping {
    pub expression: Box<dyn Expr>,
}
impl Grouping {
    pub fn new(expression: Box<dyn Expr>) -> Self {
        Grouping { expression }
    }
}
impl Expr for Grouping {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_grouping_expr(self);
    }
}

pub struct Literal {
    pub value: LoxLiteral,
}
impl Literal {
    pub fn new(value: LoxLiteral) -> Self {
        Literal { value }
    }
}
impl Expr for Literal {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_literal_expr(self);
    }
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<dyn Expr>,
}
impl Unary {
    pub fn new(operator: Token, right: Box<dyn Expr>) -> Self {
        Unary { operator, right }
    }
}
impl Expr for Unary {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_unary_expr(self);
    }
}

pub struct Ternary {
    pub condition: Box<dyn Expr>,
    pub left: Box<dyn Expr>,
    pub right: Box<dyn Expr>,
}
impl Ternary {
    pub fn new(condition: Box<dyn Expr>, left: Box<dyn Expr>, right: Box<dyn Expr>) -> Self {
        Ternary {
            condition,
            left,
            right,
        }
    }
}
impl Expr for Ternary {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_ternary_expr(self);
    }
}
