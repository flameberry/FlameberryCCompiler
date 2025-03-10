use crate::analysis::{ast::*, node::Span};
use crate::errors::{CompilerError, CompilerErrorKind};
use crate::symboltable::SymbolTable;
use crate::typedefs::{BaseType, Type, TypeCompatibility};

// TODOS: Store the scope ID somewhere in the AST probably
// ...to ensure that every time we need to find a symbol from the AST we can lookup using the scope ID

pub struct SemanticAnalyzer<'a> {
    symboltableref: &'a mut SymbolTable,
    scopeidstack: Vec<u32>,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(symboltableref: &'a mut SymbolTable) -> Self {
        Self {
            symboltableref,
            scopeidstack: vec![0], // 0 represents global scope
        }
    }

    pub fn analyze(&mut self, translation_unit: &TranslationUnit) -> Result<(), CompilerError> {
        for extdecl in &translation_unit.external_declarations {
            match &extdecl.node {
                ExternalDeclaration::FunctionDefinition(funcdef) => {
                    if let Statement::CompoundStatement(comp_stmt) = &funcdef.body.node {
                        // Note: Assign scope ID and push it onto the current scope stack
                        self.scopeidstack
                            .push(self.scopeidstack.last().unwrap() + 1);

                        for blockitem in comp_stmt {
                            match &blockitem.node {
                                BlockItem::Declaration(declaration) => {
                                    self.evaluate_declaration(declaration)?
                                }
                                BlockItem::Statement(_) => {}
                            }
                        }

                        // Pop the scope id as we have exited the function definition scope
                        self.scopeidstack.pop();
                    } else {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::SemanticError,
                            message: "Function body must be a compound statement".to_string(),
                            location: Some(funcdef.body.span.start),
                        });
                    }
                }
                ExternalDeclaration::Declaration(declaration) => {
                    self.evaluate_declaration(declaration)?
                }
            }
        }
        Ok(())
    }

    fn evaluate_declaration(&mut self, declaration: &Declaration) -> Result<(), CompilerError> {
        for init_decl in &declaration.init_declarators {
            // 1. Convert set of declaration specifiers to an actual type
            let (declaration_type, storage_class) =
                Type::from_declaration_specifiers(&declaration.specifiers)?;

            if let Some(init_node) = &init_decl.node.initializer {
                match &init_node.node {
                    Initializer::AssignmentExpression(asgn_expr) => {
                        // Evaluate the Type of the assignment expression
                        let rhs_typeinfo = self.evaluate_expr(asgn_expr, &init_node.span)?;

                        // 2. Check if the expression type is compatible with the declaration type
                        match Type::compare(&declaration_type, &rhs_typeinfo) {
                            TypeCompatibility::Identical => {}
                            TypeCompatibility::Incompatible => {
                                return Err(CompilerError {
                                    kind: CompilerErrorKind::SemanticError,
                                    message: "Assigning an incompatible type during declaration"
                                        .to_string(),
                                    location: Some(init_node.span.start),
                                })
                            }
                            _ => todo!(),
                        }
                    }
                }
            }

            // 3. Insert into the symbol table this declaration with it's details and scope ID
            match &init_decl.node.declarator.node {
                Declarator::DirectDeclarator(idname) => {
                    self.symboltableref.insert(
                        idname,
                        *self.scopeidstack.last().unwrap(),
                        declaration_type,
                        storage_class,
                        None,
                    )?;
                }
                Declarator::FunctionDeclarator(_) => {
                    todo!()
                }
            }
        }
        Ok(())
    }

    fn evaluate_expr(&self, expression: &Expression, span: &Span) -> Result<Type, CompilerError> {
        // Tasks to be performed:
        // 1. Check whether any variables used in expression are defined in the symbol table
        // 2. Check whether the type of the variables is compatible with each other
        // 3. Check whether the type of the variables used is compatible with the operator
        // 4. If conversion is needed and possible then:
        //      a. Either insert an implicit conversion in AST (like gcc does)
        //      b. Or if it is a constant then convert it immediately

        match &expression {
            Expression::BinaryOperator(binary_expr) => {
                // 1. Evaluate LHS and RHS type
                let lhs_typeinfo =
                    self.evaluate_expr(&binary_expr.lhs.node, &binary_expr.lhs.span)?;
                let rhs_typeinfo =
                    self.evaluate_expr(&binary_expr.rhs.node, &binary_expr.rhs.span)?;

                // 2. Get Common Type between the operands
                match Type::compare(&lhs_typeinfo, &rhs_typeinfo) {
                    TypeCompatibility::Identical => {}
                    TypeCompatibility::Incompatible => {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::SemanticError,
                            message: format!(
                                "Incompatible Types provided to binary expression: {:?}",
                                binary_expr.operator.node
                            ),
                            location: Some(binary_expr.operator.span.start),
                        })
                    }
                    _ => todo!(),
                }

                // 3. Check if the composite type is compatible with the type of operator used
                // TODO: Implement this

                // Finally return the evaluated type of the expression
                Ok(lhs_typeinfo)
            }
            Expression::Identifier(idname) => {
                // Tasks to be performed:
                // 1. Check if idname is a valid symbol in the symboltable
                // 2. Convert TypeName to TypeInfo
                match self
                    .symboltableref
                    .lookup(idname, *self.scopeidstack.last().unwrap())
                {
                    Some(symboldef) => Ok(symboldef.typeinfo.clone()),
                    None => Err(CompilerError {
                        kind: CompilerErrorKind::SemanticError,
                        message: format!("Undefined symbol in the given scope: {}", idname),
                        location: Some(span.start),
                    }),
                }
            }
            Expression::Constant(constant) => Ok(BaseType::from_constant(constant)),
            Expression::Comma(comma_exprs) => {
                assert!(
                    !comma_exprs.is_empty(),
                    "Comma Expression vector can't be empty"
                );

                // In case of initializer expressions, we should check whether all comma
                // expressions are of the same type or not, but in case of generic expressions like
                // 4 + 5, false;
                // Above is a valid statement
                for comma_expr in comma_exprs.iter() {
                    self.evaluate_expr(&comma_expr.node, &comma_expr.span)?;
                }

                // For now this function returns the first comma expression's evaluated type
                self.evaluate_expr(
                    &comma_exprs.first().unwrap().node,
                    &comma_exprs.first().unwrap().span,
                )
            }
            _ => todo!(),
        }
    }

    /// Returns the operand type that is expected for the given binary operator
    fn get_binary_operand_type(operator: BinaryOperator) -> Option<BaseType> {
        todo!()
    }
}
