use crate::generel::{error::Error, object::Object, token::Token};

pub trait ExpressionVisitor<T> {
    fn visit_variable_expression(&self, expression: &VariableExpression) -> Result<T, Error>;
    fn visit_literal_expression(&self, expression: &LiteralExpression) -> Result<T, Error>;
    fn visit_unary_expression(&self, expression: &UnaryExpression) -> Result<T, Error>;
    fn visit_binary_expression(&self, expression: &BinaryExpression) -> Result<T, Error>;
    fn visit_gruping_expression(&self, expression: &GroupingExpression) -> Result<T, Error>;
}

#[derive(Debug)]
pub enum Expression {
    Variable(VariableExpression),
    Literal(LiteralExpression),
    Unray(UnaryExpression),
    Binary(BinaryExpression),
    Grouping(GroupingExpression),
}

impl Expression {
    pub fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, Error> {
        match self {
            Self::Variable(expression) => expression.accept(visitor),
            Self::Literal(expression) => expression.accept(visitor),
            Self::Unray(expression) => expression.accept(visitor),
            Self::Binary(expression) => expression.accept(visitor),
            Self::Grouping(expression) => expression.accept(visitor),
        }
    }
}

#[derive(Debug)]
pub struct VariableExpression {
    pub identifier: Token,
}

impl VariableExpression {
    pub fn new(identifier: Token) -> Self {
        Self { identifier }
    }

    pub fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, Error> {
        visitor.visit_variable_expression(self)
    }
}

#[derive(Debug)]
pub struct LiteralExpression {
    pub literal: Object,
}

impl LiteralExpression {
    pub fn new(literal: Object) -> Self {
        Self { literal }
    }

    pub fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, Error> {
        visitor.visit_literal_expression(self)
    }
}

#[derive(Debug)]
pub struct UnaryExpression {
    pub operator: Token,
    pub right: Box<Expression>,
}

impl UnaryExpression {
    pub fn new(operator: Token, right: Expression) -> Self {
        Self {
            operator,
            right: Box::new(right),
        }
    }

    pub fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, Error> {
        visitor.visit_unary_expression(self)
    }
}

#[derive(Debug)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

impl BinaryExpression {
    pub fn new(left: Expression, operator: Token, right: Expression) -> Self {
        Self {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    pub fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, Error> {
        visitor.visit_binary_expression(self)
    }
}

#[derive(Debug)]
pub struct GroupingExpression {
    pub expressions: Box<Expression>,
}

impl GroupingExpression {
    pub fn new(expressions: Expression) -> Self {
        Self {
            expressions: Box::new(expressions),
        }
    }

    pub fn accept<T>(&self, visitor: &dyn ExpressionVisitor<T>) -> Result<T, Error> {
        visitor.visit_gruping_expression(self)
    }
}
