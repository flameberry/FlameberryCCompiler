use crate::analysis::ast::display_translationunit;
use crate::analysis::parser::Parser;
use crate::analysis::semantic_analyzer::SemanticAnalyzer;
use crate::symboltable::SymbolTable;
use crate::synthesis::assembly::AssemblyGenerator;
use crate::synthesis::tac::*;
use std::{
    fs::{self, File},
    io::Write,
    path,
};

/// Contains the specification for the compiler.
/// The specification includes the target file to be compiled.
pub struct CompilerSpecification<'a> {
    // target_files: Vec<String>,
    pub target_file: &'a str,
}

pub struct Compiler<'a> {
    specification: CompilerSpecification<'a>,
    symboltable: SymbolTable,
}

impl<'a> Compiler<'a> {
    /// Creates a new instance of the `Compiler` struct with the given `CompilerSpecification`.
    pub fn new(specification: CompilerSpecification<'a>) -> Self {
        Compiler {
            specification,
            symboltable: SymbolTable::new(),
        }
    }

    /// Compiles the source file specified in the `CompilerSpecification`.
    /// Panics if there is an error reading the source file.
    pub fn compile(&self) -> bool {
        // Read the source file
        let src = fs::read_to_string(self.specification.target_file);
        println!("Compiling {}...", self.specification.target_file);

        // Unwrap the result of reading the source file, panicking if there is an error
        let src_str = src.unwrap_or_else(|err| {
            panic!(
                "Failed to read source file: {}: with error: {}",
                self.specification.target_file, err
            )
        });

        // Create a new instance of the parser
        let mut parser = Parser::new(&src_str);

        // Parse the source file
        match parser.parse() {
            Ok(translation_unit) => {
                println!(
                    "\n-------------------------- Abstract Syntax Tree --------------------------"
                );

                // Display the parsed translation unit
                display_translationunit(&translation_unit);

                println!(
                    "\n-------------------------- Three Address Code --------------------------"
                );
                let tac = generate_tac(&translation_unit).unwrap();
                // println!("{:?}", tac);
                let mut i = 0;
                for instruction in tac.iter() {
                    println!("({})\t{}", i, instruction);
                    i += 1;
                }

                println!();
                debug_assert!(false);

                match SemanticAnalyzer::analyze(&translation_unit) {
                    Ok(()) => {
                        println!("Semantic Analysis was successful");

                        // This `if` statement is for developer debugging convenience, to toggle the assembly generation
                        if true {
                            // Create an instance of AssemblyGenerator
                            let mut assembly_generator = AssemblyGenerator::new();

                            // Generate assembly code from the translation unit
                            match assembly_generator.generate_assembly(&translation_unit) {
                                Ok(assembly) => {
                                    println!("---------------------------------------");
                                    println!("{}", assembly);
                                    println!("---------------------------------------");

                                    // Write to assembly file
                                    // Derive the path by replacing the extension of the source file with .s
                                    let assembly_file_path =
                                        path::Path::new(self.specification.target_file)
                                            .with_extension("s");
                                    let mut assemblyfile =
                                        File::create(assembly_file_path).unwrap();
                                    assemblyfile.write_all(assembly.as_bytes()).unwrap();
                                    true
                                }
                                // If there is an error in the assembly generation, print the error and return false
                                Err(err) => {
                                    println!("{}:{}", self.specification.target_file, err);
                                    false
                                }
                            }
                        } else {
                            true
                        }
                    }
                    // If there is an error in the semantic analysis, print the error and return false
                    Err(err) => {
                        println!("{}:{}", self.specification.target_file, err);
                        false
                    }
                }
            }
            // If there is an error in the parsing, print the error and return false
            Err(err) => {
                println!("{}:{}", self.specification.target_file, err);
                false
            }
        }
    }
}
