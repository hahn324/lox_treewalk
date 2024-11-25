use crate::report;
use crate::token::{LoxLiteral, Token};
use crate::token_type::TokenType;
use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'a> {
    pub tokens: Vec<Token>,
    pub had_error: bool,
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut keywords = HashMap::with_capacity(16);
        keywords.insert(String::from("and"), TokenType::And);
        keywords.insert(String::from("class"), TokenType::Class);
        keywords.insert(String::from("else"), TokenType::Else);
        keywords.insert(String::from("false"), TokenType::False);
        keywords.insert(String::from("for"), TokenType::For);
        keywords.insert(String::from("fun"), TokenType::Fun);
        keywords.insert(String::from("if"), TokenType::If);
        keywords.insert(String::from("nil"), TokenType::Nil);
        keywords.insert(String::from("or"), TokenType::Or);
        keywords.insert(String::from("print"), TokenType::Print);
        keywords.insert(String::from("return"), TokenType::Return);
        keywords.insert(String::from("super"), TokenType::Super);
        keywords.insert(String::from("this"), TokenType::This);
        keywords.insert(String::from("true"), TokenType::True);
        keywords.insert(String::from("var"), TokenType::Var);
        keywords.insert(String::from("while"), TokenType::While);
        keywords.insert(String::from("break"), TokenType::Break);

        Scanner {
            tokens: Vec::new(),
            had_error: false,
            source,
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) {
        let mut char_iter: Peekable<Chars<'_>> = self.source.chars().peekable();
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token(&mut char_iter);
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), None, self.line));
    }

    pub fn take(&mut self) -> Result<Vec<Token>, Vec<Token>> {
        self.start = 0;
        self.current = 0;
        self.line = 1;
        let tokens = std::mem::replace(&mut self.tokens, Vec::new());
        if self.had_error {
            self.had_error = false;
            Err(tokens)
        } else {
            Ok(tokens)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self, char_iter: &mut impl Iterator<Item = char>) -> char {
        match char_iter.next() {
            Some(return_char) => {
                self.current += return_char.len_utf8();
                return_char
            }
            None => '\0',
        }
    }

    fn scan_token(&mut self, char_iter: &mut Peekable<Chars<'_>>) {
        let c = self.advance(char_iter);
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
            '!' => match self.match_char(char_iter, '=') {
                true => self.add_token(TokenType::BangEqual, None),
                false => self.add_token(TokenType::Bang, None),
            },
            '=' => match self.match_char(char_iter, '=') {
                true => self.add_token(TokenType::EqualEqual, None),
                false => self.add_token(TokenType::Equal, None),
            },
            '<' => match self.match_char(char_iter, '=') {
                true => self.add_token(TokenType::LessEqual, None),
                false => self.add_token(TokenType::Less, None),
            },
            '>' => match self.match_char(char_iter, '=') {
                true => self.add_token(TokenType::GreaterEqual, None),
                false => self.add_token(TokenType::Greater, None),
            },
            '/' => match self.match_char(char_iter, '/') {
                true => self.comments(char_iter),
                false => match self.match_char(char_iter, '*') {
                    true => self.block_comments(char_iter),
                    false => self.add_token(TokenType::Slash, None),
                },
            },
            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,
            '"' => self.string(char_iter),
            '0'..='9' => self.number(char_iter),
            _ if c.is_alphabetic() || c == '_' => self.identifier(char_iter),
            _ => self.error(self.line, "Unexpected character."),
        }
    }

    fn add_token(&mut self, token_type: TokenType, literal: Option<LoxLiteral>) {
        let text = String::from(&self.source[self.start..self.current]);
        self.tokens
            .push(Token::new(token_type, text, literal, self.line));
    }

    fn match_char(&mut self, char_iter: &mut Peekable<Chars<'_>>, expected: char) -> bool {
        match char_iter.peek() {
            Some(val) if *val == expected => {
                self.advance(char_iter);
                true
            }
            _ => false,
        }
    }

    fn string(&mut self, char_iter: &mut Peekable<Chars<'_>>) {
        while let Some(&peek_char) = char_iter.peek() {
            if peek_char == '"' {
                break;
            }
            if peek_char == '\n' {
                self.line += 1;
            }
            self.advance(char_iter);
        }
        if self.is_at_end() {
            self.error(self.line, "Unterminated string.");
            return;
        }

        // Consume the closing '"'.
        self.advance(char_iter);

        // Time the surrounding quotes.
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String, Some(LoxLiteral::String(value)));
    }

    fn number(&mut self, char_iter: &mut Peekable<Chars<'_>>) {
        while let Some(&peek_char) = char_iter.peek() {
            if !peek_char.is_ascii_digit() {
                break;
            }
            self.advance(char_iter);
        }

        // Look for a fractional part.
        if Some(&'.') == char_iter.peek() {
            // There is no double look ahead, so just creating a new char iter on string slice.
            if let Some(next_char) = self.source[self.current + 1..].chars().next() {
                if next_char.is_ascii_digit() {
                    // Consume the '.'
                    self.advance(char_iter);
                    while let Some(&peek_char) = char_iter.peek() {
                        if !peek_char.is_ascii_digit() {
                            break;
                        }
                        self.advance(char_iter);
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

    fn identifier(&mut self, char_iter: &mut Peekable<Chars<'_>>) {
        while let Some(&peek_char) = char_iter.peek() {
            if peek_char.is_alphanumeric() || peek_char == '_' {
                self.advance(char_iter);
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

    fn comments(&mut self, char_iter: &mut Peekable<Chars<'_>>) {
        while let Some(&p) = char_iter.peek() {
            if p == '\n' {
                break;
            }
            self.advance(char_iter);
        }
    }

    fn block_comments(&mut self, char_iter: &mut Peekable<Chars<'_>>) {
        let mut current_char = self.advance(char_iter);
        let mut nested_blocks = 0;
        while current_char != '\0' {
            if current_char == '*' && Some(&'/') == char_iter.peek() {
                // Consume the '/'.
                self.advance(char_iter);
                if nested_blocks == 0 {
                    break;
                }
                nested_blocks -= 1;
            }
            if current_char == '/' && Some(&'*') == char_iter.peek() {
                nested_blocks += 1;
                // Consume the '*'.
                self.advance(char_iter);
            }
            if current_char == '\n' {
                self.line += 1;
            }
            current_char = self.advance(char_iter);
        }
    }

    fn error(&mut self, line: usize, message: &str) {
        self.had_error = true;
        report(line, "", message);
    }
}
