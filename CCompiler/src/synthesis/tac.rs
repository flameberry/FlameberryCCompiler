use core::fmt;

use crate::{analysis::ast::*, errors::CompilerError, typedefs::*};

#[derive(Debug)]
pub enum TAC {
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

impl fmt::Display for TAC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TAC::Assign {
                target,
                left,
                right,
                operator,
            } => write!(f, "{} = {} {:?} {}", target, left, operator, right),
            TAC::IfGoto { condition, label } => {
                write!(f, "If ({} <= 0) goto({})", condition, label)
            }
            TAC::Goto { label } => write!(f, "goto ({})", label),
            TAC::Return { result } => write!(f, "return {}", result),
            _ => todo!(),
        }
    }
}

pub fn generate_tac(translation_unit: &TranslationUnit) -> Result<Vec<TAC>, CompilerError> {
    let mut tac = Vec::new();

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
                                        let right = generate_tac_expr(&expr, &mut tac)?;
                                        tac.push(TAC::Assign {
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
                generate_tac_stmt(&funcdef.body.node, &mut tac)?;
            }
        }
    }
    Ok(tac)
}

fn generate_tac_stmt(statement: &Statement, out_tacvec: &mut Vec<TAC>) -> Result<Option<String>, CompilerError> {
    match statement {
        Statement::ExpressionStatement(expr) => {
            if let Some(expression) = expr {
                let target = generate_tac_expr(&expression.node, out_tacvec)?;
                Ok(Some(target))
            } else {
                Ok(None)
            }
        }
        Statement::ReturnStatement(return_stmt) => {
            let target = generate_tac_expr(&return_stmt.node, out_tacvec)?;
            out_tacvec.push(TAC::Return { result: target });
            Ok(None)
        }
        Statement::IfStatement(if_stmt) => {
            let target = generate_tac_expr(&if_stmt.expression.node, out_tacvec)?;

            let index_of_if_goto = out_tacvec.len();

            generate_tac_stmt(&if_stmt.if_block.node, out_tacvec)?;

            out_tacvec.insert(
                index_of_if_goto,
                TAC::IfGoto {
                    condition: target,
                    label: (out_tacvec.len() + 1) as u32,
                },
            );
            let end_of_if_block = out_tacvec.len();

            if let Some(else_stmt) = &if_stmt.else_block {
                generate_tac_stmt(&else_stmt.node, out_tacvec)?;
            }

            out_tacvec.insert(
                end_of_if_block,
                TAC::Goto {
                    label: (out_tacvec.len() + 1) as u32,
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
                                                let right = generate_tac_expr(&expr, out_tacvec)?;
                                                out_tacvec.push(TAC::Assign {
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
                        generate_tac_stmt(statement, out_tacvec)?;
                    }
                }
            }
            return Ok(None);
        }
        _ => todo!(),
    }
}

fn generate_tac_expr(expression: &Expression, out_tacvec: &mut Vec<TAC>) -> Result<String, CompilerError> {
    match expression {
        Expression::Identifier(identifier) => Ok(identifier.to_string()),
        Expression::Constant(constant) => Ok(format!("{}", constant)),
        Expression::BinaryOperator(binary_expr) => match &binary_expr.operator.node {
            BinaryOperator::Assign => {
                let _ = generate_tac_expr(&binary_expr.lhs.node, out_tacvec)?;
                let right = generate_tac_expr(&binary_expr.rhs.node, out_tacvec)?;
                let temp = format!("t{}", out_tacvec.len());
                out_tacvec.push(TAC::Assign {
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
                let temp = format!("t{}", out_tacvec.len());
                let left = generate_tac_expr(&binary_expr.lhs.node, out_tacvec)?;
                let right = generate_tac_expr(&binary_expr.rhs.node, out_tacvec)?;
                let operator = binary_expr.operator.node;
                out_tacvec.push(TAC::Assign {
                    target: temp.clone(),
                    left,
                    right,
                    operator,
                });
                return Ok(temp);
            }
            // BinaryOperator::Index => {
            //     let temp = format!("t{}", out_tacvec.len());
            //     let left = generate_tac_expr(&binary_expr.lhs.node, out_tacvec)?;
            //     let right = generate_tac_expr(&binary_expr.rhs.node, out_tacvec)?;
            //     // t1 = left + right * sizeof(right)

            //     out_tacvec.push(TAC::Assign {
            //         target: temp.clone(),
            //         left,
            //         right,
            //         operator,
            //     });
            //     return Ok(temp);
            // }
            _ => {
                let left = generate_tac_expr(&binary_expr.lhs.node, out_tacvec)?;
                let right = generate_tac_expr(&binary_expr.rhs.node, out_tacvec)?;
                let operator = binary_expr.operator.node;
                let temp = format!("t{}", out_tacvec.len());
                out_tacvec.push(TAC::Assign {
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
