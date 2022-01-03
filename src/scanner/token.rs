use crate::TokenType;

#[derive(Debug)]
pub struct Token {
    tokentype: TokenType,
    lexeme: String,
    line: i32,
}

impl Token {
    pub fn new(tokentype: TokenType, lexeme: String, line: i32) -> Self {
        Token { tokentype, lexeme, line }
    }
}
