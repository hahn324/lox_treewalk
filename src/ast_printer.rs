use crate::expr::{Assign, Binary, Expr, ExprVisitor, Grouping, Literal, Logical, Ternary, Unary};
use crate::stmt::{Block, Expression, If, Print, StmtVisitor, Var, While};
use crate::token::LoxLiteral;

pub struct AstPrinter;

#[allow(dead_code)]
impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter
    }

    pub fn print(&mut self, expression: &Box<dyn Expr>) -> String {
        if let LoxLiteral::String(output) = expression.accept(self) {
            output
        } else {
            String::from("")
        }
    }

    fn parenthesize(&mut self, name: &str, exprs: Vec<&Box<dyn Expr>>) -> LoxLiteral {
        let mut output = String::new();
        output.push('(');
        output.push_str(name);
        for expr in exprs {
            output.push(' ');
            if let LoxLiteral::String(val) = expr.accept(self) {
                output.push_str(&val);
            }
        }
        output.push(')');
        LoxLiteral::String(output)
    }
}
#[allow(unused_variables)]
impl ExprVisitor for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &Binary) -> LoxLiteral {
        self.parenthesize(&expr.operator.lexeme, vec![&expr.left, &expr.right])
    }

    fn visit_grouping_expr(&mut self, expr: &Grouping) -> LoxLiteral {
        self.parenthesize("group", vec![&expr.expression])
    }

    fn visit_literal_expr(&mut self, expr: &Literal) -> LoxLiteral {
        LoxLiteral::String(expr.value.stringify())
    }

    fn visit_unary_expr(&mut self, expr: &Unary) -> LoxLiteral {
        self.parenthesize(&expr.operator.lexeme, vec![&expr.right])
    }

    fn visit_ternary_expr(&mut self, expr: &Ternary) -> LoxLiteral {
        self.parenthesize("ternary", vec![&expr.condition, &expr.left, &expr.right])
    }

    fn visit_variable_expr(&mut self, expr: &crate::expr::Variable) -> LoxLiteral {
        todo!();
    }

    fn visit_assign_expr(&mut self, expr: &Assign) -> LoxLiteral {
        todo!();
    }

    fn visit_logical_expr(&mut self, expr: &Logical) -> LoxLiteral {
        todo!();
    }
}

#[allow(unused_variables)]
impl StmtVisitor for AstPrinter {
    fn visit_print_stmt(&mut self, stmt: &Print) {
        todo!();
    }

    fn visit_expression_stmt(&mut self, stmt: &Expression) {
        todo!();
    }

    fn visit_var_stmt(&mut self, stmt: &Var) {
        todo!();
    }

    fn visit_block_stmt(&mut self, stmt: &Block) {
        todo!();
    }

    fn visit_if_stmt(&mut self, stmt: &If) {
        todo!();
    }

    fn visit_while_stmt(&mut self, stmt: &While) {
        todo!();
    }

    fn visit_break_stmt(&mut self) {
        todo!();
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
        let expr: Box<dyn Expr> = Box::new(Binary::new(
            Box::new(Unary::new(
                Token::new(TokenType::Minus, String::from("-"), None, 1),
                Box::new(Literal::new(LoxLiteral::Number(123.0))),
            )),
            Token::new(TokenType::Star, String::from("*"), None, 1),
            Box::new(Grouping::new(Box::new(Literal::new(LoxLiteral::Number(
                45.67,
            ))))),
        ));
        assert_eq!(ast_printer.print(&expr), "(* (- 123) (group 45.67))");
    }

    #[test]
    fn test_ast_print_ternary() {
        let mut ast_printer = AstPrinter::new();
        let expr: Box<dyn Expr> = Box::new(Ternary::new(
            Box::new(Literal::new(LoxLiteral::Boolean(true))),
            Box::new(Literal::new(LoxLiteral::Number(1.0))),
            Box::new(Literal::new(LoxLiteral::Number(2.0))),
        ));
        assert_eq!(ast_printer.print(&expr), "(ternary true 1 2)");
    }
}
