use std::collections::HashMap;
use std::fmt;

use crate::{
    analysis::{
        ast::{
            BinaryOperator,
            BlockItem::{self, Declaration},
            Declarator::{self, FunctionDeclarator},
            Expression::{self},
            ExternalDeclaration, ForInitializer, FunctionDefinition, Initializer, Statement, TranslationUnit,
            UnaryOperator, UnaryOperatorExpression,
        },
        node::Node,
    },
    core::{
        errors::{CompilerError, CompilerErrorKind},
        typedefs::{Constant, DataType, IntegerType, Type},
    },
};

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Le,
    LeEq,
    Gr,
    GrEq,
    Eq,
    NEq,
    And,
    Or,
    Xor,
    LShift,
    RShift,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Comp,
    Not,
}

#[derive(Debug, Clone)]
pub struct SlotID(usize);

#[derive(Debug, Clone)]
pub enum Operand {
    Const(i64),
    Var(SlotID),
}

#[derive(Debug, Clone)]
pub enum IrStatement {
    BinaryOp {
        dst: SlotID,
        op: BinaryOp,
        l: Operand,
        r: Operand,
    },
    UnaryOp {
        dst: SlotID,
        op: UnaryOp,
        src: Operand,
    },
    Copy {
        dst: SlotID,
        src: Operand,
    },
    Label(u32),
    Jmp(u32),
    JmpIfZero {
        cond: Operand,
        target: u32,
    },
    Call {
        dst: Option<SlotID>,
        name: String,
        args: Vec<Operand>,
    },
    Ret(Operand),
}

impl fmt::Display for SlotID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "r{}", self.0)
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Const(value) => write!(f, "{value}"),
            Operand::Var(slot) => write!(f, "{slot}"),
        }
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
            BinaryOp::Le => "<",
            BinaryOp::LeEq => "<=",
            BinaryOp::Gr => ">",
            BinaryOp::GrEq => ">=",
            BinaryOp::Eq => "==",
            BinaryOp::NEq => "!=",
            BinaryOp::And => "&",
            BinaryOp::Or => "|",
            BinaryOp::Xor => "^",
            BinaryOp::LShift => "<<",
            BinaryOp::RShift => ">>",
        };
        write!(f, "{symbol}")
    }
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            UnaryOp::Comp => "~",
            UnaryOp::Not => "!",
        };
        write!(f, "{symbol}")
    }
}

impl fmt::Display for IrStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IrStatement::BinaryOp { dst, op, l, r } => write!(f, "{dst} = {l} {op} {r}"),
            IrStatement::UnaryOp { dst, op, src } => write!(f, "{dst} = {op}{src}"),
            IrStatement::Copy { dst, src } => write!(f, "{dst} = {src}"),
            IrStatement::Label(id) => write!(f, "L{id}:"),
            IrStatement::Jmp(target) => write!(f, "jmp L{target}"),
            IrStatement::JmpIfZero { cond, target } => write!(f, "jz {cond}, L{target}"),
            IrStatement::Call { dst, name, args } => {
                if let Some(dst) = dst {
                    write!(f, "{dst} = ")?;
                }
                write!(f, "call {name}(")?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{arg}")?;
                }
                write!(f, ")")
            }
            IrStatement::Ret(value) => write!(f, "ret {value}"),
        }
    }
}

impl fmt::Display for IrFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Header: `func name(r0, r1) [frame=N]:`
        write!(f, "func {}(", self.name)?;
        for (i, param) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{param}")?;
        }
        writeln!(f, ") [frame={}]:", self.framesize)?;

        // Body: statements indented, labels kept at the margin so they stand out.
        for statement in &self.body {
            match statement {
                IrStatement::Label(_) => writeln!(f, "{statement}")?,
                _ => writeln!(f, "    {statement}")?,
            }
        }
        Ok(())
    }
}

pub struct Slot {
    ty: Type,
    size: usize,
    align: usize,
    offset: usize,
}

pub struct IrFunction {
    pub name: String,
    pub framesize: usize,
    pub params: Vec<SlotID>,
    pub slots: Vec<Slot>,
    pub body: Vec<IrStatement>,
}

impl IrFunction {
    /// Byte offset of a slot within the frame (sp-relative).
    pub fn slot_offset(&self, slot: &SlotID) -> usize {
        self.slots[slot.0].offset
    }
}

