use crate::{
    general::{
        error::{Error, ErrorType},
        expression::{
            BinaryExpression, Expression, ExpressionVisitor, GroupingExpression, LiteralExpression,
            UnaryExpression, VariableExpression,
        },
        object::Object,
        statement::{
            AssignmentExpression, BlockStatement, ExpressionStatement, IfStatement, LetStatement,
            PrintStatement, Statement, StatementVisitor, WhileStatement,
        },
        token::TokenType,
    },
    runtime::environment::Environment,
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

    fn visit_while_statement(&mut self, statement: &WhileStatement) -> Result<Object, Error> {
        loop {
            let conditional = self.evaluate(&statement.conditional)?;
            if conditional.is_truthy() {
                self.execute(&statement.block)?;
            } else {
                break;
            }
        }
        Ok(Object::Nil)
    }

    fn visit_print_statement(&self, statement: &PrintStatement) -> Result<Object, Error> {
        let mut values = Vec::new();
        for value in &statement.values {
            values.push(self.evaluate(value)?);
        }
        for value in values {
            print!("{}", value);
        }
        println!();
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
                (Object::String(x), Object::String(y)) => Ok(Object::String(x.to_string() + y)),
                (_, _) => Err(self.error(
                    &format!(
                        "Expected 'number/string + number/string', found '{} + {}'",
                        left, right
                    ),
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
            TokenType::Modulo => match (&left, &right) {
                (Object::Number(x), Object::Number(y)) => Ok(Object::Number(x % y)),
                (_, _) => Err(self.error(
                    &format!("Expected 'number % number', found '{} % {}'", left, right),
                    expression.operator.line,
                )),
            },

            TokenType::BitwiseAnd => todo!(),

            TokenType::BitwiseOr => todo!(),

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

            TokenType::And => Ok(Object::Boolean(left.is_truthy() && right.is_truthy())),

            TokenType::Or => Ok(Object::Boolean(left.is_truthy() || right.is_truthy())),

            _ => Err(self.error(
                &format!(
                    "Expected a Binary Operator, found '{}'",
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
