use crate::{
    ast::*,
    errors::{CompilerError, CompilerErrorKind},
    node::Span,
};

pub struct SemanticAnalyzer {}

impl SemanticAnalyzer {
    pub fn analyze(translation_unit: &TranslationUnit) -> Result<(), CompilerError> {
        for extdecl in &translation_unit.external_declarations {
            match &extdecl.node {
                ExternalDeclaration::FunctionDefinition(funcdef) => {
                    if let Statement::CompoundStatement(comp_stmt) = &funcdef.body.node {
                        for blockitem in comp_stmt {
                            match &blockitem.node {
                                BlockItem::Declaration(declaration) => {
                                    for init_decl in &declaration.init_declarators {
                                        match &init_decl.node.initializer {
                                            Some(init_node) => match &init_node.node {
                                                Initializer::AssignmentExpression(asgn_expr) => {
                                                    SemanticAnalyzer::evaluate_expr(
                                                        &asgn_expr,
                                                        &init_node.span,
                                                    )?;
                                                }
                                            },
                                            None => {}
                                        }
                                    }
                                }
                                BlockItem::Statement(_) => {}
                            }
                        }
                    } else {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::SemanticError,
                            message: "Function body must be a compound statement".to_string(),
                            location: Some(funcdef.body.span.start),
                        });
                    }
                }
                ExternalDeclaration::Declaration(_) => {}
            }
        }
        Ok(())
    }

    fn evaluate_expr(expression: &Expression, span: &Span) -> Result<TypeName, CompilerError> {
        Ok(TypeName {
            specifier_qualifier_list: Vec::new(),
            abstract_declarator: None,
        })
    }
}
