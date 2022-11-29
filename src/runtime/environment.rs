use std::collections::HashMap;

use crate::general::{
    error::{Error, ErrorType},
    object::Object,
    token::Token,
};

#[derive(Debug, Clone)]
pub struct Environment {
    bindings: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn set(&mut self, identifier: &Token, value: Object) {
        self.bindings.insert(identifier.lexeme.clone(), value);
    }

    pub fn get(&self, identifier: &Token) -> Result<Object, Error> {
        if let Some(value) = self.bindings.get(&identifier.lexeme) {
            Ok(value.clone())
        } else {
            Err(Error::new(
                ErrorType::RuntimeError,
                &format!("Variable with name '{}' not found", identifier.lexeme),
                identifier.line,
            ))
        }
    }
}
