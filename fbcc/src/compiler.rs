use crate::analysis::ast::display_translationunit;
use crate::analysis::parser::Parser;
use crate::analysis::semantic_analyzer::SemanticAnalyzer;
use crate::core::errors::CompilerError;
use crate::core::symboltable::SymbolTable;
use crate::synthesis::asm::AsmEmitter;
use crate::synthesis::ir::IrEmitter;

#[derive(Default)]
pub struct Compiler {
    symboltable: SymbolTable,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            symboltable: SymbolTable::new(),
        }
    }

    pub fn compile(
        &mut self,
        input: &str,
        dump_ast: bool,
        dump_ir: bool,
        dump_asm: bool,
    ) -> Result<String, CompilerError> {
        let mut translation_unit = Parser::new(input).parse()?;
        SemanticAnalyzer::new(&mut self.symboltable).analyze(&mut translation_unit)?;

        if dump_ast {
            display_translationunit(&translation_unit);
            println!("\n\n{}", self.symboltable);
        }

        let ir = IrEmitter::new().emit(&translation_unit)?;
        if dump_ir {
            println!("\n------- Intermediate Representation (IR) -------\n");
            for function in &ir {
                println!("{function}");
            }
        }

        let asm = AsmEmitter::new().emit(&ir)?;
        if dump_asm {
            println!("------- Assembly -------\n\n{}", asm);
        }

        Ok(asm)
    }
}
