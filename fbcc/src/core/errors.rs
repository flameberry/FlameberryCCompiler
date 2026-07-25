use crate::analysis::node::{Location, Span};

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

#[derive(Debug)]
pub enum DiagnosticKind {
    Warning,
}

#[derive(Debug)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub span: Option<Span>,
}

pub trait VecExtensionDiagnosticHelpers {
    fn warning(&mut self, msg: String, span: Option<Span>);
}

impl VecExtensionDiagnosticHelpers for Vec<Diagnostic> {
    fn warning(&mut self, msg: String, span: Option<Span>) {
        self.push(Diagnostic {
            kind: DiagnosticKind::Warning,
            message: msg,
            span,
        });
    }
}
