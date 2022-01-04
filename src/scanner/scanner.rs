use crate::{Token, TokenType};

pub struct Scanner {}

impl Scanner { 
    pub fn scan_tokens(source: &str) -> Vec<Token> {
        let mut start: usize = 0;
        let mut current: usize = 0;
        let mut line: usize = 1;
        let mut result: Vec<Token> = Vec::new();
        for character in source.chars() {
            match character {
                '(' => result.push(Token::new(TokenType::LeftParen, character.to_string(), line)),
                ')' => result.push(Token::new(TokenType::RightParen, character.to_string(), line)),
                '{' => result.push(Token::new(TokenType::LeftBrace, character.to_string(), line)),
                '}' => result.push(Token::new(TokenType::RightParen, character.to_string(), line)),
                ',' => result.push(Token::new(TokenType::Comma, character.to_string(), line)),
                '.' => result.push(Token::new(TokenType::Dot, character.to_string(), line)),
                '-' => result.push(Token::new(TokenType::Minus, character.to_string(), line)),
                '+' => result.push(Token::new(TokenType::Plus, character.to_string(), line)),
                ';' => result.push(Token::new(TokenType::Semicolon, character.to_string(), line)),
                '*' => result.push(Token::new(TokenType::Star, character.to_string(), line)),
                _ => todo!()
            }
        }
        result
    }
}
