use crate::{lox_object::LoxLiteral, token_type::TokenType};
use std::{
    fmt,
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Default)]
pub struct Token<'src> {
    pub token_type: TokenType,
    pub lexeme: &'src str,
    pub literal: Option<LoxLiteral>,
    pub line: usize,
    token_id: usize,
}

impl<'src> Token<'src> {
    pub fn new(
        token_type: TokenType,
        lexeme: &'src str,
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

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} {} {:?}",
            self.token_type, self.lexeme, self.literal
        )
    }
}

impl PartialEq for Token<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.token_id == other.token_id
    }
}

impl Eq for Token<'_> {}

impl Hash for Token<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.token_id.hash(state);
    }
}
