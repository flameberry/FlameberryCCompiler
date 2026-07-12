use crate::{
    core::errors::CompilerError,
    synthesis::ir::{BinaryOp, IrFunction, IrStatement, Operand},
};
use std::fmt::Write;

pub struct AsmEmitter {}

impl AsmEmitter {
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

        // emit body
        for statement in &function.body {
            match statement {
                IrStatement::BinaryOp { dst, op, l, r } => {
                    // 1. load left operand
                    match l {
                        Operand::Var(slot) => {
                            // 1. load src operand into w9
                            writeln!(asm, "\tldr\tw9, [sp, #{}]", function.slot_offset(slot)).unwrap();
                        }
                        Operand::Const(constant) => {
                            // 1. move constant into w9
                            writeln!(asm, "\tmov\tw9, #{}", constant).unwrap();
                        }
                    }

                    // 2. load right operand
                    match r {
                        Operand::Var(slot) => {
                            // 1. load src operand into w9
                            writeln!(asm, "\tldr\tw10, [sp, #{}]", function.slot_offset(slot)).unwrap();
                        }
                        Operand::Const(constant) => {
                            // 1. move constant into w9
                            writeln!(asm, "\tmov\tw10, #{}", constant).unwrap();
                        }
                    }

                    // 3. perform binary operation
                    match op {
                        BinaryOp::Add => writeln!(asm, "\tadd\tw9, w9, w10").unwrap(),
                        BinaryOp::Sub => writeln!(asm, "\tsub\tw9, w9, w10").unwrap(),
                        BinaryOp::Mul => writeln!(asm, "\tmul\tw9, w9, w10").unwrap(),
                        BinaryOp::Div => writeln!(asm, "\tsdiv\tw9, w9, w10").unwrap(),
                        _ => todo!(),
                    }

                    // 4. store result
                    writeln!(asm, "\tstr\tw9, [sp, #{}]", function.slot_offset(dst)).unwrap();
                }
                IrStatement::Copy { dst, src } => match src {
                    Operand::Var(slot) => {
                        // 1. load src operand into w9
                        writeln!(asm, "\tldr\tw9, [sp, #{}]", function.slot_offset(slot)).unwrap();
                        // 2. store w9 into dst slot
                        writeln!(asm, "\tstr\tw9, [sp, #{}]", function.slot_offset(dst)).unwrap();
                    }
                    Operand::Const(constant) => {
                        // 1. move constant into w9
                        writeln!(asm, "\tmov\tw9, #{}", constant).unwrap();
                        // 2. store w9 into dst slot
                        writeln!(asm, "\tstr\tw9, [sp, #{}]", function.slot_offset(dst)).unwrap();
                    }
                },
                IrStatement::Ret(op) => match op {
                    Operand::Var(slot) => writeln!(asm, "\tldr\tw0, [sp, #{}]", function.slot_offset(slot)).unwrap(),
                    Operand::Const(constant) => writeln!(asm, "\tmov\tw0, #{}", constant).unwrap(),
                },
                _ => todo!(),
            }
        }

        // emit epilogue
        // 1. load previous stack frame's record adress into (x29, x30)
        writeln!(asm, "\tldp\tx29, x30, [sp, #{}]", function.framesize - 16).unwrap();
        // 2. deallocate stack frame memory
        writeln!(asm, "\tadd\tsp, sp, #{}", function.framesize).unwrap();
        // 3. return
        writeln!(asm, "\tret").unwrap();

        Ok(asm)
    }
}
