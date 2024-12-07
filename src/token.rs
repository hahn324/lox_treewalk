use crate::lox_object::LoxLiteral;
use crate::token_type::TokenType;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LoxLiteral>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LoxLiteral>,
        line: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}