struct FrameBuilder {
    slots: Vec<Slot>,
    offset: usize,
}

impl FrameBuilder {
    fn new() -> Self {
        FrameBuilder {
            slots: Vec::new(),
            offset: 0,
        }
    }

    fn allocate(&mut self, ty: Type) -> Result<SlotID, CompilerError> {
        let id = self.slots.len();
        let size = ty.size()?;
        let align = ty.align()?;

        self.offset = self.offset.next_multiple_of(align);

        let slot = Slot {
            ty,
            size,
            align,
            offset: self.offset,
        };

        self.slots.push(slot);
        self.offset += size;
        Ok(SlotID(id))
    }
}

/// Resolve an identifier to its slot, searching scopes innermost-first.
fn lookup(scopes: &[HashMap<String, SlotID>], name: &str) -> Option<SlotID> {
    scopes.iter().rev().find_map(|scope| scope.get(name)).cloned()
}

pub struct IrEmitter {
    labelcounter: u32,
}

impl IrEmitter {
    pub fn new() -> Self {
        IrEmitter { labelcounter: 0 }
    }

    fn newlabel(&mut self) -> (u32, IrStatement) {
        let label = IrStatement::Label(self.labelcounter);
        self.labelcounter += 1;
        return (self.labelcounter - 1, label);
    }

