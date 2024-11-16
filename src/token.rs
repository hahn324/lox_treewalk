use crate::token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum LoxLiteral {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

impl LoxLiteral {
    pub fn stringify(&self) -> String {
        match *self {
            LoxLiteral::Number(val) => val.to_string(),
            LoxLiteral::String(ref val) => val.clone(),
            LoxLiteral::Boolean(val) => val.to_string(),
            LoxLiteral::Nil => "nil".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
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

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
