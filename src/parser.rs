use crate::{
    error::{Error, ErrorType},
    expression::{
        BinaryExpression, Expression, GroupingExpression, LiteralExpression, UnaryExpression,
    },
    token::{Token, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Vec<Expression>, Error> {
        let mut expressions = Vec::new();

        while !self.is_eof() {
            expressions.push(self.term()?);
        }

        Ok(expressions)
    }

    fn expression(&mut self) -> Result<Expression, Error> {
        self.term()
    }

    fn term(&mut self) -> Result<Expression, Error> {
        let mut left = self.factor()?;

        if self.does_match(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.next_token();
            let right = self.factor()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        Ok(left)
    }

    fn factor(&mut self) -> Result<Expression, Error> {
        let mut left = self.unary()?;

        if self.does_match(&[TokenType::Multiplication, TokenType::Division]) {
            let operator = self.next_token();
            let right = self.unary()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right))
        }

        Ok(left)
    }

    fn unary(&mut self) -> Result<Expression, Error> {
        if self.does_match(&[TokenType::Minus]) {
            let operator = self.next_token();
            let right = self.primary()?;
            Ok(Expression::Unray(UnaryExpression::new(operator, right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expression, Error> {
        let current_token = self.peek();

        if self.does_match(&[TokenType::Number]) {
            self.advance();
            Ok(Expression::Literal(LiteralExpression::new(
                current_token.literal,
            )))
        } else if self.does_match(&[TokenType::OpenParen]) {
            self.advance();
            let expressions = self.expression()?;
            self.consume(TokenType::CloseParen, "')' after expression")?;
            Ok(Expression::Grouping(GroupingExpression::new(expressions)))
        } else {
            Err(self.error("'Number' or '('"))
        }
    }

    fn is_eof(&self) -> bool {
        self.tokens[self.current].ttype == TokenType::EOF
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn advance(&mut self) {
        if !self.is_eof() {
            self.current += 1
        }
    }

    fn next_token(&mut self) -> Token {
        let token = self.peek();
        self.advance();
        token
    }

    fn does_match(&mut self, ttypes: &[TokenType]) -> bool {
        ttypes.contains(&self.peek().ttype)
    }

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<(), Error> {
        if self.peek().ttype == ttype {
            self.advance();
            Ok(())
        } else {
            Err(self.error(message))
        }
    }

    fn error(&self, message: &str) -> Error {
        Error::new(
            ErrorType::ParsingError,
            &format!("Expected {} found '{}'", message, self.peek().lexeme),
            self.peek().line,
        )
    }
}
