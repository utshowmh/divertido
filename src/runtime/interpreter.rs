use crate::general::statement::{AssignmentExpression, BlockStatement, IfStatement};
use crate::runtime::environment::Environment;

use crate::general::{
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
        for statement in &statements {
            self.execute(statement)?;
        }

        Ok(())
    }

    fn execute(&mut self, statement: &Statement) -> Result<Object, Error> {
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

    fn visit_assignment_statement(
        &mut self,
        statement: &AssignmentExpression,
    ) -> Result<Object, Error> {
        let value = self.evaluate(&statement.value)?;
        match self.environment.get(&statement.identifier) {
            Ok(_) => {
                self.environment.set(&statement.identifier, value);
                Ok(Object::Nil)
            }
            Err(error) => Err(error),
        }
    }

    fn visit_block_statement(&mut self, statement: &BlockStatement) -> Result<Object, Error> {
        for statement in &statement.statements {
            self.execute(&statement)?;
        }

        Ok(Object::Nil)
    }

    fn visit_if_statement(&mut self, statement: &IfStatement) -> Result<Object, Error> {
        let conditional = self.evaluate(&statement.conditional)?;

        if conditional.is_truthy() {
            self.execute(&statement.if_block)?;
        } else {
            if let Some(else_block) = &statement.else_block {
                self.execute(else_block)?;
            }
        }

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

            TokenType::Bang => match &right {
                Object::Boolean(boolean) => Ok(Object::Boolean(!boolean)),
                _ => Err(self.error(
                    &format!("Expected boolean after '!', found '{}", right),
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

            TokenType::EqualEqual => Ok(Object::Boolean(left == right)),

            TokenType::BangEqual => Ok(Object::Boolean(left != right)),

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
