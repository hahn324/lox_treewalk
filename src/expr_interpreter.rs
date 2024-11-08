use crate::token::{LoxLiteral, Token};

pub trait Expr {
    fn ast_print(&self) -> String;
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
    fn ast_print(&self) -> String {
        let mut print_result = String::new();
        print_result.push('(');
        print_result.push_str(&self.operator.lexeme);
        print_result.push(' ');
        print_result.push_str(&self.left.ast_print());
        print_result.push(' ');
        print_result.push_str(&self.right.ast_print());
        print_result.push(')');
        print_result
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
    fn ast_print(&self) -> String {
        let mut print_result = String::new();
        print_result.push('(');
        print_result.push_str("group");
        print_result.push(' ');
        print_result.push_str(&self.expression.ast_print());
        print_result.push(')');
        print_result
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
    fn ast_print(&self) -> String {
        match self.value {
            LoxLiteral::Number(num) => num.to_string(),
            LoxLiteral::String(ref some_string) => some_string.clone(),
            LoxLiteral::Boolean(val) => val.to_string(),
            LoxLiteral::Nil => "nil".to_string(),
        }
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
    fn ast_print(&self) -> String {
        let mut print_result = String::new();
        print_result.push('(');
        print_result.push_str(&self.operator.lexeme);
        print_result.push(' ');
        print_result.push_str(&self.right.ast_print());
        print_result.push(')');
        print_result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token_type::TokenType;

    #[test]
    fn test_ast_print() {
        let expr = Binary::new(
            Box::new(Unary::new(
                Token::new(TokenType::Minus, String::from("-"), None, 1),
                Box::new(Literal::new(LoxLiteral::Number(123.0))),
            )),
            Token::new(TokenType::Star, String::from("*"), None, 1),
            Box::new(Grouping::new(Box::new(Literal::new(LoxLiteral::Number(
                45.67,
            ))))),
        );
        assert_eq!(expr.ast_print(), "(* (- 123) (group 45.67))".to_string());
    }
}
