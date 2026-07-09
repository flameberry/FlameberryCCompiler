use crate::analysis::ast::display_translationunit;
use crate::analysis::parser::Parser;
use crate::analysis::semantic_analyzer::SemanticAnalyzer;
use crate::core::errors::CompilerError;
use crate::core::symboltable::SymbolTable;
use crate::synthesis::ir::generate_ir;

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

    pub fn compile(&mut self, input: &str, dump_ast: bool, dump_ir: bool) -> Result<(), CompilerError> {
        let mut translation_unit = Parser::new(input).parse()?;
        SemanticAnalyzer::new(&mut self.symboltable).analyze(&mut translation_unit)?;

        if dump_ast {
            display_translationunit(&translation_unit);
            println!("\n\n{}", self.symboltable);
        }

        if dump_ir {
            let ir = generate_ir(&translation_unit)?;
            println!("\n\n{:?}", ir);
        }

        Ok(())
    }
}
