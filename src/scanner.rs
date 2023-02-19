use crate::error::LoxError;
use crate::token::{Object, Token, TokenType};
use crate::keywords::KEYWORD_MAP;
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into().chars().collect(),
            tokens: vec![],
            line: 0,
            start: 0,
            current: 0,
        }
    }

    fn add_token(&mut self, ttype: TokenType) -> () {
        self.add_token_with_object(ttype, None, None);
    }

    fn add_multi_token(&mut self, ttype: TokenType, lex: String) -> () {
        let tok = Token::new(ttype, lex, None, self.line);
        self.tokens.push(tok);
    }

    fn add_token_with_object(
        &mut self,
        ttype: TokenType,
        lex: Option<String>,
        object: Option<Object>,
    ) -> () {
        let lexeme: String = lex.unwrap_or(self.source[self.start..self.current].iter().collect());
        let tok = Token::new(ttype, lexeme, object, self.line);
        self.tokens.push(tok);
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        *self.source.get(self.current).unwrap()
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError::new(self.line, "Unterminated string."));
        }

        self.advance(); //including the closing '"'

        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_multi_token(TokenType::STRING, value);

        Ok(())
    }

    fn number(&mut self) -> Result<(), LoxError> {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' {
            if self.peek_next().is_digit(10) {
                self.advance();

                while self.peek().is_digit(10) {
                    self.advance();
                }
            } else {
                return Err(LoxError::new(
                    self.line,
                    "Error parsing Number literal. Trailing period not allowed",
                ));
            }
        }
        let value: String = self.source[self.start..self.current].iter().collect();
        self.add_token_with_object(
            TokenType::NUMBER,
            Some(value.clone()),
            Some(Object::Num(value.parse::<f64>().unwrap())),
        );

        Ok(())
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        *self.source.get(self.current + 1).unwrap()
    }

    fn identifier(&mut self) -> Result<(), LoxError> {
        while self.is_alpha_num(self.peek()) { self.advance(); }

        let text: String = self.source[self.start .. self.current].iter().collect();
        let ttype =  KEYWORD_MAP.get(&text).unwrap_or(&TokenType::IDENTIFIER);

        self.add_token(*ttype);
        Ok(())

    }

    fn is_alpha(&self, c: char) -> bool {
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
    }

    fn is_alpha_num(&self, c: char) -> bool {
        self.is_alpha(c) || c.is_digit(10)
    }
}

impl Scanner {
    fn advance(&mut self) -> char {
        self.current += 1;
        *self.source.get(self.current - 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();

        match c {
            '(' => Ok(self.add_token(TokenType::LEFT_PAREN)),
            ')' => Ok(self.add_token(TokenType::RIGHT_PAREN)),
            '{' => Ok(self.add_token(TokenType::LEFT_BRACE)),
            '}' => Ok(self.add_token(TokenType::RIGHT_BRACE)),
            ',' => Ok(self.add_token(TokenType::COMMA)),
            '.' => {
                if self.peek().is_digit(10) {
                    Err(LoxError::new(
                        self.line,
                        "Error Parsing number: Leading decimal not allowed",
                    ))
                } else {
                    Ok(self.add_token(TokenType::DOT))
                }
            },

            '-' => Ok(self.add_token(TokenType::MINUS)),
            '+' => Ok(self.add_token(TokenType::PLUS)),
            ';' => Ok(self.add_token(TokenType::SEMICOLON)),
            '*' => Ok(self.add_token(TokenType::STAR)),

            '!' => {
                if self.match_next_char('=') {
                    Ok(self.add_token(TokenType::BANG_EQUAL))
                } else {
                    Ok(self.add_token(TokenType::BANG))
                }
            }
            '=' => {
                if self.match_next_char('=') {
                    Ok(self.add_token(TokenType::EQUAL_EQUAL))
                } else {
                    Ok(self.add_token(TokenType::EQUAL))
                }
            }
            '<' => {
                if self.match_next_char('=') {
                    Ok(self.add_token(TokenType::LTE))
                } else {
                    Ok(self.add_token(TokenType::LT))
                }
            }
            '>' => {
                if self.match_next_char('=') {
                    Ok(self.add_token(TokenType::GTE))
                } else {
                    Ok(self.add_token(TokenType::GT))
                }
            }

            '/' => {
                if self.match_next_char('/') {
                    // this is a comment which will go until the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(())
                } else {
                    Ok(self.add_token(TokenType::SLASH))
                }
            }
            '"' => self.string(),

            ' ' => Ok(()),
            '\r' => Ok(()),
            '\t' => Ok(()),
            '\n' => {
                self.line += 1;
                Ok(())
            },
            _ => {
                if c.is_digit(10) {
                    // checking for base 10 number
                    self.number()
                } else if self.is_alpha(c) {
                    self.identifier()
                } else {
                    Err(LoxError::new(
                        self.line,
                        format!("Unexpected character in parsing: {}", c),
                    ))
                }
            }
        }?;

        Ok(())
    }

    fn match_next_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        };
        if *self.source.get(self.current).unwrap() != expected {
            return false;
        };

        self.current += 1;
        return true;
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.clone()
    }

    pub fn scan_tokens(&mut self) -> Result<(), LoxError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        let eof = Token::new(TokenType::EOF, "".to_string(), None, self.line);
        self.tokens.push(eof);
        Ok(())
    }
}
