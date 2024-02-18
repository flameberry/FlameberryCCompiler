use crate::{ast::*, errors::CompilerError, tokenizer::IntegerType};

pub struct AssemblyGenerator;

impl AssemblyGenerator {
    pub fn generate_assembly(translation_unit: &TranslationUnit) -> Result<String, CompilerError> {
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
                                    BlockItem::Statement(Statement::ReturnStatement(
                                        return_stmt,
                                    )) => match &return_stmt.node {
                                        Expression::Constant(Constant::Integer(
                                            IntegerType::Generic(return_value),
                                        )) => {
                                            assembly += "\tmov w0, #";
                                            assembly += return_value.to_string().as_str();
                                            assembly += "\n\tret";
                                        }
                                        _ => todo!(),
                                    },
                                    _ => todo!(),
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
}
