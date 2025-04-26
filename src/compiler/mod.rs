mod parse_rule;
mod precedence;
mod rules;
use std::collections::HashMap;

use crate::{
    chunk::{Chunk, OpCode},
    parse_f,
    parser::Parser,
    value::Value,
    Scanner, Token, TokenType,
};
pub use parse_rule::*;
use precedence::{Precedence, Precedence::*};
use rules::*;
use TokenType::*;
pub struct Compiler<'a> {
    pub previous: Token<'a>, // 当前正在解析的token
    pub current: Token<'a>,  // 下一个token
    chunk: &'a mut Chunk,
    had_error: bool,
    panic_mode: bool,
    scanner: Scanner<'a>,
    rules: HashMap<TokenType, ParseRule>,
}

impl<'a> Compiler<'a> {
    pub fn new(chunk: &'a mut Chunk, source: &'a str) -> Self {
        Self {
            chunk,
            rules: HashMap::new(),
            had_error: false,
            panic_mode: false,
            previous: Token::new(TokenType::TokenNil, "", 0),
            current: Token::new(TokenType::TokenNil, "", 0),
            scanner: Scanner::new(source),
        }
    }
    pub fn compile(&mut self) -> bool {
        register_rules(&mut self.rules);

        self.advance();
        self.expression();
        self.consume(TokenType::TokenEof, "expect end of expression");
        self.emit_return();
        !self.had_error
    }
    pub fn expression(&mut self) {
        parse_precedence(self, PrecAssignment);
    }
    fn get_rule(&self, t_type: TokenType) -> &ParseRule {
        self.rules.get(&t_type).unwrap()
    }
    pub fn previous_rule(&self) -> &ParseRule {
        self.get_rule(self.previous.t_type)
    }
    pub fn current_rule(&self) -> &ParseRule {
        self.get_rule(self.current.t_type)
    }
    pub fn advance(&mut self) {
        self.previous = self.current;
        loop {
            self.current = self.scanner.scan_token();
            if self.current.t_type != TokenType::TokenError {
                break;
            }
            self.error_at_current(self.current.start);
        }
    }
    pub fn consume(&mut self, t_type: TokenType, message: &str) {
        if self.current.t_type == t_type {
            self.advance();
            return;
        }
        self.error_at_current(message);
    }
    fn error_at(&mut self, token: Token, message: &str) {
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;
        eprint!("[line {}] Error", token.line);
        if token.t_type == TokenType::TokenEof {
            eprintln!(" at end");
        } else if token.t_type == TokenType::TokenError {
            // Nothing.
        } else {
            eprint!(" at '{}'", token.start);
        }
        eprint!(": {}\n", message);
        self.had_error = true;
    }
    fn error_at_current(&mut self, message: &str) {
        self.error_at(self.current, message);
    }
    fn error(&mut self, message: &str) {
        self.error_at(self.previous, message);
    }

    fn emit_byte(&mut self, byte: OpCode) {
        self.chunk.write_chunk(byte, self.previous.line);
    }
    fn emit_return(&mut self) {
        self.emit_byte(OpCode::OPRETURN);
    }
    fn emit_bytes(&mut self, bytes: &[OpCode]) {
        for byte in bytes {
            self.emit_byte(*byte);
        }
    }
}
#[test]
fn test() {
    let mut chunk = Chunk::new();
    let mut compiler = Compiler::new(&mut chunk, "1+2*3");
    compiler.compile();
    chunk.disassemble("?");
}
