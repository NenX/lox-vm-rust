use std::collections::HashMap;

use crate::{chunk::OpCode, parse_f};

use super::{parse_rule, precedence, Compiler, ParseRule, Precedence, TokenType};
type RuleMap = HashMap<TokenType, ParseRule>;
use Precedence::*;
use TokenType::*;
pub fn literal(c: &mut Compiler) {
    let value = c.previous.as_value();
    c.chunk.write_constant(value, c.previous.line);
}
pub fn grouping(c: &mut Compiler) {
    parse_precedence(c, Precedence::PrecCall);
    c.consume(TokenType::TokenRightParen, "expect ')' after expression");
}
pub fn unary(c: &mut Compiler) {
    let token_type = c.previous.t_type;
    parse_precedence(c, Precedence::PrecUnary);

    match token_type {
        TokenType::TokenMinus => c.emit_byte(OpCode::OPNEGATE),
        TokenType::TokenBang => c.emit_byte(OpCode::OPNOT),
        _ => return,
    }
}
pub fn binary(c: &mut Compiler) {
    let operator_type = c.previous.t_type;
    parse_precedence(c, c.previous_rule().prec.next());
    match operator_type {
        TokenType::TokenPlus => c.emit_byte(OpCode::OPADD),
        TokenType::TokenMinus => c.emit_byte(OpCode::OPSUBTRACT),
        TokenType::TokenStar => c.emit_byte(OpCode::OPMULTIPLY),
        TokenType::TokenSlash => c.emit_byte(OpCode::OPDIVIDE),
        _ => return,
    }
}
pub fn parse_precedence(c: &mut Compiler, precedence: Precedence) {
    c.advance();

    let Some(prefix_rule) = c.previous_rule().prefix else {
        let message = format!("expect expression, got {:?}", c.previous.t_type);
        c.error(&message);
        return;
    };
    prefix_rule(c);
    loop {
        if precedence > c.current_rule().prec {
            break;
        }
        c.advance();
        let Some(infix_rule) = c.previous_rule().infix else {
            let message = format!("expect expression, got {:?}", c.previous.t_type);
            c.error(&message);
            return;
        };
        infix_rule(c);
    }
}
fn r_r(
    r: &mut RuleMap,
    t_type: TokenType,
    prefix: parse_f!(),
    infix: parse_f!(),
    prec: Precedence,
) {
    r.insert(
        t_type,
        ParseRule {
            prefix,
            infix,
            prec,
        },
    );
}
pub fn register_rules(r: &mut RuleMap) {
    r_r(r, TokenLeftParen, Some(grouping), None, PrecNone);
    r_r(r, TokenRightParen, None, None, PrecNone);
    r_r(r, TokenLeftBrace, None, None, PrecNone);
    r_r(r, TokenRightBrace, None, None, PrecNone);
    r_r(r, TokenComma, None, None, PrecNone);
    r_r(r, TokenDot, None, None, PrecNone);
    r_r(r, TokenMinus, Some(unary), Some(binary), PrecTerm);
    r_r(r, TokenPlus, None, Some(binary), PrecTerm);
    r_r(r, TokenSemicolon, None, None, PrecNone);
    r_r(r, TokenSlash, None, Some(binary), PrecFactor);
    r_r(r, TokenStar, None, Some(binary), PrecFactor);
    r_r(r, TokenBang, Some(unary), None, PrecNone);
    r_r(r, TokenBangEqual, None, Some(binary), PrecEquality);
    r_r(r, TokenEqual, None, None, PrecNone);
    r_r(r, TokenEqualEqual, None, Some(binary), PrecEquality);
    r_r(r, TokenGreater, None, Some(binary), PrecComparison);
    r_r(r, TokenGreaterEqual, None, Some(binary), PrecComparison);
    r_r(r, TokenLess, None, Some(binary), PrecComparison);
    r_r(r, TokenLessEqual, None, Some(binary), PrecComparison);
    r_r(r, TokenIdentifier, None, None, PrecNone);
    r_r(r, TokenString, None, None, PrecNone);
    r_r(r, TokenNumber, Some(literal), None, PrecNone);
    r_r(r, TokenAnd, None, None, PrecNone);
    r_r(r, TokenClass, None, None, PrecNone);
    r_r(r, TokenElse, None, None, PrecNone);
    r_r(r, TokenFalse, Some(literal), None, PrecNone);
    r_r(r, TokenFor, None, None, PrecNone);
    r_r(r, TokenFun, None, None, PrecNone);
    r_r(r, TokenIf, None, None, PrecNone);
    r_r(r, TokenNil, Some(literal), None, PrecNone);
    r_r(r, TokenOr, None, None, PrecNone);
    r_r(r, TokenPrint, None, None, PrecNone);
    r_r(r, TokenReturn, None, None, PrecNone);
    r_r(r, TokenSuper, None, None, PrecNone);
    r_r(r, TokenThis, None, None, PrecNone);
    r_r(r, TokenTrue, Some(literal), None, PrecNone);
    r_r(r, TokenVar, None, None, PrecNone);
    r_r(r, TokenWhile, None, None, PrecNone);
    r_r(r, TokenError, None, None, PrecNone);
    r_r(r, TokenEof, None, None, PrecNone);
}
