use std::fmt::{Display, Formatter, Result};

use crate::general::object::Object;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number,

    Identifier,

    Let,
    If,
    Else,
    While,

    Nil,
    True,
    False,
    String,

    Print,

    OpenParen,
    CloseParen,
    OpenCurly,
    CloseCurly,

    Plus,
    Minus,
    Multiplication,
    Division,
    Modulo,

    Comma,
    Semicolon,

    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    BitwiseAnd,
    And,
    BitwiseOr,
    Or,

    EOF,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::Number => write!(f, "Number"),

            Self::Identifier => write!(f, "Identifier"),

            Self::Let => write!(f, "Let"),
            Self::If => write!(f, "If"),
            Self::Else => write!(f, "Else"),
            Self::While => write!(f, "While"),

            Self::Nil => write!(f, "Nil"),
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
            Self::String => write!(f, "String"),

            Self::Print => write!(f, "Print"),

            Self::OpenParen => write!(f, "OpenParen"),
            Self::CloseParen => write!(f, "CloseParen"),
            Self::OpenCurly => write!(f, "OpenCurly"),
            Self::CloseCurly => write!(f, "CloseCurly"),

            Self::Plus => write!(f, "Plus"),
            Self::Minus => write!(f, "Minus"),
            Self::Multiplication => write!(f, "Multiplication"),
            Self::Division => write!(f, "Division"),
            Self::Modulo => write!(f, "Modulo"),

            Self::Comma => write!(f, "Comma"),
            Self::Semicolon => write!(f, "Semicolon"),

            Self::Equal => write!(f, "Equal"),
            Self::EqualEqual => write!(f, "EqualEqual"),
            Self::Bang => write!(f, "Bang"),
            Self::BangEqual => write!(f, "BangEqual"),
            Self::Greater => write!(f, "Greater"),
            Self::GreaterEqual => write!(f, "GreaterEqual"),
            Self::Less => write!(f, "Less"),
            Self::LessEqual => write!(f, "LessEqual"),

            Self::BitwiseAnd => write!(f, "BitwiseAnd"),
            Self::And => write!(f, "And"),
            Self::BitwiseOr => write!(f, "BitwiseOr"),
            Self::Or => write!(f, "Or"),

            Self::EOF => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Object,
    pub line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: &str, literal: Object, line: usize) -> Self {
        Self {
            ttype,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Token '{}' of type '{}' (Object: {}) in line {}",
            self.lexeme, self.ttype, self.literal, self.line
        )
    }
}
