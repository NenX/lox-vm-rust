use std::{collections::HashMap, sync::LazyLock};

use super::TokenType;
use TokenType::*;

type KType = LazyLock<HashMap<&'static str, TokenType>>;

const KEYWORDS: KType = KType::new(|| {
    HashMap::from([
        ("and", TokenAnd),
        ("class", TokenClass),
        ("else", TokenElse),
        ("false", TokenFalse),
        ("for", TokenFor),
        ("fun", TokenFun),
        ("if", TokenIf),
        ("nil", TokenNil),
        ("or", TokenOr),
        ("print", TokenPrint),
        ("return", TokenReturn),
        ("super", TokenSuper),
        ("this", TokenThis),
        ("true", TokenTrue),
        ("var", TokenVar),
        ("while", TokenWhile),
    ])
});

pub fn keyword_match(name: &str) -> Option<TokenType> {
    KEYWORDS.get(name).cloned()
}
