use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    //Single Character 
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,

    //One or two characters
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GT, GTE,
    LT, LTE,

    // Literals
    IDENTIFIER, STRING, NUMBER,

    // Keywords
    AND, CLASS, ELSE, FALSE, PROC, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, LET, WHILE,
    
    EOF,
}

#[derive(Clone)]
pub enum Object {
    Num(f64),
    Str(String),
    Nil,
    True,
    False,
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(s) => write!(f, "\"{s}\""),
            Object::Nil => write!(f, "nil"),
            Object::True => write!(f, "true"),
            Object::False => write!(f, "false"),
        }
    }
}

#[derive(Clone)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Self {
        Self {
            ttype,
            lexeme,
            literal,
            line,
        }
    } 
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
        "{:?} : {} : {} at line {}", 
        self.ttype,
        self.lexeme,
        if let Some(literal) = &self.literal { literal.to_string() } else { "None".to_string() },
        self.line)
    }
}