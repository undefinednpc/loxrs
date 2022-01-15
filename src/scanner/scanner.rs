use crate::{Token, TokenType as TT};

pub struct Scanner {}

impl Scanner { 
    pub fn scan_tokens(source: &str) -> Vec<Token> {
        let mut start: usize = 0;
        let mut current: usize = 0;
        let mut line: usize = 1;
        let mut result: Vec<Token> = Vec::new();
        for character in source.chars() {
            match character {
                '(' => result.push(Token::new(TT::LeftParen, character.to_string(), line)),
                ')' => result.push(Token::new(TT::RightParen, character.to_string(), line)),
                '{' => result.push(Token::new(TT::LeftBrace, character.to_string(), line)),
                '}' => result.push(Token::new(TT::RightParen, character.to_string(), line)),
                ',' => result.push(Token::new(TT::Comma, character.to_string(), line)),
                '.' => result.push(Token::new(TT::Dot, character.to_string(), line)),
                '-' => result.push(Token::new(TT::Minus, character.to_string(), line)),
                '+' => result.push(Token::new(TT::Plus, character.to_string(), line)),
                ';' => result.push(Token::new(TT::Semicolon, character.to_string(), line)),
                '*' => result.push(Token::new(TT::Star, character.to_string(), line)),
                _ => todo!()
            }
        }
        result
    }
}
