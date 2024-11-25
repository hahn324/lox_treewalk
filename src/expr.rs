use crate::token::{LoxLiteral, Token};

pub trait ExprVisitor {
    fn visit_binary_expr(&mut self, expr: &Binary) -> LoxLiteral;
    fn visit_grouping_expr(&mut self, expr: &Grouping) -> LoxLiteral;
    fn visit_literal_expr(&mut self, expr: &Literal) -> LoxLiteral;
    fn visit_unary_expr(&mut self, expr: &Unary) -> LoxLiteral;
    fn visit_ternary_expr(&mut self, expr: &Ternary) -> LoxLiteral;
    fn visit_variable_expr(&mut self, expr: &Variable) -> LoxLiteral;
    fn visit_assign_expr(&mut self, expr: &Assign) -> LoxLiteral;
    fn visit_logical_expr(&mut self, expr: &Logical) -> LoxLiteral;
}

pub trait Expr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> LoxLiteral;

    fn get_assignment_target(&self) -> Option<&Token> {
        None
    }
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
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> LoxLiteral {
        visitor.visit_binary_expr(self)
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
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> LoxLiteral {
        visitor.visit_grouping_expr(self)
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
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> LoxLiteral {
        visitor.visit_literal_expr(self)
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
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> LoxLiteral {
        visitor.visit_unary_expr(self)
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
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> LoxLiteral {
        visitor.visit_ternary_expr(self)
    }
}

pub struct Variable {
    pub name: Token,
}
impl Variable {
    pub fn new(name: Token) -> Self {
        Variable { name }
    }
}
impl Expr for Variable {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> LoxLiteral {
        visitor.visit_variable_expr(self)
    }

    fn get_assignment_target(&self) -> Option<&Token> {
        Some(&self.name)
    }
}

pub struct Assign {
    pub name: Token,
    pub value: Box<dyn Expr>,
}
impl Assign {
    pub fn new(name: Token, value: Box<dyn Expr>) -> Self {
        Assign { name, value }
    }
}
impl Expr for Assign {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> LoxLiteral {
        visitor.visit_assign_expr(self)
    }
}

pub struct Logical {
    pub left: Box<dyn Expr>,
    pub operator: Token,
    pub right: Box<dyn Expr>,
}
impl Logical {
    pub fn new(left: Box<dyn Expr>, operator: Token, right: Box<dyn Expr>) -> Self {
        Logical {
            left,
            operator,
            right,
        }
    }
}
impl Expr for Logical {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> LoxLiteral {
        visitor.visit_logical_expr(self)
    }
}
