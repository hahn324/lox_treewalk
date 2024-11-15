use std::iter::Peekable;
use std::vec::IntoIter;

use crate::expr::{Binary, Expr, Grouping, Literal, Ternary, Unary};
use crate::report;
use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug)]
pub struct LoxParseError;

pub struct Parser {
    token_iter: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            token_iter: tokens.into_iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Box<dyn Expr>, LoxParseError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Box<dyn Expr>, LoxParseError> {
        self.comma()
    }

    fn comma(&mut self) -> Result<Box<dyn Expr>, LoxParseError> {
        let mut expr = self.ternary()?;

        let token_types = vec![TokenType::Comma];
        while let Some(operator) = self.match_token_type(&token_types) {
            let right = self.ternary()?;
            expr = Box::new(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn ternary(&mut self) -> Result<Box<dyn Expr>, LoxParseError> {
        let mut expr = self.equality()?;

        if let Some(_) = self.match_token_type(&vec![TokenType::QuestionMark]) {
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

        if let Some(_) = self.match_token_type(&vec![TokenType::LeftParen]) {
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
