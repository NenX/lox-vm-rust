use lox_vm_rust::{Scanner, Token, TokenType};

pub fn assert_number(scanner: &mut Scanner, value: f64) {
    let token = scanner.scan_token();
    assert_eq!(token.t_type, TokenType::TokenNumber);
    assert_eq!(token.start.parse::<f64>().unwrap(), value);
}
pub fn assert_string(scanner: &mut Scanner, value: &str) {
    let token = scanner.scan_token();
    assert_eq!(token.t_type, TokenType::TokenString);
    assert_eq!(token.start, value);
}
pub fn assert_token(scanner: &mut Scanner, t_type: TokenType) {
    let token = scanner.scan_token();
    assert_eq!(token.t_type, t_type);
}
pub fn assert_identifier(scanner: &mut Scanner, value: &str) {
    let token = scanner.scan_token();
    assert_eq!(token.t_type, TokenType::TokenIdentifier);
    assert_eq!(token.start, value);
}
