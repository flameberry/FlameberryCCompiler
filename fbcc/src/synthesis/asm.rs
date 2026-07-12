use crate::{
    core::errors::CompilerError,
    synthesis::ir::{IrFunction, IrStatement, Operand},
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
