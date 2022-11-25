use crate::{error::Error, expression::Expression, token::Token};

pub trait StatementVisitor<T> {
    fn visit_expression_statement(&self, statement: &ExpressionStatement) -> Result<T, Error>;
    fn visit_let_statement(&mut self, statement: &LetStatement) -> Result<T, Error>;
    fn visit_print_statement(&self, statement: &PrintStatement) -> Result<T, Error>;
}

#[derive(Debug)]
pub enum Statement {
    Expression(ExpressionStatement),
    Let(LetStatement),
    Print(PrintStatement),
}

impl Statement {
    pub fn accept<T>(&self, visitor: &mut dyn StatementVisitor<T>) -> Result<T, Error> {
        match self {
            Self::Expression(statement) => statement.accept(visitor),
            Self::Let(statement) => statement.accept(visitor),
            Self::Print(statement) => statement.accept(visitor),
        }
    }
}

#[derive(Debug)]
pub struct ExpressionStatement {
    pub expression: Expression,
}

impl ExpressionStatement {
    pub fn new(expression: Expression) -> Self {
        Self { expression }
    }

    pub fn accept<T>(&self, visitor: &dyn StatementVisitor<T>) -> Result<T, Error> {
        visitor.visit_expression_statement(self)
    }
}

#[derive(Debug)]
pub struct LetStatement {
    pub identifier: Token,
    pub value: Expression,
}

impl LetStatement {
    pub fn new(identifier: Token, value: Expression) -> Self {
        Self { identifier, value }
    }

    pub fn accept<T>(&self, visitor: &mut dyn StatementVisitor<T>) -> Result<T, Error> {
        visitor.visit_let_statement(self)
    }
}

#[derive(Debug)]
pub struct PrintStatement {
    pub value: Expression,
}

impl PrintStatement {
    pub fn new(value: Expression) -> Self {
        Self { value }
    }

    pub fn accept<T>(&self, visitor: &mut dyn StatementVisitor<T>) -> Result<T, Error> {
        visitor.visit_print_statement(self)
    }
}
