use core::fmt;
use std::error::Error;

use crate::node::TokenPosition;

#[derive(Debug)]
pub enum CompilerErrorKind {
    TokenizerError,
    SyntaxError,
    SemanticError,
}

#[derive(Debug)]
pub struct CompilerError {
    pub kind: CompilerErrorKind,
    pub message: String,
    pub location: Option<TokenPosition>, // For now the location only contains the character index optionally
}

impl Error for CompilerError {}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = match self.location {
            Some(loc) => format!("{}: ", loc),
            None => "".to_string(),
        };

        match self.kind {
            CompilerErrorKind::TokenizerError => {
                write!(f, "{}[Tokenizer Error]: {}", prefix, self.message)
            }
            CompilerErrorKind::SyntaxError => {
                write!(f, "{}[Syntax Error]: {}", prefix, self.message)
            }
            CompilerErrorKind::SemanticError => {
                write!(f, "{}[Semantic Error]: {}", prefix, self.message)
            }
        }
    }
}
