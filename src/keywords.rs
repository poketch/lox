use std::collections::HashMap;
use crate::token::TokenType;

lazy_static::lazy_static! {
    pub static ref KEYWORDS: Vec< (String, TokenType)> = vec![
        ("and".to_string(), TokenType::AND),
        ("class".to_string(), TokenType::CLASS),
        ("else".to_string(), TokenType::ELSE),
        ("false".to_string(), TokenType::FALSE),
        ("for".to_string(), TokenType::FOR),
        ("proc".to_string(), TokenType::PROC),
        ("if".to_string(), TokenType::IF),
        ("nil".to_string(), TokenType::NIL),
        ("or".to_string(), TokenType::OR),
        ("print".to_string(), TokenType::PRINT),
        ("return".to_string(), TokenType::RETURN),
        ("super".to_string(), TokenType::SUPER),
        ("this".to_string(), TokenType::THIS),
        ("true".to_string(), TokenType::TRUE),
        ("let".to_string(), TokenType::LET),
        ("while".to_string(), TokenType::WHILE),
    ]; 

    pub static ref KEYWORD_COUNT : usize = KEYWORDS.len();

    pub static ref KEYWORD_MAP: HashMap<String, TokenType> = {
        let mut map = HashMap::with_capacity(*KEYWORD_COUNT);
        for kw in &*KEYWORDS {
            map.insert(kw.clone().0, kw.clone().1);
        }
        map
    };
}