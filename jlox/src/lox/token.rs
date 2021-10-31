use std::fmt;

use super::tokentype::TokenType;
use super::literal::Literal;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<Literal>, line: u32) -> Token {
        Token {
            token_type: token_type,
            lexeme: lexeme,
            literal: literal,
            line: line
        }
    }
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.literal {
            Some(literal) => write!(f, "{} {} {}", self.token_type, self.lexeme, literal),
            None => write!(f, "{} {}", self.token_type, self.lexeme)
        }
    }
}
