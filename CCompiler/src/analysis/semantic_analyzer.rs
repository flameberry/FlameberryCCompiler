use crate::analysis::{ast::*, node::Span};
use crate::errors::{CompilerError, CompilerErrorKind};
use crate::symboltable::SymbolTable;
use crate::typedefs::{BaseType, Type, TypeCompatibility, TypeQualifiers};

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

    pub fn analyze(&mut self, translation_unit: &mut TranslationUnit) -> Result<(), CompilerError> {
        for extdecl in &mut translation_unit.external_declarations {
            match &mut extdecl.node {
                ExternalDeclaration::FunctionDefinition(funcdef) => self.evaluate_function_def(funcdef)?,
                ExternalDeclaration::Declaration(declaration) => self.evaluate_declaration(declaration)?,
            }
        }
        Ok(())
    }

    fn evaluate_function_def(&mut self, function_def: &mut FunctionDefinition) -> Result<(), CompilerError> {
        let Statement::CompoundStatement(_) = &mut function_def.body.node else {
            return Err(CompilerError {
                kind: CompilerErrorKind::SemanticError,
                message: "Function body must be a compound statement".to_string(),
                location: Some(function_def.body.span.start),
            });
        };

        let (expected_return_type, _) = Type::from_declaration_specifiers(&function_def.specifiers)?;
        self.evaluate_stmt(&mut function_def.body.node, &expected_return_type)?;
        Ok(())
    }

    fn evaluate_stmt(&mut self, statement: &mut Statement, expected_return_type: &Type) -> Result<(), CompilerError> {
        match statement {
            Statement::CompoundStatement(compound_stmt) => {
                // Note: Assign scope ID and push it onto the current scope stack
                self.scopeidstack.push(self.scopeidstack.last().unwrap() + 1);

                for blockitem in compound_stmt {
                    match &mut blockitem.node {
                        BlockItem::Declaration(declaration) => self.evaluate_declaration(declaration)?,
                        BlockItem::Statement(stmt) => self.evaluate_stmt(stmt, expected_return_type)?,
                    }
                }

                // Pop the scope id as we have exited the function definition scope
                self.scopeidstack.pop();
            }
            Statement::ReturnStatement(return_stmt) => {
                // Check if return type is same as the expected_return_type, if not check if it's castable
                let return_type = self.evaluate_expr(&mut return_stmt.node, &return_stmt.span)?;

                match Type::compare(&return_type, expected_return_type) {
                    TypeCompatibility::Identical | TypeCompatibility::Compatible => {}

                    TypeCompatibility::ImplicitConversion { .. } => {
                        // Insert an implicit cast with target expected_return_type
                        let temp_expr = std::mem::replace(&mut return_stmt.node, Expression::Empty);

                        return_stmt.node = Expression::ImplicitCast(Box::new(ImplicitCastExpression {
                            expression: temp_expr,
                            target_type: expected_return_type.base_type.clone(),
                        }));
                    }

                    TypeCompatibility::Incompatible => {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::SemanticError,
                            message: format!(
                                "Expected return type: {}, instead got {}.",
                                expected_return_type, return_type
                            ),
                            location: Some(return_stmt.span.start),
                        })
                    }
                }
            }
            _ => todo!(),
        }
        Ok(())
    }

    fn evaluate_declaration(&mut self, declaration: &mut Declaration) -> Result<(), CompilerError> {
        for init_decl in &mut declaration.init_declarators {
            // 1. Convert set of declaration specifiers to an actual type
            let (declaration_type, storage_class) = Type::from_declaration_specifiers(&declaration.specifiers)?;

            if let Some(init_node) = &mut init_decl.node.initializer {
                match &mut init_node.node {
                    Initializer::AssignmentExpression(asgn_expr) => {
                        // Evaluate the Type of the assignment expression
                        let rhs_typeinfo = self.evaluate_expr(asgn_expr, &init_node.span)?;

                        // 2. Check if the expression type is compatible with the declaration type
                        match Type::compare(&declaration_type, &rhs_typeinfo) {
                            TypeCompatibility::Identical | TypeCompatibility::Compatible => {}

                            TypeCompatibility::ImplicitConversion { .. } => {
                                // Note: Here is a logical mistake i.e., instead of checking that
                                // the two types, declaration_type and initializer_expression_type
                                // are compatible directly, i.e., rhs is convertible to lhs
                                // directly and not a common higher precision base type, we assume
                                // just because there is an implicit conversion possible that they
                                // are compatible.

                                // 1. Extracting expression from the existing initializer enum
                                let Initializer::AssignmentExpression(temp_expr) = std::mem::replace(
                                    &mut init_node.node,
                                    Initializer::AssignmentExpression(Expression::Empty),
                                );

                                // 2. Adding an implicit cast to the assignment expression
                                init_node.node = Initializer::AssignmentExpression(Expression::ImplicitCast(Box::new(
                                    ImplicitCastExpression {
                                        target_type: declaration_type.base_type.clone(),
                                        expression: temp_expr,
                                    },
                                )));
                            }

                            TypeCompatibility::Incompatible => {
                                return Err(CompilerError {
                                    kind: CompilerErrorKind::SemanticError,
                                    message: "Assigning an incompatible type during declaration".to_string(),
                                    location: Some(init_node.span.start),
                                })
                            }
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

    fn evaluate_expr(&self, expression: &mut Expression, span: &Span) -> Result<Type, CompilerError> {
        // Tasks to be performed:
        // 1. Check whether any variables used in expression are defined in the symbol table
        // 2. Check whether the type of the variables is compatible with each other
        // 3. Check whether the type of the variables used is compatible with the operator
        // 4. If conversion is needed and possible then:
        //      a. Either insert an implicit conversion in AST (like gcc does)
        //      b. Or if it is a constant then convert it immediately

        match expression {
            Expression::BinaryOperator(binary_expr) => {
                // 1. Evaluate LHS and RHS type
                let lhs_typeinfo = self.evaluate_expr(&mut binary_expr.lhs.node, &binary_expr.lhs.span)?;
                let rhs_typeinfo = self.evaluate_expr(&mut binary_expr.rhs.node, &binary_expr.rhs.span)?;
                let composite_type: BaseType;

                // 2. Get Common Type between the operands
                match Type::compare(&lhs_typeinfo, &rhs_typeinfo) {
                    TypeCompatibility::Identical => {
                        composite_type = lhs_typeinfo.base_type.clone();
                    }

                    TypeCompatibility::ImplicitConversion { base_type } => {
                        if Self::is_assignment_operator(&binary_expr.operator.node) {
                            // Here the operator is an assignment operator, so we have to cast the
                            // rhs expression to the type of the lhs expression

                            // 1. Confirm that LHS expression is a modifiable lvalue
                            // ... For example: *ptr.member = 10;
                            //                  ^^ this expression can be a modifiable lvalue
                            if !self.is_modifiable_lvalue(&binary_expr.lhs.node) {
                                return Err(CompilerError {
                                    kind: CompilerErrorKind::SemanticError,
                                    message: "LHS must be a Modifiable LValue".to_string(),
                                    location: Some(binary_expr.lhs.span.start),
                                });
                            }

                            // 2. Cast rhs expression to the type of lhs expression
                            let rhs_expr = binary_expr.rhs.clone();

                            binary_expr.rhs.node = Expression::ImplicitCast(Box::new(ImplicitCastExpression {
                                target_type: lhs_typeinfo.base_type.clone(),
                                expression: rhs_expr.node,
                            }));

                            // 3. Set the composite type
                            composite_type = lhs_typeinfo.base_type.clone();
                        } else {
                            // Here, the operator is not an assignment operator, so we need to cast
                            // both lhs and rhs accordingly.

                            // 1. Cast LHS expression if it's not of the required type (according
                            //    to the C type promotion rules)
                            if lhs_typeinfo.base_type != base_type {
                                let lhs_expr = binary_expr.lhs.clone();

                                binary_expr.lhs.node = Expression::ImplicitCast(Box::new(ImplicitCastExpression {
                                    target_type: base_type.clone(),
                                    expression: lhs_expr.node,
                                }));
                            }

                            // 2. Cast RHS expression if it's not of the required type (according
                            //    to the C type promotion rules)
                            if rhs_typeinfo.base_type != base_type {
                                let rhs_expr = binary_expr.rhs.clone();

                                binary_expr.rhs.node = Expression::ImplicitCast(Box::new(ImplicitCastExpression {
                                    target_type: base_type.clone(),
                                    expression: rhs_expr.node,
                                }));
                            }

                            // 3. Set the composite type
                            composite_type = base_type;
                        }
                    }

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
                if !Self::is_operand_type_compatible_with_operator(&composite_type, &binary_expr.operator.node) {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SemanticError,
                        message: format!(
                            "Incompatible operand ({}) usage with operator ({:?})",
                            composite_type, binary_expr.operator.node
                        ),
                        location: Some(binary_expr.operator.span.start),
                    });
                }

                // Finally return the evaluated type of the expression
                // Note: Missing proper handling of qualifiers
                Ok(Type {
                    base_type: composite_type,
                    qualifiers: lhs_typeinfo.qualifiers,
                })
            }
            Expression::Identifier(idname) => {
                // Tasks to be performed:
                // 1. Check if idname is a valid symbol in the symboltable
                // 2. Convert TypeName to TypeInfo
                match self.symboltableref.lookup(idname, *self.scopeidstack.last().unwrap()) {
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
                assert!(!comma_exprs.is_empty(), "Comma Expression vector can't be empty");

                let mut first = true;
                let mut typeinfo = Type {
                    base_type: BaseType::Void,
                    qualifiers: TypeQualifiers::default(),
                };

                // In case of initializer expressions, we should check whether all comma
                // expressions are of the same type or not, but in case of generic expressions like
                // 4 + 5, false;
                // Above is a valid statement
                for comma_expr in comma_exprs.iter_mut() {
                    if first {
                        typeinfo = self.evaluate_expr(&mut comma_expr.node, &comma_expr.span)?;
                        first = false;
                    }
                }

                // For now this function returns the first comma expression's evaluated type
                Ok(typeinfo)
            }
            _ => todo!(),
        }
    }

    fn is_lvalue(&self, expression: &Expression) -> bool {
        // TODO: Update function to recognize complex lvalue expressions
        match expression {
            Expression::Identifier(idname) => self
                .symboltableref
                .lookup(idname, *self.scopeidstack.last().unwrap())
                .is_some(),
            Expression::Constant(_) => false,
            _ => todo!(),
        }
    }

    fn is_modifiable_lvalue(&self, expression: &Expression) -> bool {
        // TODO: Update function to recognize complex lvalue expressions
        match expression {
            Expression::Identifier(idname) => self
                .symboltableref
                .lookup(idname, *self.scopeidstack.last().unwrap())
                .is_some_and(|symbol| !symbol.typeinfo.qualifiers.is_const),
            Expression::Constant(_) => false,
            _ => todo!(),
        }
    }

    /// Returns the operand type that is expected for the given binary operator
    fn is_operand_type_compatible_with_operator(operand_type: &BaseType, operator: &BinaryOperator) -> bool {
        match (operator, operand_type) {
            // Arithmetic operators: +, -, *, /, %
            (
                BinaryOperator::Plus | BinaryOperator::Minus | BinaryOperator::Multiply | BinaryOperator::Divide,
                BaseType::Int { .. }
                | BaseType::Short { .. }
                | BaseType::Long { .. }
                | BaseType::Float
                | BaseType::Double,
            ) => true,

            (BinaryOperator::Modulo, BaseType::Int { .. } | BaseType::Short { .. } | BaseType::Long { .. }) => true, // % only works with integral types

            // Bitwise operators: &, |, ^, <<, >>
            (
                BinaryOperator::BitwiseAnd
                | BinaryOperator::BitwiseOr
                | BinaryOperator::BitwiseXor
                | BinaryOperator::ShiftLeft
                | BinaryOperator::ShiftRight,
                BaseType::Int { .. } | BaseType::Short { .. } | BaseType::Long { .. } | BaseType::Char { .. },
            ) => true,

            // Comparison operators: ==, !=, <, >, <=, >=
            (
                BinaryOperator::Equals
                | BinaryOperator::NotEquals
                | BinaryOperator::Less
                | BinaryOperator::Greater
                | BinaryOperator::LessOrEqual
                | BinaryOperator::GreaterOrEqual,
                BaseType::Int { .. }
                | BaseType::Short { .. }
                | BaseType::Long { .. }
                | BaseType::Float
                | BaseType::Double
                | BaseType::Char { .. }
                | BaseType::Bool,
            ) => true,

            // Logical operators: &&, ||
            (BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr, BaseType::Bool) => true,

            _ => Self::is_assignment_operator(operator),
        }
    }

    fn is_assignment_operator(op: &BinaryOperator) -> bool {
        matches!(
            op,
            BinaryOperator::Assign
                | BinaryOperator::AssignMultiply
                | BinaryOperator::AssignDivide
                | BinaryOperator::AssignModulo
                | BinaryOperator::AssignPlus
                | BinaryOperator::AssignMinus
                | BinaryOperator::AssignShiftLeft
                | BinaryOperator::AssignShiftRight
                | BinaryOperator::AssignBitwiseAnd
                | BinaryOperator::AssignBitwiseXor
                | BinaryOperator::AssignBitwiseOr
        )
    }
}
