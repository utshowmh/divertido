use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Number(f64),
    Boolean(bool),
    String(String),
    Nil,
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Number(number) => write!(f, "{}", number),
            Self::Boolean(boolean) => write!(f, "{}", boolean),
            Self::String(string) => write!(f, "{}", string),
            Self::Nil => write!(f, "nil"),
        }
    }
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Nil => false,
            Self::Boolean(x) => x.clone(),
            _ => true,
        }
    }
}
