use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub enum Object {
    Number(f64),
    Nil,
}

impl Display for Object {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Number(number) => write!(f, "Number({})", number),
            Self::Nil => write!(f, "Nil"),
        }
    }
}
