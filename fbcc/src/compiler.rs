use crate::analysis::parser::Parser;
use crate::analysis::semantic_analyzer::SemanticAnalyzer;
use crate::common::symboltable::SymbolTable;
use crate::synthesis::assembly::AssemblyGenerator;
use crate::synthesis::tac::*;
use crate::{analysis::ast::display_translationunit, common::errors::CompilerError};
use std::path::PathBuf;
use std::{
    fs::{self, File},
    io::Write,
};

/// Contains the specification for the compiler.
/// The specification includes the target file to be compiled.
pub struct CompilerOptions {
    pub file: PathBuf,
    pub dump_ast: bool,
}

pub struct Compiler {
    options: CompilerOptions,
    symboltable: SymbolTable,
}

impl Compiler {
    /// Creates a new instance of the `Compiler` struct with the given `CompilerSpecification`.
    pub fn new(specification: CompilerOptions) -> Self {
        Compiler {
            options: specification,
            symboltable: SymbolTable::new(),
        }
    }

    /// Compiles the source file specified in the `CompilerSpecification`.
    /// Panics if there is an error reading the source file.
    pub fn compile(&mut self) -> Result<(), CompilerError> {
        // Read the source file
        let src = fs::read_to_string(&self.options.file);
        println!("Compiling {}...", self.options.file.as_path().to_str().unwrap());

        // Unwrap the result of reading the source file, panicking if there is an error
        let src_str = src.unwrap_or_else(|err| {
            panic!(
                "Failed to read source file: {}: with error: {}",
                self.options.file.as_path().to_str().unwrap(),
                err
            )
        });

        // Create a new instance of the parser
        let mut translation_unit = Parser::new(&src_str).parse()?;
        SemanticAnalyzer::new(&mut self.symboltable).analyze(&mut translation_unit)?;

        if self.options.dump_ast {
            display_translationunit(&translation_unit);
        }

        println!("\n\n{}", self.symboltable);

        // This `if` statement is for developer debugging convenience, to toggle the assembly generation
        if true {
            // Intermediate Code Generation
            println!("\n-------------------------- Three Address Code --------------------------");
            let tac = generate_tac(&translation_unit).unwrap();
            for (i, instruction) in tac.iter().enumerate() {
                println!("({})\t{}", i, instruction);
            }

            println!();

            // Code Generation
            let assembly = AssemblyGenerator::new().generate_assembly(&translation_unit)?;

            // Generate assembly code from the translation unit
            println!("---------------------------------------");
            println!("{}", assembly);
            println!("---------------------------------------");

            // Write to assembly file
            // Derive the path by replacing the extension of the source file with .s
            let assembly_file_path = self.options.file.with_extension("s");
            let mut assemblyfile = File::create(assembly_file_path).unwrap();
            assemblyfile.write_all(assembly.as_bytes()).unwrap();
        }
        Ok(())
    }
}
