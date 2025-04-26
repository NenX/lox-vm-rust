use std::{iter::Peekable, str::Chars};

use crate::{
    keyword_match,
    token::{Token, TokenType},
    MyPeekable,
};
use TokenType::*;
pub struct Scanner<'a> {
    source: &'a str,
    peekable: MyPeekable<Chars<'a>>,
    start: usize,
    current: usize,
    line: usize,
}
impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            peekable: MyPeekable::new(source.chars()),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    fn make_token(&self, t: TokenType) -> Token<'a> {
        Token::new(t, &self.source[self.start..self.current], self.line)
    }
    fn error_token(&self, message: &'a str) -> Token<'a> {
        Token::new(TokenError, message, self.line)
    }
    fn skip_whitespace(&mut self) {
        loop {
            if self.is_at_end() {
                break;
            }
            match self.peek() {
                Some(' ') | Some('\r') | Some('\t') => {
                    self.advance_unchecked();
                }
                Some('\n') => {
                    self.line += 1;
                    self.advance_unchecked();
                }
                Some('/') => {
                    if self.peek_next() == Some('/') {
                        while self.peek() != Some('\n') && !self.is_at_end() {
                            self.advance_unchecked();
                        }
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
    }
    pub fn scan_token(&mut self) -> Token<'a> {
        self.skip_whitespace();
        self.start = self.current;
        if self.is_at_end() {
            return self.make_token(TokenEof);
        }
        match self.advance_unchecked() {
            c if c.is_alphabetic() => return self.identifier(),
            c if c.is_numeric() => return self.number(),
            '"' => return self.string(),
            '(' => return self.make_token(TokenLeftParen),
            ')' => return self.make_token(TokenRightParen),
            '{' => return self.make_token(TokenLeftBrace),
            '}' => return self.make_token(TokenRightBrace),
            ';' => return self.make_token(TokenSemicolon),
            ',' => return self.make_token(TokenComma),
            '.' => return self.make_token(TokenDot),
            '-' => return self.make_token(TokenMinus),
            '+' => return self.make_token(TokenPlus),
            '/' => return self.make_token(TokenSlash),
            '*' => return self.make_token(TokenStar),
            '!' => return self.if_match_token('=', TokenBangEqual, TokenBang),
            '=' => return self.if_match_token('=', TokenEqualEqual, TokenEqual),
            '<' => return self.if_match_token('=', TokenLessEqual, TokenLess),
            '>' => return self.if_match_token('=', TokenGreaterEqual, TokenGreater),

            _ => {}
        };
        Token::new(TokenType::TokenEof, &self.source, 1)
    }
    fn advance_unchecked(&mut self) -> char {
        self.current += 1;
        let a = self.peekable.next();
        a.expect("msg")
    }
    fn peek(&mut self) -> Option<char> {
        let a = self.peekable.peek(0);
        a.copied()
    }
    fn peek_next(&mut self) -> Option<char> {
        let a = self.peekable.peek(1);
        a.copied()
    }
    fn is_at_end(&mut self) -> bool {
        self.peek().is_none()
    }

    fn if_match(&mut self, expected: char, match_expected: TokenType, not_match: TokenType) -> TokenType {
        if self.is_at_end() || self.peek().unwrap() != expected {
            return not_match;
        }
        self.advance_unchecked();
        match_expected
    }
    fn if_match_token(&mut self, expected: char, match_expected: TokenType, not_match: TokenType) -> Token<'a> {
        let t_type = self.if_match(expected, match_expected, not_match);
        return self.make_token(t_type);
    }
    fn string(&mut self) -> Token<'a> {
        loop {
            if self.is_at_end() {
                return self.error_token("Unterminated string");
            }
            let peek = self.peek().unwrap();
            if peek == '"' {
                self.advance_unchecked();
                break;
            }
            self.advance_unchecked();
        }
        self.make_token(TokenString)
    }
    fn number(&mut self) -> Token<'a> {
        let mut met_dot = false;
        loop {
            if self.is_at_end() {
                break;
            }
            let peek = self.peek().unwrap();

            if peek.is_numeric() {
                self.advance_unchecked();
                continue;
            }
            if peek == '.' && met_dot == false && self.peek_next().is_some_and(char::is_numeric) {
                met_dot = true;
                self.advance_unchecked();
                continue;
            }
            break;
        }
        self.make_token(TokenNumber)
    }
    fn identifier(&mut self) -> Token<'a> {
        loop {
            if self.is_at_end() {
                break;
            }
            let peek = self.peek().unwrap();
            if peek.is_alphanumeric() {
                self.advance_unchecked();
                continue;
            }
            break;
        }
        let maybe_keyword = keyword_match(&self.source[self.start..self.current]);
        if let Some(t_type) = maybe_keyword {
            return self.make_token(t_type);
        }
        self.make_token(TokenIdentifier)
    }
}
