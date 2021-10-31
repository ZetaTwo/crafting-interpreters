use std::iter;

use super::token::Token;
use super::tokentype::TokenType;
use super::literal::Literal;
use super::Lox;

pub struct Scanner<'a> {
    lox: &'a mut Lox,
    source: String,
    start: usize,
    current: usize,
    line: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(lox: &mut Lox, line: String) -> Scanner {
        Scanner { 
            lox: lox,
            source: line,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> std::iter::Chain<&mut Scanner<'a>, std::iter::Once<Token>> {
        self.chain(iter::once(Token::new(TokenType::Eof, "".to_string(), None, self.line)))
    }

    fn scan_token(&mut self) -> Option<Token> {
        let c = self.advance();
        match c {
            Some('(') => self.create_simple_token(TokenType::LeftParen),
            Some(')') => self.create_simple_token(TokenType::RightParen),
            Some('{') => self.create_simple_token(TokenType::LeftBrace),
            Some('}') => self.create_simple_token(TokenType::RightBrace),
            Some(',') => self.create_simple_token(TokenType::Comma),
            Some('.') => self.create_simple_token(TokenType::Dot),
            Some('-') => self.create_simple_token(TokenType::Minus),
            Some('+') => self.create_simple_token(TokenType::Plus),
            Some(';') => self.create_simple_token(TokenType::Semicolon),
            Some('*') => self.create_simple_token(TokenType::Star),

            Some(' ') => None,
            Some('\r') => None,
            Some('\t') => None,
                    
            Some('\n') => {
                self.line += 1;
                None
            },

            None => None,
            //Some(cv) => unimplemented!("Token \"{}\" not supported", cv)
            Some(c) => {
                println!("DBG: char: \"{}\"", c);
                self.lox.error(self.line, "Unexpected character.");
                None
            }
        }
    }

    fn create_simple_token(&mut self, token_type: TokenType) -> Option<Token> {
        self.create_token(token_type, None)
    }

    fn create_token(&mut self, token_type: TokenType, literal: Option<Literal>) -> Option<Token> {
        let remaining = self.source[self.start..].chars();
        let text = remaining.take(self.current - self.start).collect();
        Some(Token::new(token_type, text, literal, self.line))
    }

    fn advance(&mut self) -> Option<char> {
        let mut remaining = self.source[self.start..].chars();

        match remaining.next() {
            Some(n) => {
                self.current += n.len_utf8();
                Some(n)
            },

            None => None
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}


impl<'a> Iterator for Scanner<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.is_at_end() {
            let token = self.scan_token();
            self.start = self.current;
            match token {
                Some(token) => {
                    return Some(token);
                },
                None => {
                    continue;
                }
            }
        }
            
        None
    }
}
