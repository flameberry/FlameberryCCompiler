use std::iter::zip;

use crate::analysis::ast::ExpressionValueType::{LValue, ModifiableLValue, RValue};
use crate::analysis::ast::UnaryOperator::PostDecrement;
use crate::analysis::{ast::*, node::Span};
use crate::core::errors::{CompilerError, CompilerErrorKind, Diagnostic, VecExtensionDiagnosticHelpers};
use crate::core::symboltable::{SymbolDefinition, SymbolTable};
use crate::core::typedefs::{AssignmentConversionResult, DataType, IntegerRank, Type, TypeQualifiers};

use super::node::Node;

pub struct SemanticAnalyzer<'a> {
    symboltableref: &'a mut SymbolTable,
    scopeidstack: Vec<u32>,
    counter: u32,
    num_loops_or_switches: u32,
    diagnostics: &'a mut Vec<Diagnostic>,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(symboltableref: &'a mut SymbolTable, diagnostics: &'a mut Vec<Diagnostic>) -> Self {
        Self {
            symboltableref,
            scopeidstack: vec![0], // 0 represents global scope
            counter: 1,
            num_loops_or_switches: 0,
            diagnostics,
        }
    }

    fn lookup_innermost_scope_symbol(&self, name: &str) -> Option<&SymbolDefinition> {
        // The most idiomatic way to do this, i.e., go from the current scope to the outermost
        // scope and search for the symbol definition in each scope. We stop at the innermost scope
        // that we find the symbol with the given name.
        // Note: There are possibly better ways to do this

        for scopeid in self.scopeidstack.iter().rev() {
            if let Some(symboldef) = self.symboltableref.lookup(name, *scopeid) {
                return Some(symboldef);
            }
        }

        None
    }

    fn push_scope(&mut self) {
        self.scopeidstack.push(self.counter);
        self.counter += 1;
    }

    fn pop_scope(&mut self) {
        self.scopeidstack.pop();
    }

    pub fn analyze(&mut self, translation_unit: &mut TranslationUnit) -> Result<(), CompilerError> {
        for extdecl in &mut translation_unit.external_declarations {
            match &mut extdecl.node {
                ExternalDeclaration::FunctionDefinition(funcdef) => self.validate_function_def(funcdef)?,
                ExternalDeclaration::Declaration(declaration) => self.validate_declaration(declaration)?,
            }
        }
        Ok(())
    }

    fn validate_function_def(&mut self, function_def: &mut FunctionDefinition) -> Result<(), CompilerError> {
        let Statement::CompoundStatement(compound_stmt) = &mut function_def.body.node else {
            return Err(CompilerError {
                kind: CompilerErrorKind::SemanticError,
                message: "Function body must be a compound statement".to_string(),
                span: Some(function_def.body.span),
            });
        };

        // Get the return type from declaration specifiers using the function definition
        let (expected_return_type, _) = Type::from_declaration_specifiers(&function_def.specifiers)?;

        // Note the scope outside the function
        let scopeid: u32 = *self.scopeidstack.last().unwrap();
        let mut param_types: Vec<Type> = Vec::new();

        assert!(
            scopeid == 0,
            "Function definition found not inside a global scope: {}",
            scopeid
        );

        self.push_scope();

        // Insert params as symbols in the symbol table belonging to the function scope
        for param in &function_def.declarator.node.parameters {
            let (param_type, param_storage_class) = Type::from_declaration_specifiers(&param.node.specifiers)?;

            // 1. Push the param_type into the param_types vector, which will be needed for the
            //    function symbol definition info
            param_types.push(param_type.clone());

            // 2. Insert the individual param as a separate symbol, to help evaluation of the
            //    function body
            if let Some(declarator) = &param.node.declarator {
                match &declarator.node {
                    Declarator::DirectDeclarator(idname) => {
                        self.symboltableref.insert(
                            idname.as_str(),
                            *self.scopeidstack.last().unwrap(),
                            param_type,
                            param_storage_class,
                            None,
                        )?;
                    }
                    _ => todo!(),
                }
            }
        }

        // Construct the function signature
        let function_type = Type::new(DataType::Function {
            return_type: Box::new(expected_return_type.clone()),
            parameters: param_types,
        });

        // Insert the function itself as a symbol into the symbol table
        self.symboltableref.insert(
            function_def.declarator.node.identifier.as_str(),
            scopeid,
            function_type,
            0,
            None,
        )?;

        for blockitem in compound_stmt {
            match &mut blockitem.node {
                BlockItem::Declaration(declaration) => self.validate_declaration(declaration)?,
                BlockItem::Statement(stmt) => self.validate_statement(stmt, &expected_return_type)?,
            }
        }

        self.pop_scope();
        Ok(())
    }

    fn validate_statement(
        &mut self,
        statement: &mut Statement,
        expected_return_type: &Type,
    ) -> Result<(), CompilerError> {
        match statement {
            Statement::CompoundStatement(compound_stmt) => {
                self.push_scope();

                for blockitem in compound_stmt {
                    match &mut blockitem.node {
                        BlockItem::Declaration(declaration) => self.validate_declaration(declaration)?,
                        BlockItem::Statement(stmt) => self.validate_statement(stmt, expected_return_type)?,
                    }
                }

                self.pop_scope();
            }

            Statement::ReturnStatement(return_stmt) => {
                // Check if return type is same as the expected_return_type, if not check if it's castable
                let (return_type, _) = self.validate_expr(&mut return_stmt.node, &return_stmt.span)?;

                match Type::check_assignment_conversion(expected_return_type, &return_type)? {
                    AssignmentConversionResult::Identical => {}
                    result => {
                        Self::implicit_cast(return_stmt, &expected_return_type.datatype);

                        if let AssignmentConversionResult::CastWithWarning(warning) = result {
                            self.diagnostics.warning(warning, Some(return_stmt.span));
                        }
                    }
                }
            }

            Statement::ExpressionStatement(expr_node) => {
                if let Some(expression) = expr_node {
                    self.validate_expr(&mut expression.node, &expression.span)?;
                }
            }

            Statement::ForStatement(for_stmt) => {
                // This is done for verification of break and continue statements
                self.num_loops_or_switches += 1;

                // So understand this, I'm considering a for-loop itself consisting of 2 scopes:
                //
                // 1. The outer scope which contains any definitions that may be made in the
                //    initializer part of the for statement.
                // 2. The inner scope which contains the code that is to be executed repeatedly
                //    till the for condition is true.
                //
                // Reason: Because of this any initializer variable will be visible to the code
                // inside for-loop, but any variable inside for-loop won't be visible to the
                // for-condition or for-step statement.
                self.push_scope();

                // 1. Verify for loop initializer statement
                match &mut for_stmt.initializer.node {
                    ForInitializer::Empty => {}
                    ForInitializer::Expression(expression) => {
                        self.validate_expr(expression, &for_stmt.initializer.span)?;
                    }
                    ForInitializer::Declaration(declaration) => self.validate_declaration(declaration)?,
                }

                // 2. Evaluate condition and check if the type can evaluate into a boolean
                if let Some(condition) = &mut for_stmt.condition {
                    let (condition_type, _) = self.validate_expr(&mut condition.node, &condition.span)?;

                    if Type::is_boolean_compatible(&condition_type) {
                        Self::implicit_cast_to_bool(condition, &condition_type);
                    } else {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::SemanticError,
                            message: format!("Expected boolean expression, instead got {}", condition_type),
                            span: Some(condition.span),
                        });
                    }
                }

                // 3. Evaluate step statement of for-loop
                if let Some(step_expr) = &mut for_stmt.step {
                    self.validate_expr(&mut step_expr.node, &step_expr.span)?;
                }

                // 4. Evaluate the for-loop body
                self.validate_statement(&mut for_stmt.statement.node, expected_return_type)?;

                // Pop the scope id as we have exited the for-loop scope
                self.pop_scope();

                // This is done for verification of break and continue statements
                self.num_loops_or_switches -= 1;
            }

            Statement::WhileStatement(while_stmt) | Statement::DoWhileStatement(while_stmt) => {
                // 1. Evaluate condition and check if the type can evaluate into a boolean
                let (condition_type, _) =
                    self.validate_expr(&mut while_stmt.condition.node, &while_stmt.condition.span)?;

                if Type::is_boolean_compatible(&condition_type) {
                    Self::implicit_cast_to_bool(&mut while_stmt.condition, &condition_type);
                } else {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SemanticError,
                        message: format!("Expected boolean expression, instead got {}", condition_type),
                        span: Some(while_stmt.condition.span),
                    });
                }

                self.num_loops_or_switches += 1;
                // 2. Evaluate the while-loop body
                self.validate_statement(&mut while_stmt.statement.node, expected_return_type)?;
                self.num_loops_or_switches -= 1;
            }

            Statement::IfStatement(if_stmt) => {
                // 1. Evaluate condition and check if the type can evaluate into a boolean
                let (condition_type, _) = self.validate_expr(&mut if_stmt.condition.node, &if_stmt.condition.span)?;

                if Type::is_boolean_compatible(&condition_type) {
                    Self::implicit_cast_to_bool(&mut if_stmt.condition, &condition_type);
                } else {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SemanticError,
                        message: format!("Expected boolean expression, instead got {}", condition_type),
                        span: Some(if_stmt.condition.span),
                    });
                }

                // 2. Evaluate the if-statement body
                self.validate_statement(&mut if_stmt.if_block.node, expected_return_type)?;

                // 3. Evaluate the else-statement body if it exists
                if let Some(else_block) = &mut if_stmt.else_block {
                    self.validate_statement(&mut else_block.node, expected_return_type)?;
                }
            }

            Statement::SwitchStatement(switch_stmt) => {
                self.num_loops_or_switches += 1;
                self.validate_statement(&mut switch_stmt.statement.node, expected_return_type)?;
                self.num_loops_or_switches -= 1;
            }

            Statement::BreakStatement | Statement::ContinueStatement => {
                if self.num_loops_or_switches == 0 {
                    let keyword = match statement {
                        Statement::BreakStatement => "break",
                        Statement::ContinueStatement => "continue",
                        _ => unreachable!(), // Only these two arms are possible
                    };

                    return Err(CompilerError {
                        kind: CompilerErrorKind::SemanticError,
                        message: format!("{} statement not allowed outside of a loop", keyword),
                        span: None,
                    });
                }
            }

            _ => todo!(),
        }
        Ok(())
    }

    fn validate_declaration(&mut self, declaration: &mut Declaration) -> Result<(), CompilerError> {
        for init_decl in &mut declaration.init_declarators {
            // 1. Convert set of declaration specifiers to an actual type
            let (declaration_type, storage_class) = Type::from_declaration_specifiers(&declaration.specifiers)?;

            if let Some(init_node) = &mut init_decl.node.initializer {
                match &mut init_node.node {
                    Initializer::AssignmentExpression(asgn_expr) => {
                        // Evaluate the Type of the assignment expression
                        let (rhs_typeinfo, _) = self.validate_expr(asgn_expr, &init_node.span)?;

                        // 2. Check if the expression type is compatible with the declaration type
                        match Type::check_assignment_conversion(&declaration_type, &rhs_typeinfo)? {
                            AssignmentConversionResult::Identical => {}

                            result => {
                                // 1. Extracting expression from the existing initializer enum
                                let Initializer::AssignmentExpression(temp_expr) = std::mem::replace(
                                    &mut init_node.node,
                                    Initializer::AssignmentExpression(Expression::Empty),
                                );

                                // 2. Adding an implicit cast to the assignment expression
                                init_node.node = Initializer::AssignmentExpression(Expression::ImplicitCast(Box::new(
                                    ImplicitCastExpression {
                                        target_type: declaration_type.datatype.clone(),
                                        expression: temp_expr,
                                    },
                                )));

                                if let AssignmentConversionResult::CastWithWarning(warning) = result {
                                    self.diagnostics.warning(warning, Some(init_node.span));
                                }
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

    fn validate_expr(
        &mut self,
        expression: &mut Expression,
        span: &Span,
    ) -> Result<(Type, ExpressionValueType), CompilerError> {
        // Tasks to be performed:
        // 1. Check whether any variables used in expression are defined in the symbol table
        // 2. Check whether the type of the variables is compatible with each other
        // 3. Check whether the type of the variables used is compatible with the operator
        // 4. If conversion is needed and possible then:
        //      a. Either insert an implicit conversion in AST (like gcc does)
        //      b. Or if it is a constant then convert it immediately

        match expression {
            Expression::Empty => Ok((Type::new(DataType::Void), RValue)),
            Expression::Identifier(idname) => {
                // Tasks to be performed:
                // 1. Check if idname is a valid symbol in the symboltable
                // 2. Convert TypeName to TypeInfo
                match self.lookup_innermost_scope_symbol(idname) {
                    Some(symboldef) => {
                        if symboldef.typeinfo.qualifiers.is_const {
                            Ok((symboldef.typeinfo.clone(), LValue))
                        } else {
                            Ok((symboldef.typeinfo.clone(), ModifiableLValue))
                        }
                    }
                    None => Err(CompilerError {
                        kind: CompilerErrorKind::SemanticError,
                        message: format!(
                            "Unable to find the symbol named '{}' in any of the reachable scopes.",
                            idname
                        ),
                        span: Some(*span),
                    }),
                }
            }

            Expression::Constant(constant) => Ok((Type::from_constant(constant), RValue)),

            Expression::StringLiteral(literal) => Ok((
                Type::new(DataType::Array {
                    element_type: Box::new(Type::new(DataType::new_integer(IntegerRank::Char, true))),
                    size: Some(literal.len() + 1),
                }),
                LValue,
            )),

            Expression::UnaryOperator(unary_expr) => {
                let (operand_type, operand_val_type) =
                    self.validate_expr(&mut unary_expr.operand.node, &unary_expr.operand.span)?;

                // check if type is compatible with the unary operator
                if !operand_type
                    .datatype
                    .is_compatible_with_unary_operator(&unary_expr.operator.node)
                {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SemanticError,
                        message: format!(
                            "operand type {} not compatible with {}",
                            operand_type, unary_expr.operator.node
                        ),
                        span: Some(unary_expr.operand.span),
                    });
                }

                match &unary_expr.operator.node {
                    UnaryOperator::Minus | UnaryOperator::Plus | UnaryOperator::Complement | UnaryOperator::Negate => {
                        if !operand_type.datatype.is_integer() {
                            return Err(CompilerError {
                                kind: CompilerErrorKind::SemanticError,
                                message: format!(
                                    "unary operator {} is only supported on integer types for now",
                                    unary_expr.operator.node
                                ),
                                span: Some(unary_expr.operator.span),
                            });
                        }

                        Ok((
                            if matches!(&unary_expr.operator.node, UnaryOperator::Negate) {
                                Type {
                                    datatype: DataType::new_integer(IntegerRank::Int, true),
                                    qualifiers: operand_type.qualifiers,
                                }
                            } else {
                                operand_type
                            },
                            RValue,
                        ))
                    }

                    inc_dec_op @ (UnaryOperator::PreIncrement
                    | UnaryOperator::PostIncrement
                    | UnaryOperator::PreDecrement
                    | UnaryOperator::PostDecrement) => {
                        // ensure that operand is a modifiable lvalue
                        if !matches!(operand_val_type, ModifiableLValue) {
                            Err(CompilerError {
                                kind: CompilerErrorKind::SemanticError,
                                message: format!("operand to {} must be a modifiable lvalue", inc_dec_op),
                                span: Some(unary_expr.operator.span),
                            })
                        } else {
                            Ok((operand_type, RValue))
                        }
                    }

                    op => {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::SemanticError,
                            message: format!("unary operator {} is not supported yet", op),
                            span: Some(unary_expr.operator.span),
                        })
                    }
                }
            }

            Expression::BinaryOperator(binary_expr) => {
                // 1. evaluate lhs and rhs expression types
                let (lhs_typeinfo, _) = self.validate_expr(&mut binary_expr.lhs.node, &binary_expr.lhs.span)?;
                let (rhs_typeinfo, _) = self.validate_expr(&mut binary_expr.rhs.node, &binary_expr.rhs.span)?;

                // 2. usual arithmetic conversions
                let uac_datatype = Type::common_datatype_for_uac(&lhs_typeinfo, &rhs_typeinfo)?;

                if uac_datatype != lhs_typeinfo.datatype {
                    Self::implicit_cast(&mut binary_expr.lhs, &uac_datatype);
                }

                if uac_datatype != rhs_typeinfo.datatype {
                    Self::implicit_cast(&mut binary_expr.rhs, &uac_datatype);
                }

                // 3. check if the uac type is compatible with the type of operator used
                if !uac_datatype.is_compatible_with_binary_operator(&binary_expr.operator.node) {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SemanticError,
                        message: format!(
                            "Incompatible operand ({}) usage with operator ({:?})",
                            uac_datatype, binary_expr.operator.node
                        ),
                        span: Some(binary_expr.operator.span),
                    });
                }

                // 4. return evaluated type and drop qualifiers as binary expression is always an rvalue
                Ok((Type::new(uac_datatype), RValue))
            }

            Expression::AssignOperator(assign_expr) => {
                // 1. evaluate lhs and rhs expression types
                let (lhs_type, lhs_val_type) = self.validate_expr(&mut assign_expr.lhs.node, &assign_expr.lhs.span)?;
                let (rhs_type, _) = self.validate_expr(&mut assign_expr.rhs.node, &assign_expr.rhs.span)?;

                if !matches!(lhs_val_type, ModifiableLValue) {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SemanticError,
                        message: "lhs of an assignment expression must be a modifiable lvalue".to_string(),
                        span: Some(assign_expr.lhs.span),
                    });
                }

                let final_rhs_type: Type;

                if let Some(underlying_binary_op) = assign_expr.operator.node.underlying_binary_op() {
                    // 2. usual arithmetic conversions
                    let uac_datatype = Type::common_datatype_for_uac(&lhs_type, &rhs_type)?;

                    if uac_datatype != rhs_type.datatype {
                        Self::implicit_cast(&mut assign_expr.rhs, &uac_datatype);
                    }

                    // 3. check if the composite type is compatible with the type of operator used
                    if !uac_datatype.is_compatible_with_binary_operator(&underlying_binary_op) {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::SemanticError,
                            message: format!(
                                "Incompatible operand ({}) usage with operator ({:?})",
                                uac_datatype, assign_expr.operator.node
                            ),
                            span: Some(assign_expr.operator.span),
                        });
                    }

                    final_rhs_type = Type::new(uac_datatype);
                    assign_expr.uac_type = Some(final_rhs_type.clone());
                } else {
                    final_rhs_type = rhs_type.clone();
                }

                // 4. now check assignment conversion
                match Type::check_assignment_conversion(&lhs_type, &final_rhs_type)? {
                    AssignmentConversionResult::Identical => {}
                    result => {
                        assign_expr.should_cast = true;

                        if let AssignmentConversionResult::CastWithWarning(warning) = result {
                            self.diagnostics.warning(warning, Some(assign_expr.operator.span));
                        }
                    }
                }

                // 5. return evaluated type and drop qualifiers as binary expression is always an rvalue
                Ok((Type::new(lhs_type.datatype), RValue))
            }

            Expression::TernaryOperator(ternary_expr) => {
                // 1. evaluate type of the condition expression
                let (condition_type, _) =
                    self.validate_expr(&mut ternary_expr.condition.node, &ternary_expr.condition.span)?;

                // 2. check if condition expression type is boolean compatible
                if Type::is_boolean_compatible(&condition_type) {
                    // 3. cast to bool if not already
                    Self::implicit_cast_to_bool(&mut ternary_expr.condition, &condition_type);

                    // 4. evaluate types of if expression and else expression
                    let (if_type, _) =
                        self.validate_expr(&mut ternary_expr.if_expr.node, &ternary_expr.if_expr.span)?;
                    let (else_type, _) =
                        self.validate_expr(&mut ternary_expr.else_expr.node, &ternary_expr.else_expr.span)?;

                    // 5. if if_type and else_type are arithmetic then perform uac
                    if if_type.datatype.is_arithmetic() && else_type.datatype.is_arithmetic() {
                        let uac_datatype = Type::common_datatype_for_uac(&if_type, &else_type)?;

                        if uac_datatype != if_type.datatype {
                            Self::implicit_cast(&mut ternary_expr.if_expr, &uac_datatype);
                        }

                        if uac_datatype != else_type.datatype {
                            Self::implicit_cast(&mut ternary_expr.else_expr, &uac_datatype);
                        }

                        Ok((Type::new(uac_datatype), RValue))
                    } else {
                        Err(CompilerError {
                            kind: CompilerErrorKind::InternalError,
                            message: format!(
                                "cannot check compatibility for types ({}, {}) in ternary operator expression yet",
                                if_type, else_type
                            ),
                            span: Some(*span),
                        })
                    }
                } else {
                    Err(CompilerError {
                        kind: CompilerErrorKind::SemanticError,
                        message: "condition expression in a ternary expression must be boolean compatible".to_string(),
                        span: Some(ternary_expr.condition.span),
                    })
                }
            }

            Expression::Call(call_expr) => {
                // Get the function signature
                let (callee_type, _) = self.validate_expr(&mut call_expr.callee.node, span)?;

                if let Type {
                    datatype:
                        DataType::Function {
                            return_type,
                            parameters,
                        },
                    qualifiers: _,
                } = &callee_type
                {
                    // ensure number of args = number of parameters
                    if call_expr.argument_expr_list.len() != parameters.len() {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::SemanticError,
                            message: format!(
                                "expected {} args to function instead got {}",
                                parameters.len(),
                                call_expr.argument_expr_list.len()
                            ),
                            span: Some(*span),
                        });
                    }

                    for (param, arg) in zip(parameters, call_expr.argument_expr_list.iter_mut()) {
                        // 1. evaluate argument expression type
                        let (arg_type, _) = self.validate_expr(&mut arg.node, &arg.span)?;

                        // 2. check whether arg type is assignable to param type and add an implicit cast if necessary
                        match Type::check_assignment_conversion(param, &arg_type)? {
                            AssignmentConversionResult::Identical => {}
                            result => {
                                Self::implicit_cast(arg, &param.datatype);

                                if let AssignmentConversionResult::CastWithWarning(warning) = result {
                                    self.diagnostics.warning(warning, Some(arg.span));
                                }
                            }
                        }
                    }

                    // 3. return type of the function is the expression type
                    Ok((return_type.as_ref().clone(), RValue))
                } else {
                    Err(CompilerError {
                        kind: CompilerErrorKind::SemanticError,
                        message: format!("Called object type {} is not a function.", callee_type),
                        span: Some(call_expr.callee.span),
                    })
                }
            }

            Expression::Comma(comma_exprs) => {
                if comma_exprs.is_empty() {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::InternalError,
                        message: "Comma Expression vector can't be empty".to_string(),
                        span: Some(*span),
                    });
                }

                // stores evaluated type of last expression
                let mut ty = Type {
                    datatype: DataType::Void,
                    qualifiers: TypeQualifiers::default(),
                };

                // In case of initializer expressions, we should check whether all comma
                // expressions are of the same type or not, but in case of generic expressions like
                // 4 + 5, false;
                // Above is a valid statement
                for comma_expr in comma_exprs.iter_mut() {
                    (ty, _) = self.validate_expr(&mut comma_expr.node, &comma_expr.span)?;
                }

                Ok((ty, RValue))
            }

            _ => {
                println!("SemanticAnalyzer::validate_expr not implemented for {:?}", expression);
                todo!()
            }
        }
    }

    fn implicit_cast(expr_node: &mut Node<Expression>, datatype: &DataType) {
        let temp_expr = std::mem::replace(&mut expr_node.node, Expression::Empty);

        expr_node.node = Expression::ImplicitCast(Box::new(ImplicitCastExpression {
            expression: temp_expr,
            target_type: datatype.clone(),
        }));
    }

    /// Casts to bool only if the expression type is not bool
    fn implicit_cast_to_bool(expr_node: &mut Node<Expression>, expr_type: &Type) {
        if !matches!(
            expr_type.datatype,
            DataType::Integer {
                rank: IntegerRank::Bool,
                signed: _
            }
        ) {
            let temp_expr = std::mem::replace(&mut expr_node.node, Expression::Empty);

            expr_node.node = Expression::ImplicitCast(Box::new(ImplicitCastExpression {
                expression: temp_expr,
                target_type: DataType::new_integer(IntegerRank::Bool, false),
            }));
        }
    }
}
