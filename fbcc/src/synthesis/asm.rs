use crate::{
    core::errors::{CompilerError, CompilerErrorKind},
    synthesis::ir::{
        BinaryOp, IrFunction,
        IrStatement::{self},
        Operand, UnaryOp,
    },
};
use std::fmt::Write;

pub struct Arm64AsmEmitter {}

impl Arm64AsmEmitter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn emit(&self, irfuncs: &Vec<IrFunction>) -> Result<String, CompilerError> {
        let mut asm = String::new();
        for function in irfuncs {
            asm.push_str(self.emit_func(function)?.as_str());
        }
        Ok(asm)
    }

    fn emit_func(&self, function: &IrFunction) -> Result<String, CompilerError> {
        let mut asm = String::new();

        // mark the function global so the linker (and the C runtime, for `main`) can resolve it
        writeln!(asm, "\t.globl\t_{}", function.name).unwrap();
        writeln!(asm, "_{}:", function.name).unwrap();

        // emit prologue
        // 1. allocate stack frame
        writeln!(asm, "\tsub\tsp, sp, #{}", function.framesize).unwrap();
        // 2. store previous frame record address (x29, x30) on stack
        writeln!(asm, "\tstp\tx29, x30, [sp, #{}]", function.framesize - 16).unwrap();
        // 3. update (x29, x30) to contain current frame record address
        writeln!(asm, "\tadd\tx29, sp, #{}", function.framesize - 16).unwrap();

        // currently we only support 8 parameters
        if function.params.len() > 8 {
            return Err(CompilerError {
                kind: CompilerErrorKind::InternalError,
                message: "function call with more than 8 arguments is not supported".to_string(),
                location: None,
            });
        }

        // store parameters onto stack
        for (index, param) in function.params.iter().enumerate() {
            writeln!(asm, "\tstr\tw{}, [sp, #{}]", index, function.slot_offset(param)).unwrap();
        }

        let mut did_emit_epilogue = false;
        self.emit_funcbody(function, &mut asm, &mut did_emit_epilogue)?;

        if !did_emit_epilogue {
            self.emit_epilogue(function, &mut asm);
        }

        Ok(asm)
    }

    fn emit_epilogue(&self, function: &IrFunction, asm: &mut String) {
        // 1. load previous stack frame's record adress into (x29, x30)
        writeln!(asm, "\tldp\tx29, x30, [sp, #{}]", function.framesize - 16).unwrap();
        // 2. deallocate stack frame memory
        writeln!(asm, "\tadd\tsp, sp, #{}", function.framesize).unwrap();
        // 3. return
        writeln!(asm, "\tret").unwrap();
    }

    fn emit_funcbody(
        &self,
        function: &IrFunction,
        asm: &mut String,
        did_emit_epilogue: &mut bool,
    ) -> Result<(), CompilerError> {
        for statement in &function.body {
            match statement {
                IrStatement::BinaryOp { dst, op, l, r } => {
                    // 1. load left operand
                    self.emit_operand_to_reg(l, "w9", function, asm);

                    // 2. load right operand
                    self.emit_operand_to_reg(r, "w10", function, asm);

                    // 3. perform binary operation
                    match op {
                        BinaryOp::Add => writeln!(asm, "\tadd\tw9, w9, w10").unwrap(),
                        BinaryOp::Sub => writeln!(asm, "\tsub\tw9, w9, w10").unwrap(),
                        BinaryOp::Mul => writeln!(asm, "\tmul\tw9, w9, w10").unwrap(),
                        BinaryOp::Div => writeln!(asm, "\tsdiv\tw9, w9, w10").unwrap(),
                        BinaryOp::Mod => {
                            writeln!(asm, "\tsdiv\tw11, w9, w10").unwrap();
                            writeln!(asm, "\tmsub\tw9, w11, w10, w9").unwrap();
                        }
                        BinaryOp::Lt => {
                            writeln!(asm, "\tsubs\tw9, w9, w10").unwrap();
                            writeln!(asm, "\tcset\tw9, lt").unwrap();
                        }
                        BinaryOp::Le => {
                            writeln!(asm, "\tsubs\tw9, w9, w10").unwrap();
                            writeln!(asm, "\tcset\tw9, le").unwrap();
                        }
                        BinaryOp::Gt => {
                            writeln!(asm, "\tsubs\tw9, w9, w10").unwrap();
                            writeln!(asm, "\tcset\tw9, gt").unwrap();
                        }
                        BinaryOp::Ge => {
                            writeln!(asm, "\tsubs\tw9, w9, w10").unwrap();
                            writeln!(asm, "\tcset\tw9, ge").unwrap();
                        }
                        BinaryOp::Eq => {
                            writeln!(asm, "\tsubs\tw9, w9, w10").unwrap();
                            writeln!(asm, "\tcset\tw9, eq").unwrap();
                        }
                        BinaryOp::NEq => {
                            writeln!(asm, "\tsubs\tw9, w9, w10").unwrap();
                            writeln!(asm, "\tcset\tw9, ne").unwrap();
                        }
                        BinaryOp::And => writeln!(asm, "\tand\tw9, w9, w10").unwrap(),
                        BinaryOp::Or => writeln!(asm, "\torr\tw9, w9, w10").unwrap(),
                        BinaryOp::Xor => writeln!(asm, "\teor\tw9, w9, w10").unwrap(),
                        BinaryOp::LShift => writeln!(asm, "\tlsl\tw9, w9, w10").unwrap(),
                        BinaryOp::RShift => writeln!(asm, "\tasr\tw9, w9, w10").unwrap(),
                    }

                    // 4. store result
                    writeln!(asm, "\tstr\tw9, [sp, #{}]", function.slot_offset(dst)).unwrap();
                }

                IrStatement::UnaryOp { dst, op, src } => {
                    self.emit_operand_to_reg(src, "w9", function, asm);

                    match op {
                        UnaryOp::Minus => writeln!(asm, "\tneg\tw9, w9").unwrap(),
                        UnaryOp::Comp => writeln!(asm, "\tmvn\tw9, w9").unwrap(),
                        UnaryOp::Not => {
                            writeln!(asm, "\tsubs\tw9, w9, #0").unwrap();
                            writeln!(asm, "\tcset\tw9, eq").unwrap();
                        }
                    }

                    writeln!(asm, "\tstr\tw9, [sp, #{}]", function.slot_offset(dst)).unwrap();
                }

                IrStatement::Copy { dst, src } => {
                    // 1. load src operand into w9
                    self.emit_operand_to_reg(src, "w9", function, asm);

                    // 2. store w9 into dst slot
                    writeln!(asm, "\tstr\tw9, [sp, #{}]", function.slot_offset(dst)).unwrap();
                }

                IrStatement::Label(label) => writeln!(asm, ".L{}:", label).unwrap(),
                IrStatement::Jmp(label) => writeln!(asm, "\tb\t.L{}", label).unwrap(),

                IrStatement::JmpIfZero { cond, target } => {
                    self.emit_operand_to_reg(cond, "w9", function, asm);
                    writeln!(asm, "\tcbz\tw9, .L{}", target).unwrap(); // compare and jump to target if zero
                }

                IrStatement::Call { dst, name, args } => {
                    if args.len() > 8 {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::InternalError,
                            message: "function call with more than 8 arguments is not supported".to_string(),
                            location: None,
                        });
                    }

                    // 1. Store the arguments in w0-w7 in order
                    for (index, arg) in args.iter().enumerate() {
                        let reg = format!("w{index}");
                        self.emit_operand_to_reg(arg, reg.as_str(), function, asm);
                    }

                    // 2. Call the procedure
                    writeln!(asm, "\tbl\t_{}", name).unwrap();

                    // 3. Store the return value onto stack
                    if let Some(return_dest) = dst {
                        writeln!(asm, "\tstr\tw0, [sp, #{}]", function.slot_offset(return_dest)).unwrap();
                    }
                }

                IrStatement::Ret(op) => {
                    self.emit_operand_to_reg(op, "w0", function, asm);
                    self.emit_epilogue(function, asm);
                    *did_emit_epilogue = true;
                }
            }
        }
        Ok(())
    }

    fn emit_operand_to_reg(&self, operand: &Operand, reg: &str, function: &IrFunction, asm: &mut String) {
        match operand {
            Operand::Var(slot) => writeln!(asm, "\tldr\t{}, [sp, #{}]", reg, function.slot_offset(slot)).unwrap(),
            Operand::Const(constant) => self.emit_imm_const_to_reg(*constant, reg, asm),
        }
    }

    fn emit_imm_const_to_reg(&self, constant: i64, reg: &str, asm: &mut String) {
        let bits = constant as u32;
        let (low, high) = (bits & 0xffff, bits >> 16);

        writeln!(asm, "\tmov\t{}, #{}", reg, low).unwrap();
        if high != 0 {
            writeln!(asm, "\tmovk\t{}, #{}, lsl #16", reg, high).unwrap();
        }
    }
}
