use crate::lox_object::LoxLiteral;
use crate::report;
use crate::token::Token;
use crate::token_type::TokenType;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'a> {
    pub tokens: Vec<Token>,
    pub had_error: bool,
    source: &'a str,
    source_iter: Peekable<Chars<'a>>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
    next_token_id: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut keywords = HashMap::with_capacity(16);
        keywords.insert("and", TokenType::And);
        keywords.insert("class", TokenType::Class);
        keywords.insert("else", TokenType::Else);
        keywords.insert("false", TokenType::False);
        keywords.insert("for", TokenType::For);
        keywords.insert("fun", TokenType::Fun);
        keywords.insert("if", TokenType::If);
        keywords.insert("nil", TokenType::Nil);
        keywords.insert("or", TokenType::Or);
        keywords.insert("print", TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super", TokenType::Super);
        keywords.insert("this", TokenType::This);
        keywords.insert("true", TokenType::True);
        keywords.insert("var", TokenType::Var);
        keywords.insert("while", TokenType::While);
        keywords.insert("break", TokenType::Break);

        let source_iter: Peekable<Chars<'_>> = source.chars().peekable();

        Scanner {
            tokens: Vec::new(),
            had_error: false,
            source,
            source_iter,
            start: 0,
            current: 0,
            line: 1,
            keywords,
            next_token_id: 0,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            None,
            self.line,
            self.next_token_id,
        ));
        self.next_token_id += 1;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        match self.source_iter.next() {
            Some(return_char) => {
                self.current += return_char.len_utf8();
                return_char
            }
            None => '\0',
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            ':' => self.add_token(TokenType::Colon, None),
            '?' => self.add_token(TokenType::QuestionMark, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => match self.match_char('=') {
                true => self.add_token(TokenType::BangEqual, None),
                false => self.add_token(TokenType::Bang, None),
            },
            '=' => match self.match_char('=') {
                true => self.add_token(TokenType::EqualEqual, None),
                false => self.add_token(TokenType::Equal, None),
            },
            '<' => match self.match_char('=') {
                true => self.add_token(TokenType::LessEqual, None),
                false => self.add_token(TokenType::Less, None),
            },
            '>' => match self.match_char('=') {
                true => self.add_token(TokenType::GreaterEqual, None),
                false => self.add_token(TokenType::Greater, None),
            },
            '/' => match self.match_char('/') {
                true => self.comments(),
                false => match self.match_char('*') {
                    true => self.block_comments(),
                    false => self.add_token(TokenType::Slash, None),
                },
            },
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(),
            '0'..='9' => self.number(),
            _ if c.is_alphabetic() || c == '_' => self.identifier(),
            _ => self.error(self.line, "Unexpected character."),
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<LoxLiteral>) {
        let text = String::from(&self.source[self.start..self.current]);
        self.tokens.push(Token::new(
            token_type,
            text,
            literal,
            self.line,
            self.next_token_id,
        ));
        self.next_token_id += 1;
    }

    fn match_char(&mut self, expected: char) -> bool {
        match self.source_iter.peek() {
            Some(val) if *val == expected => {
                self.advance();
                true
            }
            _ => false,
        }
    }

    fn string(&mut self) {
        while let Some(&peek_char) = self.source_iter.peek() {
            if peek_char == '"' {
                break;
            }
            if peek_char == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            self.error(self.line, "Unterminated string.");
            return;
        }

        // Consume the closing '"'.
        self.advance();

        // Time the surrounding quotes.
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, Some(LoxLiteral::String(value)));
    }

    fn number(&mut self) {
        while let Some(&peek_char) = self.source_iter.peek() {
            if !peek_char.is_ascii_digit() {
                break;
            }
            self.advance();
        }

        // Look for a fractional part.
        if Some(&'.') == self.source_iter.peek() {
            // There is no double look ahead, so just creating a new char iter on string slice.
            if let Some(next_char) = self.source[self.current + 1..].chars().next() {
                if next_char.is_ascii_digit() {
                    // Consume the '.'
                    self.advance();
                    while let Some(&peek_char) = self.source_iter.peek() {
                        if !peek_char.is_ascii_digit() {
                            break;
                        }
                        self.advance();
                    }
                }
            }
        }

        self.add_token(
            TokenType::Number,
            Some(LoxLiteral::Number(
                self.source[self.start..self.current]
                    .parse()
                    .expect(&format!(
                        "Failed to parse number literal '{}' on line {}",
                        &self.source[self.start..self.current],
                        self.line
                    )),
            )),
        );
    }

    fn identifier(&mut self) {
        while let Some(&peek_char) = self.source_iter.peek() {
            if peek_char.is_alphanumeric() || peek_char == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let token_type = match self.keywords.get(&self.source[self.start..self.current]) {
            Some(&token_variant) => token_variant,
            None => TokenType::Identifier,
        };

        match token_type {
            TokenType::False => self.add_token(token_type, Some(LoxLiteral::Boolean(false))),
            TokenType::True => self.add_token(token_type, Some(LoxLiteral::Boolean(true))),
            TokenType::Nil => self.add_token(token_type, Some(LoxLiteral::Nil)),
            _ => self.add_token(token_type, None),
        }
    }

    fn comments(&mut self) {
        while let Some(&p) = self.source_iter.peek() {
            if p == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn block_comments(&mut self) {
        let mut current_char = self.advance();
        let mut nested_blocks = 0;
        while current_char != '\0' {
            if current_char == '*' && Some(&'/') == self.source_iter.peek() {
                // Consume the '/'.
                self.advance();
                if nested_blocks == 0 {
                    break;
                }
                nested_blocks -= 1;
            }
            if current_char == '/' && Some(&'*') == self.source_iter.peek() {
                nested_blocks += 1;
                // Consume the '*'.
                self.advance();
            }
            if current_char == '\n' {
                self.line += 1;
            }
            current_char = self.advance();
        }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.had_error = true;
        report(line, "", message);
    }
}
