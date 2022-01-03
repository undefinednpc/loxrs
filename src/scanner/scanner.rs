use crate::{Token, TokenType};

pub struct Scanner {}

impl Scanner { 
    pub fn scan_tokens(source: &str) -> Vec<Token> {
        let splitted = source.split_whitespace();
        let mut result: Vec<Token> = Vec::new();
        for token in splitted {
            result.push(Token::new(TokenType::String, token.to_string(), 1));
        }
        result
    }
}
