use crate::lox_object::LoxLiteral;
use crate::token_type::TokenType;
use std::{
    fmt,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Default)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<LoxLiteral>,
    pub line: usize,
    token_id: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<LoxLiteral>,
        line: usize,
        token_id: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
            token_id,
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

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_id == other.token_id
    }
}

impl Eq for Token {}

impl Hash for Token {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.token_id.hash(state);
    }
}
