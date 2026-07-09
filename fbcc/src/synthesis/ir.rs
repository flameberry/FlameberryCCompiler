use core::fmt;

use crate::{analysis::ast::*, core::errors::CompilerError};

#[derive(Debug)]
pub enum IrOperation {
    Assign {
        target: String,
        left: String,
        right: String,
        operator: BinaryOperator,
    },
    IfGoto {
        condition: String, // Note that this is a condition that if turned out to be false will goto label
        label: u32,
    },
    Goto {
        label: u32,
    },
    Label {
        label: String,
    },
    Param {
        param: String,
    },
    Call {
        function: String,
        result: String,
    },
    Return {
        result: String,
    },
    Function {
        name: String,
    },
    EndFunction,
}

impl fmt::Display for IrOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrOperation::Assign {
                target,
                left,
                right,
                operator,
            } => write!(f, "{} = {} {:?} {}", target, left, operator, right),
            IrOperation::IfGoto { condition, label } => {
                write!(f, "If ({} <= 0) goto({})", condition, label)
            }
            IrOperation::Goto { label } => write!(f, "goto ({})", label),
            IrOperation::Return { result } => write!(f, "return {}", result),
            _ => todo!(),
        }
    }
}

pub fn generate_ir(translation_unit: &TranslationUnit) -> Result<Vec<IrOperation>, CompilerError> {
    let mut ir = Vec::new();

    for extdecl in translation_unit.external_declarations.iter() {
        match &extdecl.node {
            ExternalDeclaration::Declaration(declaration) => {
                for init_decl in &declaration.init_declarators {
                    match &init_decl.node.declarator.node {
                        Declarator::DirectDeclarator(identifier) => {
                            if let Some(initializer) = &init_decl.node.initializer {
                                match &initializer.node {
                                    Initializer::AssignmentExpression(expr) => {
                                        let target = identifier;
                                        let right = generate_ir_expr(&expr, &mut ir)?;
                                        ir.push(IrOperation::Assign {
                                            target: target.to_string(),
                                            left: "".to_string(),
                                            right,
                                            operator: BinaryOperator::Assign,
                                        });
                                    }
                                }
                            }
                        }
                        Declarator::FunctionDeclarator(_) => {
                            todo!()
                        }
                    }
                }
            }
            ExternalDeclaration::FunctionDefinition(funcdef) => {
                generate_ir_stmt(&funcdef.body.node, &mut ir)?;
            }
        }
    }
    Ok(ir)
}

fn generate_ir_stmt(statement: &Statement, out_irvec: &mut Vec<IrOperation>) -> Result<Option<String>, CompilerError> {
    match statement {
        Statement::ExpressionStatement(expr) => {
            if let Some(expression) = expr {
                let target = generate_ir_expr(&expression.node, out_irvec)?;
                Ok(Some(target))
            } else {
                Ok(None)
            }
        }
        Statement::ReturnStatement(return_stmt) => {
            let target = generate_ir_expr(&return_stmt.node, out_irvec)?;
            out_irvec.push(IrOperation::Return { result: target });
            Ok(None)
        }
        Statement::IfStatement(if_stmt) => {
            let target = generate_ir_expr(&if_stmt.expression.node, out_irvec)?;

            let index_of_if_goto = out_irvec.len();

            generate_ir_stmt(&if_stmt.if_block.node, out_irvec)?;

            out_irvec.insert(
                index_of_if_goto,
                IrOperation::IfGoto {
                    condition: target,
                    label: (out_irvec.len() + 1) as u32,
                },
            );
            let end_of_if_block = out_irvec.len();

            if let Some(else_stmt) = &if_stmt.else_block {
                generate_ir_stmt(&else_stmt.node, out_irvec)?;
            }

            out_irvec.insert(
                end_of_if_block,
                IrOperation::Goto {
                    label: (out_irvec.len() + 1) as u32,
                },
            );
            Ok(None)
        }
        Statement::CompoundStatement(blockitems) => {
            for blockitem in blockitems.iter() {
                match &blockitem.node {
                    BlockItem::Declaration(declaration) => {
                        for init_decl in &declaration.init_declarators {
                            match &init_decl.node.declarator.node {
                                Declarator::DirectDeclarator(identifier) => {
                                    if let Some(initializer) = &init_decl.node.initializer {
                                        match &initializer.node {
                                            Initializer::AssignmentExpression(expr) => {
                                                let target = identifier;
                                                let right = generate_ir_expr(&expr, out_irvec)?;
                                                out_irvec.push(IrOperation::Assign {
                                                    target: target.to_string(),
                                                    left: "".to_string(),
                                                    right,
                                                    operator: BinaryOperator::Assign,
                                                });
                                            }
                                        }
                                    }
                                }
                                Declarator::FunctionDeclarator(_) => {
                                    todo!()
                                }
                            }
                        }
                    }
                    BlockItem::Statement(statement) => {
                        generate_ir_stmt(statement, out_irvec)?;
                    }
                }
            }
            return Ok(None);
        }
        _ => todo!(),
    }
}

