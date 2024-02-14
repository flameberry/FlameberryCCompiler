use crate::parser::{display_translationunit, Parser};
use std::fs;

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
                true
            }
            Err(err) => {
                println!("{}:{}", self.specification.target_file, err);
                false
            }
        }
    }
}
