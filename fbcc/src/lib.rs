pub mod analysis;
pub mod common;
pub mod compiler;
pub mod preprocessor;
pub mod synthesis;

use compiler::{Compiler, CompilerOptions};
use std::path::PathBuf;

pub fn compile(srcpath: &PathBuf, dump_ast: bool) {
    let opts = CompilerOptions {
        file: srcpath.clone(),
        dump_ast,
    };

    let result = Compiler::new(opts).compile();

    if result.is_err() {
        println!("{}:{}", srcpath.as_path().to_str().unwrap(), result.err().unwrap());
    }
}
