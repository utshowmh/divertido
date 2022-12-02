use crate::general::{
    error::{Error, ErrorType},
    expression::{
        BinaryExpression, Expression, GroupingExpression, LiteralExpression, UnaryExpression,
        VariableExpression,
    },
    statement::{
        AssignmentExpression, BlockStatement, ExpressionStatement, IfStatement, LetStatement,
        PrintStatement, Statement, WhileStatement,
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

    pub fn parse(&mut self) -> Result<Vec<Statement>, Error> {
        let mut statements = Vec::new();

        while !self.is_eof() {
            statements.push(self.statement()?);
        }

        Ok(statements)
    }

    fn statement(&mut self) -> Result<Statement, Error> {
        match self.peek().ttype {
            TokenType::Let => self.let_statement(),
            TokenType::Identifier => self.assignment_statement(),
            TokenType::OpenCurly => self.block_statement(),
            TokenType::If => self.if_statement(),
            TokenType::While => self.while_statement(),
            TokenType::Print => self.print_statement(),
            _ => self.expression_statement(),
        }
    }

    fn let_statement(&mut self) -> Result<Statement, Error> {
        self.advance();
        let identifier = self.consume(
            TokenType::Identifier,
            &format!(
                "Expected identifier after 'let', found '{}'",
                self.peek().lexeme
            ),
        )?;
        self.consume(
            TokenType::Equal,
            &format!(
                "Expected '=' after identifier, found '{}'",
                self.peek().lexeme
            ),
        )?;
        let value = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            &format!(
                "Expected ';' after variable declaration, found '{}'",
                self.peek().lexeme
            ),
        )?;
        Ok(Statement::Let(LetStatement::new(identifier, value)))
    }

    fn assignment_statement(&mut self) -> Result<Statement, Error> {
        let identifier = self.next_token();
        self.consume(
            TokenType::Equal,
            &format!(
                "Expected '=' after identifier, found '{}'",
                self.peek().lexeme
            ),
        )?;
        let value = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            &format!(
                "Expected ';' after variable declaration, found '{}'",
                self.peek().lexeme
            ),
        )?;
        Ok(Statement::Assignment(AssignmentExpression::new(
            identifier, value,
        )))
    }

    fn print_statement(&mut self) -> Result<Statement, Error> {
        self.advance();
        let expression = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            &format!(
                "Expected ';' after expression, found '{}'",
                self.peek().lexeme
            ),
        )?;
        Ok(Statement::Print(PrintStatement::new(expression)))
    }

    fn if_statement(&mut self) -> Result<Statement, Error> {
        self.advance();
        let conditional = self.expression()?;
        let if_block = self.block_statement()?;
        let mut else_block = None;

        if self.peek().ttype == TokenType::Else {
            self.advance();
            if self.peek().ttype == TokenType::If {
                else_block = Some(self.if_statement()?);
            } else {
                else_block = Some(self.block_statement()?);
            }
        }

        Ok(Statement::If(IfStatement::new(
            conditional,
            if_block,
            else_block,
        )))
    }

    fn while_statement(&mut self) -> Result<Statement, Error> {
        self.advance();
        let conditional = self.expression()?;
        let block = self.block_statement()?;
        Ok(Statement::While(WhileStatement::new(conditional, block)))
    }

    fn block_statement(&mut self) -> Result<Statement, Error> {
        self.advance();
        let mut statements = Vec::new();

        while !self.does_match(&[TokenType::CloseCurly]) && !self.is_eof() {
            statements.push(self.statement()?);
        }

        self.consume(
            TokenType::CloseCurly,
            &format!(
                "Expected '}}' after if_block, found '{}'",
                self.peek().lexeme
            ),
        )?;

        Ok(Statement::Block(BlockStatement::new(statements)))
    }

    fn expression_statement(&mut self) -> Result<Statement, Error> {
        let expression = self.expression()?;
        self.consume(
            TokenType::Semicolon,
            &format!(
                "Expected ';' after expression, found '{}'",
                self.peek().lexeme
            ),
        )?;
        Ok(Statement::Expression(ExpressionStatement::new(expression)))
    }

    fn expression(&mut self) -> Result<Expression, Error> {
        self.logical_expression()
    }

    fn logical_expression(&mut self) -> Result<Expression, Error> {
        self.logical_or()
    }

    fn logical_or(&mut self) -> Result<Expression, Error> {
        let mut left = self.logical_and()?;

        while self.does_match(&[TokenType::Or]) {
            let operator = self.next_token();
            let right = self.logical_and()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        Ok(left)
    }

    fn logical_and(&mut self) -> Result<Expression, Error> {
        let mut left = self.comparison()?;

        while self.does_match(&[TokenType::And]) {
            let operator = self.next_token();
            let right = self.comparison()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        Ok(left)
    }

    fn comparison(&mut self) -> Result<Expression, Error> {
        let mut left = self.term()?;

        while self.does_match(&[
            TokenType::BangEqual,
            TokenType::EqualEqual,
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.next_token();
            let right = self.term()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        Ok(left)
    }

    fn term(&mut self) -> Result<Expression, Error> {
        let mut left = self.factor()?;

        while self.does_match(&[TokenType::Plus, TokenType::Minus]) {
            let operator = self.next_token();
            let right = self.factor()?;
            left = Expression::Binary(BinaryExpression::new(left, operator, right));
        }

        Ok(left)
    }

    fn factor(&mut self) -> Result<Expression, Error> {
        let mut left = self.unary()?;

        while self.does_match(&[
            TokenType::Multiplication,
            TokenType::Division,
            TokenType::Modulo,
            TokenType::BitwiseAnd,
            TokenType::BitwiseOr,
        ]) {
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
        } else if self.does_match(&[TokenType::Bang]) {
            let operator = self.next_token();
            let right = self.primary()?;
            Ok(Expression::Unray(UnaryExpression::new(operator, right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expression, Error> {
        let current_token = self.peek();

        if self.does_match(&[
            TokenType::Number,
            TokenType::String,
            TokenType::True,
            TokenType::False,
            TokenType::Nil,
        ]) {
            self.advance();
            Ok(Expression::Literal(LiteralExpression::new(
                current_token.literal,
            )))
        } else if self.does_match(&[TokenType::Identifier]) {
            self.advance();
            Ok(Expression::Variable(VariableExpression::new(current_token)))
        } else if self.does_match(&[TokenType::OpenParen]) {
            self.advance();
            let expressions = self.expression()?;
            self.consume(
                TokenType::CloseParen,
                &format!(
                    "Expected ')' after expression, found '{}'",
                    self.peek().lexeme
                ),
            )?;
            Ok(Expression::Grouping(GroupingExpression::new(expressions)))
        } else {
            Err(self.error(&format!("Unexpected '{}'", self.peek().lexeme)))
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

    fn consume(&mut self, ttype: TokenType, message: &str) -> Result<Token, Error> {
        if self.peek().ttype == ttype {
            Ok(self.next_token())
        } else {
            Err(self.error(message))
        }
    }

    fn error(&self, message: &str) -> Error {
        Error::new(ErrorType::ParsingError, message, self.peek().line)
    }
}
