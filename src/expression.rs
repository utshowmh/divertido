use crate::{object::Object, token::Token};

#[derive(Debug)]
pub enum Expression {
    Literal(LiteralExpression),
    Unray(UnaryExpression),
    Binary(BinaryExpression),
    Grouping(GroupingExpression),
}

#[derive(Debug)]
pub struct LiteralExpression {
    _literal: Object,
}

impl LiteralExpression {
    pub fn new(literal: Object) -> Self {
        Self { _literal: literal }
    }
}

#[derive(Debug)]
pub struct UnaryExpression {
    _operator: Token,
    _right: Box<Expression>,
}

impl UnaryExpression {
    pub fn new(operator: Token, right: Expression) -> Self {
        Self {
            _operator: operator,
            _right: Box::new(right),
        }
    }
}

#[derive(Debug)]
pub struct BinaryExpression {
    _left: Box<Expression>,
    _operator: Token,
    _right: Box<Expression>,
}

impl BinaryExpression {
    pub fn new(left: Expression, operator: Token, right: Expression) -> Self {
        Self {
            _left: Box::new(left),
            _operator: operator,
            _right: Box::new(right),
        }
    }
}

#[derive(Debug)]
pub struct GroupingExpression {
    _expressions: Box<Expression>,
}

impl GroupingExpression {
    pub fn new(expressions: Expression) -> Self {
        Self {
            _expressions: Box::new(expressions),
        }
    }
}
