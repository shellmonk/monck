use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum MonckError {
    SyntaxError(String),
    ParsingError(String),
    RuntimeError(String)
}

impl Display for MonckError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MonckError::SyntaxError(msg) => write!(f, "Syntax error: {msg}"),
            MonckError::ParsingError(msg) => write!(f, "Parsing error: {msg}"),
            MonckError::RuntimeError(msg) => write!(f, "Runtime error: {msg}")
        }
    }
}

impl Error for MonckError {

}