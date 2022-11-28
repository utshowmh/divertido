use std::collections::HashMap;

use crate::general::{
    error::{Error, ErrorType},
    object::Object,
    token::{Token, TokenType},
};

pub struct Lexer {
    source: String,
    source_as_u8: Vec<u8>,
    source_len: usize,

    keywords: HashMap<String, TokenType>,

    line: usize,
    current: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            source_as_u8: source.as_bytes().to_vec(),
            source_len: source.len(),

            keywords: HashMap::new(),

            line: 1,
            current: 0,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();
        self.init_keywords();

        while !self.is_eof() {
            let start = self.current;

            match self.peek() {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }

                '\n' => {
                    self.advance();
                    self.line += 1
                }

                '(' => {
                    self.advance();
                    tokens.push(Token::new(
                        TokenType::OpenParen,
                        &self.source[start..self.current],
                        Object::Nil,
                        self.line,
                    ));
                }

                ')' => {
                    self.advance();
                    tokens.push(Token::new(
                        TokenType::CloseParen,
                        &self.source[start..self.current],
                        Object::Nil,
                        self.line,
                    ));
                }

                '{' => {
                    self.advance();
                    tokens.push(Token::new(
                        TokenType::OpenCurly,
                        &self.source[start..self.current],
                        Object::Nil,
                        self.line,
                    ));
                }

                '}' => {
                    self.advance();
                    tokens.push(Token::new(
                        TokenType::CloseCurly,
                        &self.source[start..self.current],
                        Object::Nil,
                        self.line,
                    ));
                }

                '+' => {
                    self.advance();
                    tokens.push(Token::new(
                        TokenType::Plus,
                        &self.source[start..self.current],
                        Object::Nil,
                        self.line,
                    ));
                }

                '-' => {
                    self.advance();
                    tokens.push(Token::new(
                        TokenType::Minus,
                        &self.source[start..self.current],
                        Object::Nil,
                        self.line,
                    ));
                }

                '*' => {
                    self.advance();
                    tokens.push(Token::new(
                        TokenType::Multiplication,
                        &self.source[start..self.current],
                        Object::Nil,
                        self.line,
                    ));
                }

                ';' => {
                    self.advance();
                    tokens.push(Token::new(
                        TokenType::Semicolon,
                        &self.source[start..self.current],
                        Object::Nil,
                        self.line,
                    ));
                }

                '"' => {
                    self.advance();
                    let string = self.extract_string()?;
                    tokens.push(Token::new(
                        TokenType::String,
                        &self.source[start..self.current],
                        Object::String(string),
                        self.line,
                    ));
                }

                '/' => {
                    self.advance();
                    if self.peek() == '/' {
                        self.advance();
                        self.ignore_comment();
                    } else {
                        tokens.push(Token::new(
                            TokenType::Division,
                            &self.source[start..self.current],
                            Object::Nil,
                            self.line,
                        ));
                    }
                }

                '=' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        tokens.push(Token::new(
                            TokenType::EqualEqual,
                            &self.source[start..self.current],
                            Object::Nil,
                            self.line,
                        ));
                    } else {
                        tokens.push(Token::new(
                            TokenType::Equal,
                            &self.source[start..self.current],
                            Object::Nil,
                            self.line,
                        ));
                    }
                }

                '!' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        tokens.push(Token::new(
                            TokenType::BangEqual,
                            &self.source[start..self.current],
                            Object::Nil,
                            self.line,
                        ));
                    } else {
                        tokens.push(Token::new(
                            TokenType::Bang,
                            &self.source[start..self.current],
                            Object::Nil,
                            self.line,
                        ));
                    }
                }

                '>' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        tokens.push(Token::new(
                            TokenType::GreaterEqual,
                            &self.source[start..self.current],
                            Object::Nil,
                            self.line,
                        ));
                    } else {
                        tokens.push(Token::new(
                            TokenType::Greater,
                            &self.source[start..self.current],
                            Object::Nil,
                            self.line,
                        ));
                    }
                }

                '<' => {
                    self.advance();
                    if self.peek() == '=' {
                        self.advance();
                        tokens.push(Token::new(
                            TokenType::LessEqual,
                            &self.source[start..self.current],
                            Object::Nil,
                            self.line,
                        ));
                    } else {
                        tokens.push(Token::new(
                            TokenType::Less,
                            &self.source[start..self.current],
                            Object::Nil,
                            self.line,
                        ));
                    }
                }

                peek => {
                    if peek.is_digit(10) {
                        tokens.push(self.extract_number()?);
                    } else if peek.is_ascii_alphabetic() {
                        tokens.push(self.extract_identifier()?);
                    } else {
                        return Err(self.error(&format!("Invalid charecter '{}'", peek)));
                    }
                }
            }
        }

        tokens.push(Token::new(TokenType::EOF, "\0", Object::Nil, self.line));

        Ok(tokens)
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn peek(&self) -> char {
        self.source_as_u8[self.current] as char
    }

    fn is_eof(&self) -> bool {
        if self.source_len > self.current {
            false
        } else {
            true
        }
    }

    fn extract_number(&mut self) -> Result<Token, Error> {
        let mut number_str = String::new();
        while self.peek().is_digit(10) && !self.is_eof() {
            number_str.push(self.peek());
            self.advance();
        }

        if let Ok(number) = number_str.parse() {
            Ok(Token::new(
                TokenType::Number,
                &number_str,
                Object::Number(number),
                self.line,
            ))
        } else {
            Err(self.error("Expected a digit"))
        }
    }

    fn extract_string(&mut self) -> Result<String, Error> {
        let mut string = String::new();
        while self.peek().is_ascii() && !self.is_eof() && self.peek() != '"' {
            string.push(self.peek());
            self.advance();
        }

        if self.is_eof() {
            Err(self.error("Unterminated string"))
        } else {
            self.advance();
            Ok(string)
        }
    }

    fn extract_identifier(&mut self) -> Result<Token, Error> {
        let mut identifier = String::new();
        while self.peek().is_ascii_alphabetic() && !self.is_eof() {
            identifier.push(self.peek());
            self.advance();
        }

        if let Some(ttype) = self.keywords.get(&identifier) {
            match ttype {
                TokenType::True => Ok(Token::new(
                    TokenType::True,
                    &identifier,
                    Object::Boolean(true),
                    self.line,
                )),
                TokenType::False => Ok(Token::new(
                    TokenType::False,
                    &identifier,
                    Object::Boolean(false),
                    self.line,
                )),
                ttype => Ok(Token::new(
                    ttype.clone(),
                    &identifier,
                    Object::Nil,
                    self.line,
                )),
            }
        } else {
            Ok(Token::new(
                TokenType::Identifier,
                &identifier,
                Object::Nil,
                self.line,
            ))
        }
    }

    fn ignore_comment(&mut self) {
        while !self.is_eof() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn error(&self, message: &str) -> Error {
        Error::new(ErrorType::LexingError, message, self.line)
    }

    fn init_keywords(&mut self) {
        self.keywords.insert("let".to_string(), TokenType::Let);
        self.keywords.insert("if".to_string(), TokenType::If);
        self.keywords.insert("else".to_string(), TokenType::Else);
        self.keywords.insert("while".to_string(), TokenType::While);

        self.keywords.insert("true".to_string(), TokenType::True);
        self.keywords.insert("false".to_string(), TokenType::False);
        self.keywords.insert("nil".to_string(), TokenType::Nil);

        self.keywords.insert("print".to_string(), TokenType::Print);
    }
}
