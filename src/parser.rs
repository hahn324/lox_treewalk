use std::iter::Peekable;
use std::vec::IntoIter;

use crate::expr::{Assign, Binary, Expr, Grouping, Literal, Logical, Ternary, Unary, Variable};
use crate::report;
use crate::stmt::{Block, Break, Expression, If, Print, Stmt, Var, While};
use crate::token::{LoxLiteral, Token};
use crate::token_type::TokenType;

#[derive(Debug)]
pub struct LoxParseError;

pub struct Parser {
    token_iter: Peekable<IntoIter<Token>>,
    had_error: bool,
    loop_level: u32,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            token_iter: tokens.into_iter().peekable(),
            had_error: false,
            loop_level: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Box<dyn Stmt>>, LoxParseError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }

        match self.had_error {
            true => Err(LoxParseError),
            false => Ok(statements),
        }
    }

    fn declaration(&mut self) -> Option<Box<dyn Stmt>> {
        let res = match self.peek_token_type() {
            TokenType::Var => {
                // Consume the Var token.
                self.advance();
                self.var_declaration()
            }
            _ => self.statement(),
        };
        match res {
            Ok(stmt) => Some(stmt),
            Err(_) => {
                self.had_error = true;
                self.synchronize();
                None
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Box<dyn Stmt>, LoxParseError> {
        let name = self.consume(TokenType::Identifier, "Expect variable name.")?;

        let mut initializer = None;
        if self.check(&TokenType::Equal) {
            // Consume the Equal token.
            self.advance();
            initializer = Some(self.expression()?);
        }

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Box::new(Var::new(name, initializer)))
    }

    fn statement(&mut self) -> Result<Box<dyn Stmt>, LoxParseError> {
        let token_types = vec![
            TokenType::Print,
            TokenType::LeftBrace,
            TokenType::If,
            TokenType::While,
            TokenType::For,
            TokenType::Break,
        ];
        if let Some(statement_token) = self.match_token_type(&token_types) {
            match statement_token.token_type {
                TokenType::Print => self.print_statement(),
                TokenType::LeftBrace => Ok(Box::new(Block::new(self.block()?))),
                TokenType::If => self.if_statement(),
                TokenType::While => self.while_statement(),
                TokenType::For => self.for_statement(),
                TokenType::Break => self.break_statement(),
                _ => unreachable!("Above match_token_type guarentees that no other token types are possible here."),
            }
        } else {
            self.expression_statement()
        }
    }

    fn for_statement(&mut self) -> Result<Box<dyn Stmt>, LoxParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer_option = if self.check(&TokenType::Semicolon) {
            // Consume Semicolon token.
            self.advance();
            None
        } else if self.check(&TokenType::Var) {
            // Consume Var token.
            self.advance();
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let condition = match self.check(&TokenType::Semicolon) {
            true => Box::new(Literal::new(LoxLiteral::Boolean(true))),
            false => self.expression()?,
        };
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let increment_option = match self.check(&TokenType::RightParen) {
            true => None,
            false => Some(self.expression()?),
        };
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        self.loop_level += 1;
        let mut body = self.statement()?;
        self.loop_level -= 1;

        if let Some(increment) = increment_option {
            body = Box::new(Block::new(vec![body, Box::new(Expression::new(increment))]));
        }

        body = Box::new(While::new(condition, body));

        if let Some(initializer) = initializer_option {
            body = Box::new(Block::new(vec![initializer, body]));
        }

        Ok(body)
    }

    fn while_statement(&mut self) -> Result<Box<dyn Stmt>, LoxParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;

        self.loop_level += 1;
        let body = self.statement()?;
        self.loop_level -= 1;

        Ok(Box::new(While::new(condition, body)))
    }

    fn break_statement(&mut self) -> Result<Box<dyn Stmt>, LoxParseError> {
        let stmt_end = self.consume(TokenType::Semicolon, "Expect ';' after 'break' statement.")?;
        if self.loop_level == 0 {
            report(
                stmt_end.line,
                "at 'break;'",
                "A 'break;' cannot appear outside of any enclosing loop.",
            );
            self.had_error = true;
        }
        Ok(Box::new(Break))
    }

    fn print_statement(&mut self) -> Result<Box<dyn Stmt>, LoxParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Box::new(Print::new(value)))
    }

    fn block(&mut self) -> Result<Vec<Box<dyn Stmt>>, LoxParseError> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(statements)
    }

    fn if_statement(&mut self) -> Result<Box<dyn Stmt>, LoxParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after 'if' condition.")?;

        let then_branch = self.statement()?;
        let else_branch = match self.check(&TokenType::Else) {
            true => {
                // Consume the Else token.
                self.advance();
                Some(self.statement()?)
            }
            false => None,
        };
        Ok(Box::new(If::new(condition, then_branch, else_branch)))
    }

    fn expression_statement(&mut self) -> Result<Box<dyn Stmt>, LoxParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Box::new(Expression::new(expr)))
    }

    fn expression(&mut self) -> Result<Box<dyn Expr>, LoxParseError> {
        self.comma()
    }

    fn comma(&mut self) -> Result<Box<dyn Expr>, LoxParseError> {
        let mut expr = self.assignment()?;

        let token_types = vec![TokenType::Comma];
        while let Some(operator) = self.match_token_type(&token_types) {
            let right = self.assignment()?;
            expr = Box::new(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn assignment(&mut self) -> Result<Box<dyn Expr>, LoxParseError> {
        let expr = self.ternary()?;

        if self.check(&TokenType::Equal) {
            let equals = self.advance().unwrap();
            let value = self.assignment()?;

            match expr.get_assignment_target() {
                Some(target) => {
                    return Ok(Box::new(Assign::new(target.clone(), value)));
                }
                None => {
                    report(
                        equals.line,
                        &format!("at '{}'", equals.lexeme),
                        "Invalid assignment target.",
                    );
                }
            }
        }

        Ok(expr)
    }

    fn ternary(&mut self) -> Result<Box<dyn Expr>, LoxParseError> {
        let mut expr = self.or()?;

        if self.check(&TokenType::QuestionMark) {
            // Consume the QuestionMark Token.
            self.advance();
            let left = self.ternary()?;
            self.consume(
                TokenType::Colon,
                "Expect ':' to separate two expressions after '?'",
            )?;
            let right = self.ternary()?;
            expr = Box::new(Ternary::new(expr, left, right));
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Box<dyn Expr>, LoxParseError> {
        let mut expr = self.and()?;

        let token_types = vec![TokenType::Or];
        while let Some(operator) = self.match_token_type(&token_types) {
            let right = self.and()?;
            expr = Box::new(Logical::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Box<dyn Expr>, LoxParseError> {
        let mut expr = self.equality()?;

        let token_types = vec![TokenType::And];
        while let Some(operator) = self.match_token_type(&token_types) {
            let right = self.equality()?;
            expr = Box::new(Logical::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Box<dyn Expr>, LoxParseError> {
        let mut expr = self.comparison()?;

        let token_types = vec![TokenType::BangEqual, TokenType::EqualEqual];
        while let Some(operator) = self.match_token_type(&token_types) {
            let right = self.comparison()?;
            expr = Box::new(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<dyn Expr>, LoxParseError> {
        let mut expr = self.term()?;

        let token_types = vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];
        while let Some(operator) = self.match_token_type(&token_types) {
            let right = self.term()?;
            expr = Box::new(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<dyn Expr>, LoxParseError> {
        let mut expr = self.factor()?;

        let token_types = vec![TokenType::Minus, TokenType::Plus];
        while let Some(operator) = self.match_token_type(&token_types) {
            let right = self.factor()?;
            expr = Box::new(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<dyn Expr>, LoxParseError> {
        let mut expr = self.binary_operator_error()?;

        let token_types = vec![TokenType::Slash, TokenType::Star];
        while let Some(operator) = self.match_token_type(&token_types) {
            let right = self.binary_operator_error()?;
            expr = Box::new(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn binary_operator_error(&mut self) -> Result<Box<dyn Expr>, LoxParseError> {
        let token_types = vec![
            TokenType::Comma,
            TokenType::BangEqual,
            TokenType::EqualEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
            TokenType::Plus,
            TokenType::Slash,
            TokenType::Star,
        ];
        if let Some(operator) = self.match_token_type(&token_types) {
            // Consume expression after invalid binary operator.
            let _ = match operator.token_type {
                TokenType::Comma => self.equality(),
                TokenType::BangEqual | TokenType::EqualEqual => self.comparison(),
                TokenType::Greater | TokenType::GreaterEqual | TokenType::Less | TokenType::LessEqual => self.term(),
                TokenType::Plus => self.factor(),
                TokenType::Slash | TokenType::Star => self.unary(),
                _ => unreachable!("Above match_token_type guarentees that no other token types are possible here."),
            };
            report(
                operator.line,
                &format!("at '{}'", operator.lexeme),
                "Invalid use of binary operator, must be preceded by an expression.",
            );
            Err(LoxParseError)
        } else {
            self.unary()
        }
    }

    fn unary(&mut self) -> Result<Box<dyn Expr>, LoxParseError> {
        let token_types = vec![TokenType::Bang, TokenType::Minus];
        if let Some(operator) = self.match_token_type(&token_types) {
            let right = self.unary()?;
            return Ok(Box::new(Unary::new(operator, right)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Box<dyn Expr>, LoxParseError> {
        let literal_token_types = vec![
            TokenType::False,
            TokenType::True,
            TokenType::Nil,
            TokenType::Number,
            TokenType::String,
        ];
        if let Some(token) = self.match_token_type(&literal_token_types) {
            return Ok(Box::new(Literal::new(token.literal.unwrap())));
        }

        if self.check(&TokenType::Identifier) {
            return Ok(Box::new(Variable::new(self.advance().unwrap())));
        }

        if self.check(&TokenType::LeftParen) {
            // Consume the LeftParen token.
            self.advance();
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            return Ok(Box::new(Grouping::new(expr)));
        }

        // Will always be Some variant from peek since we never consume the last Eof token.
        let next_token = self.token_iter.peek().unwrap();
        report(
            next_token.line,
            &format!("at '{}'", next_token.lexeme),
            "Failed to match a valid expression literal.",
        );
        Err(LoxParseError)
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, LoxParseError> {
        match self.check(&token_type) {
            // Will always be Some variant in true arm of this match.
            true => Ok(self.advance().unwrap()),
            false => {
                // Will always be Some variant from peek since we never consume the last Eof token.
                let next_token = self.token_iter.peek().unwrap();
                match next_token.token_type {
                    TokenType::Eof => report(next_token.line, " at end", message),
                    _ => report(
                        next_token.line,
                        &format!("at '{}'", next_token.lexeme),
                        message,
                    ),
                }
                Err(LoxParseError)
            }
        }
    }

    fn match_token_type(&mut self, token_types: &Vec<TokenType>) -> Option<Token> {
        for token_type in token_types {
            if self.check(token_type) {
                return self.advance();
            }
        }
        None
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        match self.is_at_end() {
            true => false,
            false => self.peek_token_type() == *token_type,
        }
    }

    fn advance(&mut self) -> Option<Token> {
        match self.is_at_end() {
            true => None,
            false => self.token_iter.next(),
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.peek_token_type() == TokenType::Eof
    }

    fn peek_token_type(&mut self) -> TokenType {
        self.token_iter
            .peek()
            .expect("Parser should never be able to consume Eof token and reach end of iteration.")
            .token_type
    }

    fn synchronize(&mut self) {
        while let Some(token) = self.advance() {
            if token.token_type == TokenType::Semicolon {
                break;
            }

            match self.peek_token_type() {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    break;
                }
                _ => (),
            }
        }
    }
}
