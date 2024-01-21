use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum CompilerError {
    SyntaxError(String),
    UnexpectedTokenError(String),
}

impl Error for CompilerError {}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CompilerError::SyntaxError(err) | CompilerError::UnexpectedTokenError(err) => {
                write!(f, "[Compiler Error]: {}", err)
            }
        }
    }
}
