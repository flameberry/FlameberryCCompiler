use flameberrycc::compiler::{Compiler, CompilerSpecification};
use std::time::Instant;

fn compile_file(srcpath: &str) {
    let specification = CompilerSpecification {
        target_file: srcpath,
    };
    let mut compiler = Compiler::new(specification);
    compiler.compile();
}

fn main() {
    let start = Instant::now();

    let testpath = "Sandbox/test_semantic.c";
    compile_file(testpath);

    let end = Instant::now() - start;
    println!("Compilation took {:?}", end);
}
