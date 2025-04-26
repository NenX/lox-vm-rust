mod common;
use common::{assert_identifier, assert_number, assert_string, assert_token};
use lox_vm_rust::{Scanner, TokenType};

#[test]
fn test_comment() {
    let mut scanner = Scanner::new("// abc\n1");
    assert_number(&mut scanner, 1.0);
}

#[test]
fn test_whitespace() {
    let mut scanner = Scanner::new(" 1");
    assert_number(&mut scanner, 1.0);
}

#[test]
fn test_number() {
    let mut scanner = Scanner::new("123.22.4");
    assert_number(&mut scanner, 123.22);

    assert_token(&mut scanner, TokenType::TokenDot);
    assert_number(&mut scanner, 4.0);
}
#[test]
fn test_string() {
    let mut scanner = Scanner::new(r#" "12" 45"#);

    assert_string(&mut scanner, r#""12""#);
    assert_number(&mut scanner, 45.0);
}

#[test]
fn test_keyword() {
    let mut scanner = Scanner::new("var a != 1");
    assert_token(&mut scanner, TokenType::TokenVar);
    assert_identifier(&mut scanner, "a");
    assert_token(&mut scanner, TokenType::TokenBangEqual);
    assert_number(&mut scanner, 1.0);
}
#[test]
fn test_eof() {
    let mut scanner = Scanner::new("var a != 1");
    for _ in 0..4 {
        scanner.scan_token();
    }
    assert_token(&mut scanner, TokenType::TokenEof);
}
#[test]
fn test_error() {
    let mut scanner = Scanner::new(r#" "12"#);
    assert_token(&mut scanner, TokenType::TokenError);
}
