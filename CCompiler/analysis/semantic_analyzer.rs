use std::iter::zip;

use crate::analysis::{ast::*, node::Span};
use crate::errors::{CompilerError, CompilerErrorKind};
use crate::symboltable::{SymbolDefinition, SymbolTable};
use crate::typedefs::{BaseType, Type, TypeCompatibility, TypeQualifiers};

use super::node::Node;

// TODOS: Store the scope ID somewhere in the AST probably
// ...to ensure that every time we need to find a symbol from the AST we can lookup using the scope ID

// Simple way to keep track of the context of the evaluation.
// For checking whether break, continue statements are inside any valid loops
enum EvaluationContext {
    None,
    Loop(u32), // Stores the depth of the loop in case of nested loops
}

pub struct SemanticAnalyzer<'a> {
    symboltableref: &'a mut SymbolTable,
    scopeidstack: Vec<u32>,
    counter: u32,
    evaluation_context: EvaluationContext,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new(symboltableref: &'a mut SymbolTable) -> Self {
        Self {
            symboltableref,
            scopeidstack: vec![0], // 0 represents global scope
            counter: 1,
            evaluation_context: EvaluationContext::None,
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

    fn push_loop(&mut self) {
        match self.evaluation_context {
            EvaluationContext::None => self.evaluation_context = EvaluationContext::Loop(1),
            EvaluationContext::Loop(depth) => self.evaluation_context = EvaluationContext::Loop(depth + 1),
        }
    }

    fn pop_loop(&mut self) {
        match self.evaluation_context {
            EvaluationContext::None => panic!("pop_loop called when no loop has been pushed."),
            EvaluationContext::Loop(1) => self.evaluation_context = EvaluationContext::None,
            EvaluationContext::Loop(depth) => self.evaluation_context = EvaluationContext::Loop(depth - 1),
        }
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
                location: Some(function_def.body.span.start),
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
        let function_type = Type::new(BaseType::Function {
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
                let return_type = self.validate_expr(&mut return_stmt.node, &return_stmt.span)?;

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
            Statement::ExpressionStatement(expr_node) => {
                if let Some(expression) = expr_node {
                    self.validate_expr(&mut expression.node, &expression.span)?;
                }
            }

            Statement::ForStatement(for_stmt) => {
                // This is done for verification of break and continue statements
                self.push_loop();

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
                    let condition_type = self.validate_expr(&mut condition.node, &condition.span)?;

                    match Type::compare(&condition_type, &Type::new(BaseType::Bool)) {
                        TypeCompatibility::Identical | TypeCompatibility::Compatible => {}

                        TypeCompatibility::ImplicitConversion { .. } => {
                            // Add an implicit cast with target boolean type
                            let temp_expr = for_stmt.condition.take().unwrap();

                            for_stmt.condition = Some(Node::new(
                                Expression::ImplicitCast(Box::new(ImplicitCastExpression {
                                    expression: temp_expr.node,
                                    target_type: BaseType::Bool,
                                })),
                                temp_expr.span,
                            ));
                        }
                        TypeCompatibility::Incompatible => {
                            return Err(CompilerError {
                                kind: CompilerErrorKind::SemanticError,
                                message: format!("Expected boolean expression, instead got {}", condition_type),
                                location: Some(condition.span.start),
                            })
                        }
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
                self.pop_loop();
            }

            Statement::WhileStatement(while_stmt) | Statement::DoWhileStatement(while_stmt) => {
                // 1. Evaluate condition and check if the type can evaluate into a boolean
                let condition_type =
                    self.validate_expr(&mut while_stmt.expression.node, &while_stmt.expression.span)?;

                match Type::compare(&condition_type, &Type::new(BaseType::Bool)) {
                    TypeCompatibility::Identical | TypeCompatibility::Compatible => {}

                    TypeCompatibility::ImplicitConversion { .. } => {
                        // Add an implicit cast with target boolean type
                        let temp_expr = std::mem::replace(&mut while_stmt.expression.node, Expression::Empty);

                        while_stmt.expression.node = Expression::ImplicitCast(Box::new(ImplicitCastExpression {
                            expression: temp_expr,
                            target_type: BaseType::Bool,
                        }));
                    }
                    TypeCompatibility::Incompatible => {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::SemanticError,
                            message: format!("Expected boolean expression, instead got {}", condition_type),
                            location: Some(while_stmt.expression.span.start),
                        })
                    }
                }

                self.push_loop();
                // 2. Evaluate the while-loop body
                self.validate_statement(&mut while_stmt.statement.node, expected_return_type)?;
                self.pop_loop();
            }

            Statement::IfStatement(if_stmt) => {
                // 1. Evaluate condition and check if the type can evaluate into a boolean
                let condition_type = self.validate_expr(&mut if_stmt.expression.node, &if_stmt.expression.span)?;

                match Type::compare(&condition_type, &Type::new(BaseType::Bool)) {
                    TypeCompatibility::Identical | TypeCompatibility::Compatible => {}

                    TypeCompatibility::ImplicitConversion { .. } => {
                        // Add an implicit cast with target boolean type
                        let temp_expr = std::mem::replace(&mut if_stmt.expression.node, Expression::Empty);

                        if_stmt.expression.node = Expression::ImplicitCast(Box::new(ImplicitCastExpression {
                            expression: temp_expr,
                            target_type: BaseType::Bool,
                        }));
                    }
                    TypeCompatibility::Incompatible => {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::SemanticError,
                            message: format!("Expected boolean expression, instead got {}", condition_type),
                            location: Some(if_stmt.expression.span.start),
                        })
                    }
                }

                // 2. Evaluate the if-statement body
                self.validate_statement(&mut if_stmt.if_block.node, expected_return_type)?;

                // 3. Evaluate the else-statement body if it exists
                if let Some(else_block) = &mut if_stmt.else_block {
                    self.validate_statement(&mut else_block.node, expected_return_type)?;
                }
            }

            Statement::BreakStatement | Statement::ContinueStatement => {
                if !matches!(self.evaluation_context, EvaluationContext::Loop(_)) {
                    let keyword = match statement {
                        Statement::BreakStatement => "break",
                        Statement::ContinueStatement => "continue",
                        _ => unreachable!(), // Only these two arms are possible
                    };

                    return Err(CompilerError {
                        kind: CompilerErrorKind::SemanticError,
                        message: format!("{} statement not allowed outside of a loop", keyword),
                        location: None,
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
                        let rhs_typeinfo = self.validate_expr(asgn_expr, &init_node.span)?;

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

    fn validate_expr(&self, expression: &mut Expression, span: &Span) -> Result<Type, CompilerError> {
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
                let lhs_typeinfo = self.validate_expr(&mut binary_expr.lhs.node, &binary_expr.lhs.span)?;
                let rhs_typeinfo = self.validate_expr(&mut binary_expr.rhs.node, &binary_expr.rhs.span)?;
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
                                "Incompatible Types provided to binary expression: {:?} => {} and {}",
                                binary_expr.operator.node, lhs_typeinfo, rhs_typeinfo
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

            Expression::Call(call_expr) => {
                // Get the function signature
                let callee_type = self.validate_expr(&mut call_expr.callee.node, span)?;

                if let Type {
                    base_type:
                        BaseType::Function {
                            return_type,
                            parameters,
                        },
                    qualifiers: _,
                } = &callee_type
                {
                    for (param, arg) in zip(parameters, call_expr.argument_expr_list.iter_mut()) {
                        // 1. Evaluate argument expression type
                        let arg_type = self.validate_expr(&mut arg.node, &arg.span)?;

                        // 2. Compare the parameter type and argument type, and add an implicit
                        //    cast if necessary
                        match Type::compare(param, &arg_type) {
                            TypeCompatibility::Identical | TypeCompatibility::Compatible => {}

                            TypeCompatibility::ImplicitConversion { .. } => {
                                let temp_expr = std::mem::replace(&mut arg.node, Expression::Empty);

                                arg.node = Expression::ImplicitCast(Box::new(ImplicitCastExpression {
                                    expression: temp_expr,
                                    target_type: param.base_type.clone(),
                                }));
                            }

                            TypeCompatibility::Incompatible => {
                                return Err(CompilerError {
                                    kind: CompilerErrorKind::SemanticError,
                                    message: format!("Expected argument of type: {}, instead got {}", param, arg_type),
                                    location: Some(arg.span.start),
                                })
                            }
                        }
                    }
                    Ok(return_type.as_ref().clone())
                } else {
                    Err(CompilerError {
                        kind: CompilerErrorKind::SemanticError,
                        message: format!("Called object type {} is not a function.", callee_type),
                        location: Some(call_expr.callee.span.start),
                    })
                }
            }

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
                        typeinfo = self.validate_expr(&mut comma_expr.node, &comma_expr.span)?;
                        first = false;
                    }
                }

                // For now this function returns the first comma expression's evaluated type
                Ok(typeinfo)
            }

            Expression::Identifier(idname) => {
                // Tasks to be performed:
                // 1. Check if idname is a valid symbol in the symboltable
                // 2. Convert TypeName to TypeInfo
                match self.lookup_innermost_scope_symbol(idname) {
                    Some(symboldef) => Ok(symboldef.typeinfo.clone()),
                    None => Err(CompilerError {
                        kind: CompilerErrorKind::SemanticError,
                        message: format!(
                            "Unable to find the symbol named '{}' in any of the reachable scopes.",
                            idname
                        ),
                        location: Some(span.start),
                    }),
                }
            }

            Expression::Constant(constant) => Ok(BaseType::from_constant(constant)),

            _ => todo!(),
        }
    }

    fn is_lvalue(&self, expression: &Expression) -> bool {
        // TODO: Update function to recognize complex lvalue expressions
        match expression {
            Expression::Identifier(idname) => self.lookup_innermost_scope_symbol(idname).is_some(),
            Expression::Constant(_) => false,
            _ => todo!(),
        }
    }

    fn is_modifiable_lvalue(&self, expression: &Expression) -> bool {
        // TODO: Update function to recognize complex lvalue expressions
        match expression {
            Expression::Identifier(idname) => self
                .lookup_innermost_scope_symbol(idname)
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
