use crate::analysis::node::Location;

#[derive(Debug)]
pub enum CompilerErrorKind {
    InternalError,
    TokenizerError,
    SyntaxError,
    SemanticError,
}

#[derive(Debug)]
pub struct CompilerError {
    pub kind: CompilerErrorKind,
    pub message: String,
    pub location: Option<Location>,
}
