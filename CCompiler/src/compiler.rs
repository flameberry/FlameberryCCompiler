use crate::ast::display_translationunit;
use crate::icg::AssemblyGenerator;
use crate::parser::Parser;
use crate::semantic_analyzer::SemanticAnalyzer;
use std::{
    fs::{self, File},
    io::Write,
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
                        true
                        // match AssemblyGenerator::generate_assembly(&translation_unit) {
                        //     Ok(assembly) => {
                        //         println!("{}", assembly);

                        //         // Write to assembly file
                        //         let mut assemblyfile =
                        //             File::create("Sandbox/fbcc_return_2.s").unwrap();
                        //         assemblyfile.write_all(assembly.as_bytes()).unwrap();
                        //         true
                        //     }
                        //     Err(err) => {
                        //         println!("{}:{}", self.specification.target_file, err);
                        //         false
                        //     }
                        // }
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