    pub fn emit(&mut self, translation_unit: &TranslationUnit) -> Result<Vec<IrFunction>, CompilerError> {
        let mut functions: Vec<IrFunction> = Vec::new();

        for extdecl in &translation_unit.external_declarations {
            match &extdecl.node {
                ExternalDeclaration::FunctionDefinition(function) => functions.push(self.emit_func(function)?),

                ExternalDeclaration::Declaration(_) => {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::InternalError,
                        message: "global variables are not implemented in IR yet".to_string(),
                        location: None,
                    })
                }
            }
        }

        Ok(functions)
    }

    fn emit_func(&mut self, function: &FunctionDefinition) -> Result<IrFunction, CompilerError> {
        let Statement::CompoundStatement(compound_stmt) = &function.body.node else {
            return Err(CompilerError {
                kind: CompilerErrorKind::SemanticError,
                message: "function body must be a compound statement".to_string(),
                location: Some(function.body.span.start),
            });
        };

        let mut params: Vec<SlotID> = Vec::new();
        let mut framebuilder = FrameBuilder::new();
        let mut scopes: Vec<HashMap<String, SlotID>> = Vec::new();
        scopes.push(HashMap::new());

        // deal with params
        for param in &function.declarator.node.parameters {
            // IMP: redundand type calculation, preferrably store it in ast itself
            let (paramtype, _) = Type::from_declaration_specifiers(&param.node.specifiers)?;

            // Skipping allocating `void` parameters
            if matches!(paramtype.datatype, DataType::Void) {
                continue;
            }

            let slotid = framebuilder.allocate(paramtype)?;

            let paramname: Option<String> = match &param.node.declarator {
                // A plain parameter name (`int a`) is a DirectDeclarator, not a FunctionDeclarator.
                Some(declarator) => match &declarator.node {
                    Declarator::DirectDeclarator(name) => Some(name.clone()),
                    Declarator::FunctionDeclarator(_) => {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::InternalError,
                            message: "function-typed parameters are not yet supported".to_string(),
                            location: Some(param.span.start),
                        });
                    }
                },
                // Unnamed parameter, e.g. a prototype's `int` or `void`.
                None => None,
            };

            // Only named params get a scope binding; an unnamed param still has a slot.
            if let Some(name) = paramname {
                if let Some(currscope) = scopes.last_mut() {
                    currscope.insert(name, slotid.clone());
                }
            }

            params.push(slotid);
        }

        let mut units: Vec<IrStatement> = Vec::new();

        for blockitem in compound_stmt {
            match &blockitem.node {
                BlockItem::Declaration(declaration) => {
                    units.extend(self.emit_declaration(declaration, &mut framebuilder, &mut scopes)?);
                }
                BlockItem::Statement(stmt) => {
                    units.extend(self.emit_stmt(&stmt, &mut scopes, &mut framebuilder)?);
                }
            }
        }

        scopes.pop();

        let irfunction = IrFunction {
            name: function.declarator.node.identifier.clone(),
            framesize: (framebuilder.offset + 16).next_multiple_of(16),
            params,
            slots: framebuilder.slots,
            body: units,
        };

        Ok(irfunction)
    }

    fn emit_declaration(
        &mut self,
        declaration: &crate::analysis::ast::Declaration,
        framebuilder: &mut FrameBuilder,
        scopes: &mut Vec<HashMap<String, SlotID>>,
    ) -> Result<Vec<IrStatement>, CompilerError> {
        let mut units: Vec<IrStatement> = Vec::new();
        for init_declarator in &declaration.init_declarators {
            if let Declarator::DirectDeclarator(identifier) = &init_declarator.node.declarator.node {
                let (decltype, _) = Type::from_declaration_specifiers(&declaration.specifiers)?;
                let slotid = framebuilder.allocate(decltype)?;

                if let Some(currscope) = scopes.last_mut() {
                    currscope.insert(identifier.clone(), slotid.clone());
                }

                // emit the initializer ir
                if let Some(Node {
                    node: Initializer::AssignmentExpression(expression),
                    span: _,
                }) = &init_declarator.node.initializer
                {
                    let (operand, expr_ir) = self.emit_expr(expression, scopes, framebuilder)?;
                    units.extend(expr_ir);

                    units.push(IrStatement::Copy {
                        dst: slotid,
                        src: operand,
                    });
                }
            }
        }
        Ok(units)
    }

    fn emit_stmt(
        &mut self,
        stmt: &Statement,
        scopes: &mut Vec<HashMap<String, SlotID>>,
        framebuilder: &mut FrameBuilder,
    ) -> Result<Vec<IrStatement>, CompilerError> {
        let mut units: Vec<IrStatement> = Vec::new();
        match stmt {
            Statement::CompoundStatement(compound_stmt) => {
                scopes.push(HashMap::new());

                for blockitem in compound_stmt {
                    match &blockitem.node {
                        BlockItem::Declaration(declaration) => {
                            self.emit_declaration(declaration, framebuilder, scopes)?;
                        }
                        BlockItem::Statement(stmt) => {
                            units.extend(self.emit_stmt(&stmt, scopes, framebuilder)?);
                        }
                    }
                }

                scopes.pop();
            }

            Statement::ExpressionStatement(expr_stmt_opt) => {
                if let Some(expr_stmt) = expr_stmt_opt {
                    let (_, expr_ir) = self.emit_expr(&expr_stmt.node, scopes, framebuilder)?;
                    units.extend(expr_ir);
                }
            }

            Statement::IfStatement(ifstmt) => {
                let (condresult, cond_ir) = self.emit_expr(&ifstmt.expression.node, scopes, framebuilder)?;
                units.extend(cond_ir);

                let (lelse_id, lelse) = self.newlabel();

                units.push(IrStatement::JmpIfZero {
                    cond: condresult,
                    target: lelse_id,
                });

                units.extend(self.emit_stmt(&ifstmt.if_block.node, scopes, framebuilder)?);
                if let Some(else_block) = &ifstmt.else_block {
                    let (lendif_id, lendif) = self.newlabel();
                    units.push(IrStatement::Jmp(lendif_id));
                    units.push(lelse);
                    units.extend(self.emit_stmt(&else_block.node, scopes, framebuilder)?);
                    units.push(lendif);
                } else {
                    units.push(lelse);
                }
            }

            Statement::WhileStatement(whilestmt) => {
                let (condresult, cond_ir) = self.emit_expr(&whilestmt.expression.node, scopes, framebuilder)?;

                let (lstart_id, lstart) = self.newlabel();
                let (lend_id, lend) = self.newlabel();

                units.push(lstart);
                units.extend(cond_ir);
                units.push(IrStatement::JmpIfZero {
                    cond: condresult,
                    target: lend_id,
                });

                units.extend(self.emit_stmt(&whilestmt.statement.node, scopes, framebuilder)?);
                units.push(IrStatement::Jmp(lstart_id));
                units.push(lend);
            }

            Statement::ForStatement(forstmt) => {
                match &forstmt.initializer.node {
                    ForInitializer::Empty => {}
                    ForInitializer::Declaration(declaration) => {
                        units.extend(self.emit_declaration(declaration, framebuilder, scopes)?);
                    }
                    ForInitializer::Expression(expression) => {
                        let (_, expr_ir) = self.emit_expr(expression, scopes, framebuilder)?;
                        units.extend(expr_ir);
                    }
                }

                let (startid, start) = self.newlabel();
                let (endid, end) = self.newlabel();

                units.push(start);

                if let Some(condition) = &forstmt.condition {
                    let (condresult, cond_ir) = self.emit_expr(&condition.node, scopes, framebuilder)?;

                    units.extend(cond_ir);
                    units.push(IrStatement::JmpIfZero {
                        cond: condresult,
                        target: endid,
                    });
                }

                units.extend(self.emit_stmt(&forstmt.statement.node, scopes, framebuilder)?);

                if let Some(step) = &forstmt.step {
                    let (_, step_ir) = self.emit_expr(&step.node, scopes, framebuilder)?;
                    units.extend(step_ir);
                }

                units.push(IrStatement::Jmp(startid));
                units.push(end);
            }

            Statement::ReturnStatement(returnstmt) => {
                let (operand, expr_ir) = self.emit_expr(&returnstmt.node, scopes, framebuilder)?;
                units.extend(expr_ir);
                units.push(IrStatement::Ret(operand));
            }
            _ => todo!(),
        }
        Ok(units)
    }

    fn emit_expr(
        &mut self,
        expr: &Expression,
        scopes: &mut Vec<HashMap<String, SlotID>>,
        framebuilder: &mut FrameBuilder,
    ) -> Result<(Operand, Vec<IrStatement>), CompilerError> {
        match expr {
            Expression::UnaryOperator(unaryexpr) => {
                let (unaryop_result, units) = self.emit_expr(&unaryexpr.operand.node, scopes, framebuilder)?;

                match &unaryexpr.operator.node {
                    UnaryOperator::Plus => return Ok((unaryop_result, units)),
                    // `-x` has no dedicated IR op; lower it as `0 - x` into a fresh temp.
                    UnaryOperator::Minus => {
                        let mut units = units;
                        let dst = framebuilder.allocate(Type::new(DataType::Int { signed: true }))?;
                        units.push(IrStatement::BinaryOp {
                            dst: dst.clone(),
                            op: BinaryOp::Sub,
                            l: Operand::Const(0),
                            r: unaryop_result,
                        });
                        return Ok((Operand::Var(dst), units));
                    }
                    // PostIncrement
                    // PostDecrement,
                    // PreIncrement,
                    // PreDecrement,
                    // Address,
                    // Indirection,
                    // Complement,
                    // Negate,
                    _ => todo!(),
                }
            }

            Expression::BinaryOperator(binaryexpr) => {
                let (lhs, lhs_ir) = self.emit_expr(&binaryexpr.lhs.node, scopes, framebuilder)?;
                let (rhs, rhs_ir) = self.emit_expr(&binaryexpr.rhs.node, scopes, framebuilder)?;

                let mut units = [lhs_ir.as_slice(), rhs_ir.as_slice()].concat();

                match &binaryexpr.operator.node {
                    BinaryOperator::Index
                    | BinaryOperator::LogicalAnd
                    | BinaryOperator::LogicalOr
                    | BinaryOperator::AssignMultiply
                    | BinaryOperator::AssignDivide
                    | BinaryOperator::AssignModulo
                    | BinaryOperator::AssignPlus
                    | BinaryOperator::AssignMinus
                    | BinaryOperator::AssignShiftLeft
                    | BinaryOperator::AssignShiftRight
                    | BinaryOperator::AssignBitwiseAnd
                    | BinaryOperator::AssignBitwiseXor
                    | BinaryOperator::AssignBitwiseOr => todo!(),

                    BinaryOperator::Assign => {
                        if let Operand::Var(lhs_slot) = &lhs {
                            units.push(IrStatement::Copy {
                                dst: lhs_slot.clone(),
                                src: rhs,
                            });
                            return Ok((lhs, units));
                        } else {
                            return Err(CompilerError {
                                kind: CompilerErrorKind::SemanticError,
                                message: format!("lhs of assign statement must be a lvalue"),
                                location: Some(binaryexpr.operator.span.start),
                            });
                        }
                    }

                    _ => {
                        let binaryop = match &binaryexpr.operator.node {
                            BinaryOperator::Plus => BinaryOp::Add,
                            BinaryOperator::Minus => BinaryOp::Sub,
                            BinaryOperator::Multiply => BinaryOp::Mul,
                            BinaryOperator::Divide => BinaryOp::Div,
                            BinaryOperator::Modulo => BinaryOp::Mod,
                            BinaryOperator::Less => BinaryOp::Le,
                            BinaryOperator::LessOrEqual => BinaryOp::LeEq,
                            BinaryOperator::Greater => BinaryOp::Gr,
                            BinaryOperator::GreaterOrEqual => BinaryOp::GrEq,
                            BinaryOperator::Equals => BinaryOp::Eq,
                            BinaryOperator::NotEquals => BinaryOp::NEq,
                            BinaryOperator::BitwiseAnd => BinaryOp::And,
                            BinaryOperator::BitwiseOr => BinaryOp::Or,
                            BinaryOperator::BitwiseXor => BinaryOp::Xor,
                            BinaryOperator::ShiftLeft => BinaryOp::LShift,
                            BinaryOperator::ShiftRight => BinaryOp::RShift,
                            _ => unreachable!(),
                        };

                        let resultop = framebuilder.allocate(Type::new(DataType::Int { signed: true }))?;
                        units.push(IrStatement::BinaryOp {
                            dst: resultop.clone(),
                            op: binaryop,
                            l: lhs,
                            r: rhs,
                        });

                        return Ok((Operand::Var(resultop), units));
                    }
                }
            }

            Expression::Identifier(identifier) => {
                let slot = lookup(scopes, identifier).ok_or_else(|| CompilerError {
                    kind: CompilerErrorKind::InternalError,
                    message: format!("undeclared identifier `{identifier}` reached IR lowering"),
                    location: None,
                })?;
                return Ok((Operand::Var(slot), Vec::new()));
            }

            Expression::Constant(constant) => match constant {
                Constant::Integer(integertype) => match integertype {
                    IntegerType::Generic(integer) => return Ok((Operand::Const(integer.clone()), Vec::new())),
                    _ => todo!(),
                },
                _ => todo!(),
            },

            Expression::ImplicitCast(cast) => {
                let (operand, mut units) = self.emit_expr(&cast.expression, scopes, framebuilder)?;
                match cast.target_type {
                    // int-width integer: representation is unchanged, forward as-is.
                    DataType::Int { .. } => return Ok((operand, units)),
                    // `(_Bool)x` == `x != 0`; normalizes any int to 0/1.
                    DataType::Bool => {
                        let dst = framebuilder.allocate(Type::new(DataType::Bool))?;
                        units.push(IrStatement::BinaryOp {
                            dst: dst.clone(),
                            op: BinaryOp::NEq,
                            l: operand,
                            r: Operand::Const(0),
                        });
                        return Ok((Operand::Var(dst), units));
                    }
                    // Width changes / float conversions need the source type too, which
                    // `emit_expr` does not yet carry — out of scope for now.
                    _ => {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::InternalError,
                            message: format!("unsupported implicit cast to `{}`", cast.target_type),
                            location: None,
                        })
                    }
                }
            }

            Expression::Call(callexpr) => {
                let funcname = match &callexpr.callee.node {
                    Expression::Identifier(fname) => fname,
                    _ => {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::InternalError,
                            message: "callee expressions other than an identifier expression is not supported"
                                .to_string(),
                            location: Some(callexpr.callee.span.start),
                        })
                    }
                };

                let mut units: Vec<IrStatement> = Vec::new();
                let mut args: Vec<Operand> = Vec::new();

                for param in &callexpr.argument_expr_list {
                    let (argop, arg_ir) = self.emit_expr(&param.node, scopes, framebuilder)?;
                    units.extend(arg_ir);
                    args.push(argop);
                }

                let dst = framebuilder.allocate(Type::new(DataType::Int { signed: true }))?;
                units.push(IrStatement::Call {
                    dst: Some(dst.clone()),
                    name: funcname.clone(),
                    args,
                });
                return Ok((Operand::Var(dst), units));
            }

            Expression::Empty => {}
            _ => {
                println!("{:?}", expr);
                return Err(CompilerError {
                    kind: CompilerErrorKind::InternalError,
                    message: format!("emit_expr not implemented for {:?} yet", expr),
                    location: None,
                });
            }
        }
        todo!()
    }
}
