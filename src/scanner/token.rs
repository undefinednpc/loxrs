use crate::TokenType;

#[derive(Debug)]
pub struct Token {
    tokentype: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(tokentype: TokenType, lexeme: String, line: usize) -> Self {
        Token { tokentype, lexeme, line }
    }
}
