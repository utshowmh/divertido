use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Object {
    Number(f64),
    Boolean(bool),
    Nil,
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Number(number) => write!(f, "number({})", number),
            Self::Boolean(boolean) => write!(f, "boolean({})", boolean),
            Self::Nil => write!(f, "nil"),
        }
    }
}
