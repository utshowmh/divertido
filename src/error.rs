use std::{
    fmt::{Display, Formatter, Result},
    process::exit,
};

pub enum ErrorType {
    LexingError,
    ParsingError,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::LexingError => write!(f, "LexingError"),
            Self::ParsingError => write!(f, "ParsingError"),
        }
    }
}

pub struct Error {
    etype: ErrorType,
    message: String,
    line: usize,
}

impl Error {
    pub fn new(etype: ErrorType, message: &str, line: usize) -> Self {
        Self {
            etype,
            message: message.to_string(),
            line,
        }
    }

    pub fn throw(&self) {
        eprintln!("{}", self);
        exit(1);
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "[line {}] {}: {}.", self.line, self.etype, self.message)
    }
}
