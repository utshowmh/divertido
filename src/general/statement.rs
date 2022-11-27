use crate::general::{error::Error, expression::Expression, token::Token};

pub trait StatementVisitor<T> {
    fn visit_expression_statement(&self, statement: &ExpressionStatement) -> Result<T, Error>;
    fn visit_let_statement(&mut self, statement: &LetStatement) -> Result<T, Error>;
    fn visit_assignment_statement(&mut self, statement: &AssignmentExpression) -> Result<T, Error>;
    fn visit_block_statement(&mut self, statement: &BlockStatement) -> Result<T, Error>;
    fn visit_if_statement(&mut self, statement: &IfStatement) -> Result<T, Error>;
    fn visit_print_statement(&self, statement: &PrintStatement) -> Result<T, Error>;
}

#[derive(Debug)]
pub enum Statement {
    Expression(ExpressionStatement),
    Let(LetStatement),
    Assignment(AssignmentExpression),
    Block(BlockStatement),
    If(IfStatement),
    Print(PrintStatement),
}

impl Statement {
    pub fn accept<T>(&self, visitor: &mut dyn StatementVisitor<T>) -> Result<T, Error> {
        match self {
            Self::Expression(statement) => statement.accept(visitor),
            Self::Let(statement) => statement.accept(visitor),
            Self::Assignment(statement) => statement.accept(visitor),
            Self::Block(statement) => statement.accept(visitor),
            Self::If(statement) => statement.accept(visitor),
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
pub struct AssignmentExpression {
    pub identifier: Token,
    pub value: Expression,
}

impl AssignmentExpression {
    pub fn new(identifier: Token, value: Expression) -> Self {
        Self { identifier, value }
    }

    pub fn accept<T>(&self, visitor: &mut dyn StatementVisitor<T>) -> Result<T, Error> {
        visitor.visit_assignment_statement(self)
    }
}

#[derive(Debug)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

impl BlockStatement {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    pub fn accept<T>(&self, visitor: &mut dyn StatementVisitor<T>) -> Result<T, Error> {
        visitor.visit_block_statement(self)
    }
}

#[derive(Debug)]
pub struct IfStatement {
    pub conditional: Expression,
    pub block: Box<Statement>,
    pub else_block: Option<Box<Statement>>,
}

impl IfStatement {
    pub fn new(conditional: Expression, block: Statement, else_block: Option<Statement>) -> Self {
        Self {
            conditional,
            block: Box::new(block),
            else_block: match else_block {
                Some(block) => Some(Box::new(block)),
                None => None,
            },
        }
    }

    pub fn accept<T>(&self, visitor: &mut dyn StatementVisitor<T>) -> Result<T, Error> {
        visitor.visit_if_statement(self)
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
