use crate::expression::Expression;

#[derive(Debug)]
pub enum Statement {
    Expression(ExpressionStatement),
    Let(LetStatement),
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

#[derive(Debug)]
pub struct LetStatement {
    _identifier: Expression,
    _value: Expression,
}

impl LetStatement {
    pub fn new(identifier: Expression, value: Expression) -> Self {
        Self {
            _identifier: identifier,
            _value: value,
        }
    }
}
