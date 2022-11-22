use crate::{
    error::{Error, ErrorType},
    object::Object,
    token::{Token, TokenType},
};

pub struct Lexer {
    source: String,
    source_as_u8: Vec<u8>,
    source_len: usize,

    line: usize,
    current: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            source_as_u8: source.as_bytes().to_vec(),
            source_len: source.len(),
            line: 1,
            current: 0,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();

        while !self.is_eof() {
            let start = self.current;

            match self.current_char() {
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

                '/' => {
                    self.advance();
                    tokens.push(Token::new(
                        TokenType::Division,
                        &self.source[start..self.current],
                        Object::Nil,
                        self.line,
                    ));
                }

                current_char => {
                    if current_char.is_digit(10) {
                        let number = self.extract_number()?;
                        tokens.push(Token::new(
                            TokenType::Number,
                            &self.source[start..self.current],
                            Object::Number(number),
                            self.line,
                        ));
                    } else {
                        return Err(self.error(&format!("Invalid charecter '{}'", current_char)));
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

    fn current_char(&self) -> char {
        self.source_as_u8[self.current] as char
    }

    fn is_eof(&self) -> bool {
        if self.source_len > self.current {
            false
        } else {
            true
        }
    }

    fn extract_number(&mut self) -> Result<f64, Error> {
        let mut number = String::new();
        while self.current_char().is_digit(10) && !self.is_eof() {
            number.push(self.current_char());
            self.advance();
        }
        if let Ok(number) = number.parse() {
            Ok(number)
        } else {
            Err(self.error("Expected a digit"))
        }
    }

    fn error(&self, message: &str) -> Error {
        Error::new(ErrorType::LexingError, message, self.line)
    }
}
