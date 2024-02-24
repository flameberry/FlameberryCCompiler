use crate::ast::display_translationunit;
use crate::parser::Parser;
use crate::semantic_analyzer::SemanticAnalyzer;
use crate::Assembly::AssemblyGenerator;
use std::{
    fs::{self, File},
    io::Write,
    path,
};

pub struct CompilerSpecification<'a> {
    // target_files: Vec<String>,
    pub target_file: &'a str,
}

pub struct Compiler<'a> {
    specification: CompilerSpecification<'a>,
}

impl<'a> Compiler<'a> {
    pub fn new(specification: CompilerSpecification<'a>) -> Self {
        Compiler { specification }
    }

    pub fn compile(&self) -> bool {
        let src = fs::read_to_string(self.specification.target_file);
        println!("Compiling {}...", self.specification.target_file);

        let src_str = src.unwrap_or_else(|err| {
            panic!(
                "Failed to read source file: {}: with error: {}",
                self.specification.target_file, err
            )
        });

        let mut parser = Parser::new(&src_str);

        match parser.parse() {
            Ok(translation_unit) => {
                // println!("{:?}", translation_unit),
                display_translationunit(&translation_unit);

                match SemanticAnalyzer::analyze(&translation_unit) {
                    Ok(()) => {
                        println!("Semantic Analysis was successful");

                        // This `if` statement is for developer debugging convenience
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
                                Err(err) => {
                                    println!("{}:{}", self.specification.target_file, err);
                                    false
                                }
                            }
                        } else {
                            true
                        }
                    }
                    Err(err) => {
                        println!("{}:{}", self.specification.target_file, err);
                        false
                    }
                }
            }
            Err(err) => {
                println!("{}:{}", self.specification.target_file, err);
                false
            }
        }
    }
}
