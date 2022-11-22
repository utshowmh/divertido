use crate::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    Expression(ExpressionStatement),
}

#[derive(Debug)]
pub struct ExpressionStatement {
    _expression: Expression,
}

impl ExpressionStatement {
    pub fn new(expression: Expression) -> Self {
        Self {
            _expression: expression,
        }
    }
}
