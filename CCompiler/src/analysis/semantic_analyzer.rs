use crate::analysis::{ast::*, node::Span};
use crate::errors::{CompilerError, CompilerErrorKind};
use crate::symboltable::SymbolTable;

// TODOS: Store the scope ID somewhere in the AST probably
// ...to ensure that every time we need to find a symbol from the AST we can lookup using the scope ID

pub struct SemanticAnalyzer<'a> {
    symboltableref: &'a mut SymbolTable,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn analyze(translation_unit: &TranslationUnit) -> Result<(), CompilerError> {
        for extdecl in &translation_unit.external_declarations {
            match &extdecl.node {
                ExternalDeclaration::FunctionDefinition(funcdef) => {
                    if let Statement::CompoundStatement(comp_stmt) = &funcdef.body.node {
                        // Note: Assign scope ID and push it onto the current scope stack
                        for blockitem in comp_stmt {
                            match &blockitem.node {
                                BlockItem::Declaration(declaration) => {
                                    for init_decl in &declaration.init_declarators {
                                        match &init_decl.node.initializer {
                                            Some(init_node) => match &init_node.node {
                                                Initializer::AssignmentExpression(asgn_expr) => {
                                                    // Step 1: Check if the expression type is compatible with the declaration type
                                                    SemanticAnalyzer::evaluate_expr(
                                                        &asgn_expr,
                                                        &init_node.span,
                                                    )?;

                                                    // Step 2: Insert into the symbol table this declaration with it's details and scope ID
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
        // Functions to be performed:
        // 1. Check whether any variables used in expression are defined in the symbol table
        // 2. Check whether the type of the variables used is appropriate
        // 3. If conversion is needed and possible then:
        //      a. Either insert an implicit conversion in AST (like gcc does)
        //      b. Or if it is a constant then convert it immediately

        Ok(TypeName {
            specifier_qualifier_list: Vec::new(),
            abstract_declarator: None,
        })
    }
}
