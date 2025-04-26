use crate::{chunk::Chunk, token::Token, Scanner, TokenType};

pub struct Parser<'a> {
    had_error: bool,
    pub previous: Token<'a>, // 当前正在解析的token
    pub current: Token<'a>,  // 下一个token
    scanner: Scanner<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            had_error: false,
            previous: Token::new(TokenType::TokenNil, "", 0),
            current: Token::new(TokenType::TokenNil, "", 0),
            scanner: Scanner::new(source),
        }
    }


}
