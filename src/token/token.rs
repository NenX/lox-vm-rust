use crate::value::Value;

use super::token_type::TokenType;
use TokenType::*;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<'a> {
    pub t_type: TokenType,
    pub start: &'a str,
    pub line: usize,
}
impl<'a> Token<'a> {
    pub fn new(t_type: TokenType, lexme: &'a str, line: usize) -> Self {
        Self {
            t_type,
            start: lexme,
            line,
        }
    }
    pub fn is(&self, t_type: TokenType) -> bool {
        self.t_type == t_type
    }
    pub fn as_value(&self) -> Value {
        match self.t_type {
            TokenNumber => Value::Number(self.start.parse().unwrap()),
            TokenTrue => Value::Bool(true),
            TokenFalse => Value::Bool(false),
            TokenNil => Value::Nil,
            _ => panic!("not a value"),
        }
    }
}
