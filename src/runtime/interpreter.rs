use crate::runtime::environment::Environment;

use crate::generel::{
    error::{Error, ErrorType},
    expression::{
        BinaryExpression, Expression, ExpressionVisitor, GroupingExpression, LiteralExpression,
        UnaryExpression, VariableExpression,
    },
    object::Object,
    statement::{ExpressionStatement, LetStatement, PrintStatement, Statement, StatementVisitor},
    token::TokenType,
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn run(&mut self, statements: Vec<Statement>) -> Result<(), Error> {
        for statement in statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    fn execute(&mut self, statement: Statement) -> Result<Object, Error> {
        statement.accept(self)
    }

    fn evaluate(&self, expression: &Expression) -> Result<Object, Error> {
        expression.accept(self)
    }

    fn error(&self, message: &str, line: usize) -> Error {
        Error::new(ErrorType::RuntimeError, message, line)
    }
}

impl StatementVisitor<Object> for Interpreter {
    fn visit_expression_statement(&self, statement: &ExpressionStatement) -> Result<Object, Error> {
        self.evaluate(&statement.expression)
    }

    fn visit_let_statement(&mut self, statement: &LetStatement) -> Result<Object, Error> {
        let value = self.evaluate(&statement.value)?;
        self.environment.set(&statement.identifier, value);

        Ok(Object::Nil)
    }

    fn visit_print_statement(&self, statement: &PrintStatement) -> Result<Object, Error> {
        let value = self.evaluate(&statement.value)?;

        println!("{}", value);

        Ok(Object::Nil)
    }
}

impl ExpressionVisitor<Object> for Interpreter {
    fn visit_variable_expression(&self, expression: &VariableExpression) -> Result<Object, Error> {
        self.environment.get(&expression.identifier)
    }

    fn visit_literal_expression(&self, expression: &LiteralExpression) -> Result<Object, Error> {
        Ok(expression.literal.clone())
    }

    fn visit_unary_expression(&self, expression: &UnaryExpression) -> Result<Object, Error> {
        let right = self.evaluate(&expression.right)?;

        match &expression.operator.ttype {
            TokenType::Minus => match &right {
                Object::Number(number) => Ok(Object::Number(number * -1.)),
                _ => Err(self.error(
                    &format!("Expected number after '-', found '{}", right),
                    expression.operator.line,
                )),
            },
            _ => Err(self.error(
                &format!("Expected '-', found '{}", expression.operator.lexeme),
                expression.operator.line,
            )),
        }
    }

    fn visit_binary_expression(&self, expression: &BinaryExpression) -> Result<Object, Error> {
        let left = self.evaluate(&expression.left)?;
        let right = self.evaluate(&expression.right)?;

        match &expression.operator.ttype {
            TokenType::Plus => match (&left, &right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Number(x + y)),
                (_, _) => Err(self.error(
                    &format!("Expected 'number + number', found '{} + {}'", left, right),
                    expression.operator.line,
                )),
            },
            TokenType::Minus => match (&left, &right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Number(x - y)),
                (_, _) => Err(self.error(
                    &format!("Expected 'number - number', found '{} - {}'", left, right),
                    expression.operator.line,
                )),
            },
            TokenType::Multiplication => match (&left, &right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Number(x * y)),
                (_, _) => Err(self.error(
                    &format!("Expected 'number * number', found '{} * {}'", left, right),
                    expression.operator.line,
                )),
            },
            TokenType::Division => match (&left, &right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Number(x / y)),
                (_, _) => Err(self.error(
                    &format!("Expected 'number / number', found '{} / {}'", left, right),
                    expression.operator.line,
                )),
            },
            TokenType::EqualEqual => match (&left, &right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Boolean(x == y)),
                (Object::Boolean(x), Object::Boolean(y)) => Ok(Object::Boolean(x == y)),
                (Object::Nil, Object::Nil) => Ok(Object::Boolean(true)),
                (_, _) => Ok(Object::Boolean(false)),
            },
            TokenType::Greater => match (&left, &right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Boolean(x > y)),
                (_, _) => Err(self.error(
                    &format!("Expected 'number > number found '{} > {}'", left, right),
                    expression.operator.line,
                )),
            },
            TokenType::GreaterEqual => match (&left, &right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Boolean(x >= y)),
                (_, _) => Err(self.error(
                    &format!("Expected 'number >= number', found '{} >= {}'", left, right),
                    expression.operator.line,
                )),
            },
            TokenType::Less => match (&left, &right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Boolean(x < y)),
                (_, _) => Err(self.error(
                    &format!("Expected 'number < number', found '{} < {}'", left, right),
                    expression.operator.line,
                )),
            },
            TokenType::LessEqual => match (&left, &right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Boolean(x <= y)),
                (_, _) => Err(self.error(
                    &format!("Expected 'number <= number', found '{} <= {}'", left, right),
                    expression.operator.line,
                )),
            },
            _ => Err(self.error(
                &format!(
                    "Expected (+ | - | * | /), found '{}'",
                    expression.operator.lexeme
                ),
                expression.operator.line,
            )),
        }
    }

    fn visit_gruping_expression(&self, expression: &GroupingExpression) -> Result<Object, Error> {
        self.evaluate(&expression.expressions)
    }
}
