use crate::expr::{Binary, Expr, Grouping, Literal, Ternary, Unary, Visitor};
use crate::token::LoxLiteral;

pub struct AstPrinter {
    pub output: String,
}
impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {
            output: String::new(),
        }
    }

    fn parenthesize(&mut self, name: &str, exprs: Vec<&Box<dyn Expr>>) {
        self.output.push('(');
        self.output.push_str(name);
        for expr in exprs {
            self.output.push(' ');
            expr.accept(self);
        }
        self.output.push(')');
    }
}
impl Visitor for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Binary) {
        self.parenthesize(&expr.operator.lexeme, vec![&expr.left, &expr.right]);
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) {
        self.parenthesize("group", vec![&expr.expression]);
    }

    fn visit_literal_expr(&mut self, expr: &Literal) {
        let print_val = match expr.value {
            LoxLiteral::Number(num) => &num.to_string(),
            LoxLiteral::String(ref some_string) => some_string,
            LoxLiteral::Boolean(val) => &val.to_string(),
            LoxLiteral::Nil => "nil",
        };
        self.output.push_str(print_val);
    }

    fn visit_unary_expr(&mut self, expr: &Unary) {
        self.parenthesize(&expr.operator.lexeme, vec![&expr.right]);
    }

    fn visit_ternary_expr(&mut self, expr: &Ternary) {
        self.parenthesize("ternary", vec![&expr.condition, &expr.left, &expr.right]);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::{LoxLiteral, Token};
    use crate::token_type::TokenType;

    #[test]
    fn test_ast_printer_visitor() {
        let mut ast_printer = AstPrinter::new();
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
        expr.accept(&mut ast_printer);
        assert_eq!(ast_printer.output, "(* (- 123) (group 45.67))".to_string());
    }

    #[test]
    fn test_ast_print_ternary() {
        let mut ast_printer = AstPrinter::new();
        let expr = Ternary::new(
            Box::new(Literal::new(LoxLiteral::Boolean(true))),
            Box::new(Literal::new(LoxLiteral::Number(1.0))),
            Box::new(Literal::new(LoxLiteral::Number(2.0))),
        );
        expr.accept(&mut ast_printer);
        assert_eq!(ast_printer.output, "(ternary true 1 2)".to_string());
    }
}
