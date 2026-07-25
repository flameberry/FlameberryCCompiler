use crate::analysis::ast::display_translationunit;
use crate::analysis::parser::Parser;
use crate::analysis::semantic_analyzer::SemanticAnalyzer;
use crate::core::errors::{CompilerError, Diagnostic};
use crate::core::symboltable::SymbolTable;
use crate::synthesis::asm::Arm64AsmEmitter;
use crate::synthesis::ir::IrEmitter;

#[derive(Debug)]
pub struct Compiler {}

impl Compiler {
    pub fn compile(
        input: &str,
        dump_ast: bool,
        dump_ir: bool,
        dump_asm: bool,
    ) -> (Vec<Diagnostic>, Result<String, CompilerError>) {
        let mut symboltable = SymbolTable::new();
        let mut diagnostics: Vec<Diagnostic> = Vec::new();

        let result = (|| -> Result<String, CompilerError> {
            let mut translation_unit = Parser::new(input).parse()?;
            SemanticAnalyzer::new(&mut symboltable, &mut diagnostics).analyze(&mut translation_unit)?;

            if dump_ast {
                display_translationunit(&translation_unit);
                println!("\n\n{}", symboltable);
            }

            let ir = IrEmitter::new().emit(&translation_unit)?;
            if dump_ir {
                println!("\n------- Intermediate Representation (IR) -------\n");
                for function in &ir {
                    println!("{function}");
                }
            }

            let asm = Arm64AsmEmitter::new().emit(&ir)?;
            if dump_asm {
                println!("------- Assembly -------\n\n{}", asm);
            }

            Ok(asm)
        })();

        (diagnostics, result)
    }
}
