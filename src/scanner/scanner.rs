use crate::TokenType;

pub struct Scanner {}

impl Scanner { 
    pub fn scan_tokens(source: &str) -> Vec<&str> {
        let splitted = source.split_whitespace();
        let result: Vec<&str> = splitted.collect();
        result
    }
}