fn generate_ir_expr(expression: &Expression, out_irvec: &mut Vec<IrOperation>) -> Result<String, CompilerError> {
    match expression {
        Expression::Identifier(identifier) => Ok(identifier.to_string()),
        Expression::Constant(constant) => Ok(format!("{}", constant)),
        Expression::BinaryOperator(binary_expr) => match &binary_expr.operator.node {
            BinaryOperator::Assign => {
                let _ = generate_ir_expr(&binary_expr.lhs.node, out_irvec)?;
                let right = generate_ir_expr(&binary_expr.rhs.node, out_irvec)?;
                let temp = format!("t{}", out_irvec.len());
                out_irvec.push(IrOperation::Assign {
                    target: temp.clone(),
                    left: "".to_string(),
                    right,
                    operator: binary_expr.operator.node,
                });
                return Ok(temp);
            }
            BinaryOperator::AssignPlus
            | BinaryOperator::AssignMinus
            | BinaryOperator::AssignMultiply
            | BinaryOperator::AssignDivide
            | BinaryOperator::AssignModulo
            | BinaryOperator::AssignBitwiseAnd
            | BinaryOperator::AssignBitwiseOr
            | BinaryOperator::AssignBitwiseXor
            | BinaryOperator::AssignShiftLeft
            | BinaryOperator::AssignShiftRight => {
                let temp = format!("t{}", out_irvec.len());
                let left = generate_ir_expr(&binary_expr.lhs.node, out_irvec)?;
                let right = generate_ir_expr(&binary_expr.rhs.node, out_irvec)?;
                let operator = binary_expr.operator.node;
                out_irvec.push(IrOperation::Assign {
                    target: temp.clone(),
                    left,
                    right,
                    operator,
                });
                return Ok(temp);
            }
            // BinaryOperator::Index => {
            //     let temp = format!("t{}", out_irvec.len());
            //     let left = generate_ir_expr(&binary_expr.lhs.node, out_irvec)?;
            //     let right = generate_ir_expr(&binary_expr.rhs.node, out_irvec)?;
            //     // t1 = left + right * sizeof(right)

            //     out_irvec.push(ir::Assign {
            //         target: temp.clone(),
            //         left,
            //         right,
            //         operator,
            //     });
            //     return Ok(temp);
            // }
            _ => {
                let left = generate_ir_expr(&binary_expr.lhs.node, out_irvec)?;
                let right = generate_ir_expr(&binary_expr.rhs.node, out_irvec)?;
                let operator = binary_expr.operator.node;
                let temp = format!("t{}", out_irvec.len());
                out_irvec.push(IrOperation::Assign {
                    target: temp.clone(),
                    left,
                    right,
                    operator,
                });
                return Ok(temp);
            }
        },
        _ => todo!("{:?}", expression),
    }
}
