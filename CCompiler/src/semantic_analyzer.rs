use crate::{ast::*, errors::CompilerError};

pub struct SemanticAnalyzer {}

impl SemanticAnalyzer {
    pub fn analyze(translation_unit: &TranslationUnit) -> Result<(), CompilerError> {
        for extdecl in &translation_unit.external_declarations {
            match extdecl.node {
                ExternalDeclaration::FunctionDefinition(_) => todo!(),
                ExternalDeclaration::Declaration(_) => todo!()
            }
        }
        Ok(())
    }
}
