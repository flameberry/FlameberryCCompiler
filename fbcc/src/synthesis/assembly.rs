//! This module contains the `AssemblyGenerator` struct and its implementation.
//! The `AssemblyGenerator` is responsible for generating assembly code from the AST (Abstract Syntax Tree)
//! of a translation unit in the C programming language.
//! It provides a method `generate_assembly` that takes a `TranslationUnit` and returns the generated assembly code as a `String`.
//! The generated assembly code includes the necessary directives and labels for the main function.
//! It also provides helper methods `generate_statement` and `generate_expression` to generate assembly code for statements and expressions, respectively.
//! The module also defines the `IntegerType` enum, which represents different types of integer constants in the C language.
//! This module is used by the compiler to convert C code into assembly code.

use crate::analysis::ast::*;
use crate::common::errors::CompilerError;
use crate::common::typedefs::*;

/// Contains methods to generate traverse the AST (Abstract Syntax Tree) of a translation unit
/// and calls methods for generation of assembly code from it.
#[allow(non_snake_case)]
pub struct AssemblyGenerator {
    assemblylayer: NativeAssemblyLayer,
}

impl AssemblyGenerator {
    /// Returns a new instance of `AssemblyGenerator`.
    pub fn new() -> Self {
        AssemblyGenerator {
            assemblylayer: NativeAssemblyLayer::new(),
        }
    }

    /// Generates assembly code from the given `TranslationUnit`.
    /// The generated assembly code includes the necessary directives and labels for the main function.
    /// Returns the generated assembly code as a `Result<String, CompilerError>`.
    pub fn generate_assembly(&mut self, translation_unit: &TranslationUnit) -> Result<String, CompilerError> {
        let mut assembly = String::new();

        assembly += "\t.globl _main\n";

        for extdecl in &translation_unit.external_declarations {
            // For now iterate through every function definition and check if it is the main function
            match &extdecl.node {
                ExternalDeclaration::FunctionDefinition(funcdef) => {
                    if funcdef.declarator.node.identifier == "main" {
                        // Add _start
                        assembly += "_main:\n\t.cfi_startproc\n";

                        if let Statement::CompoundStatement(comp_stmt) = &funcdef.body.node {
                            for blockitem in comp_stmt {
                                match &blockitem.node {
                                    BlockItem::Statement(statement) => {
                                        assembly += &self.generate_statement(&statement);
                                    }
                                    BlockItem::Declaration(_) => todo!(),
                                }
                            }
                        } else {
                            panic!("Internal Error: A function body must be a compound statement, and this should have been handled by the Semantic Analyzer");
                        }

                        assembly += "\n\t.cfi_endproc\n";
                    } else {
                        todo!()
                    }
                }
                ExternalDeclaration::Declaration(_) => todo!(),
            }
        }
        Ok(assembly)
    }

    // A function to generate assembly for a statement node
    fn generate_statement(&mut self, statement: &Statement) -> String {
        match &statement {
            Statement::ReturnStatement(return_stmt) => {
                let reg = &self.assemblylayer.allocate_register_32();
                format!("{}\tret", self.generate_expression(&return_stmt.node, reg))
            }
            _ => todo!(),
        }
    }

    // A function to generate assembly for an expression node
    fn generate_expression(&mut self, expression: &Expression, dst_register: &str) -> String {
        match &expression {
            Expression::Constant(constant) => match &constant {
                Constant::Integer(integer_type) => match &integer_type {
                    IntegerType::Generic(value) => format!("\tmov {}, #{}\n", dst_register, value),
                    _ => todo!(),
                },
                _ => todo!(),
            },
            Expression::BinaryOperator(binary_op) => {
                // Allocate two working registers
                let lhs_register = &self.assemblylayer.allocate_register_32();
                let rhs_register = &self.assemblylayer.allocate_register_32();

                let lhs_assembly = self.generate_expression(&binary_op.lhs.node, lhs_register);
                let rhs_assembly = self.generate_expression(&binary_op.rhs.node, rhs_register);
                match &binary_op.operator.node {
                    BinaryOperator::Plus => format!(
                        "{}{}\tadd {}, {}, {}\n",
                        lhs_assembly, rhs_assembly, dst_register, lhs_register, rhs_register
                    ),
                    BinaryOperator::Minus => format!(
                        "{}{}\tsub {}, {}, {}\n",
                        lhs_assembly, rhs_assembly, dst_register, lhs_register, rhs_register
                    ),
                    BinaryOperator::Multiply => format!(
                        "{}{}\tmul {}, {}, {}\n",
                        lhs_assembly, rhs_assembly, dst_register, lhs_register, rhs_register
                    ),
                    BinaryOperator::Divide => format!(
                        "{}{}\tsdiv {}, {}, {}\n",
                        lhs_assembly, rhs_assembly, dst_register, lhs_register, rhs_register
                    ),
                    BinaryOperator::Modulo => {
                        let temp_register = &self.assemblylayer.allocate_register_32();
                        format!(
                            "{}{}\tsdiv {}, {}, {}\n\tmsub {}, {}, {}, {}\n",
                            lhs_assembly,
                            rhs_assembly,
                            temp_register,
                            lhs_register,
                            rhs_register,
                            dst_register,
                            temp_register,
                            rhs_register,
                            lhs_register,
                        )
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }
}

#[cfg(target_arch = "aarch64")]
struct NativeAssemblyLayer {
    registers: [bool; 31],
    reserved_registers: Vec<usize>,
}

/// Implementation of AssemblyCommand struct for Aarch64 architecture.
#[cfg(target_arch = "aarch64")]
impl NativeAssemblyLayer {
    /// Returns a new instance of `NativeAssemblyLayer`.
    fn new() -> Self {
        NativeAssemblyLayer {
            registers: [false; 31],

            #[cfg(target_os = "macos")]
            reserved_registers: vec![19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29],
        }
    }

    /// Allocates a 32-bit register.
    ///
    /// This method searches for an available 32-bit register and marks it as allocated.
    /// If no register is available, it panics with a "No more registers available" message.
    ///
    /// # Returns
    ///
    /// The name of the allocated register as a String, in the format "wX" where X is the register number.
    ///
    fn allocate_register_32(&mut self) -> String {
        for i in 0..31 {
            if !self.registers[i] {
                self.registers[i] = true;
                return format!("w{}", i);
            }
        }
        panic!("No more registers available");
    }

    /// Allocates a 64-bit register.
    ///
    /// This method searches for an available 64-bit register and marks it as allocated.
    /// If no register is available, it panics with a "No more registers available" message.
    ///
    /// # Returns
    ///
    /// The name of the allocated register as a String, in the format "xX" where X is the register number.
    ///
    fn allocate_register_64(&mut self) -> String {
        for i in 0..31 {
            if !self.registers[i] {
                self.registers[i] = true;
                return format!("x{}", i);
            }
        }
        panic!("No more registers available");
    }

    /// Frees a register.
    ///
    /// This method marks the specified register as deallocated.
    ///
    /// # Arguments
    ///
    /// * `register` - The name of the register to free, in the format "wX" or "xX" where X is the register number.
    ///
    fn free_register(&mut self, register: &str) {
        let register_number: usize = register[1..].parse().unwrap();
        self.registers[register_number] = false;
    }

    /// Frees all registers.
    ///
    /// This method marks all registers as deallocated.
    ///
    fn free_all_registers(&mut self) {
        for i in 0..31 {
            self.registers[i] = false;
        }
    }
}
