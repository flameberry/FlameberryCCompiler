use flameberrycc::compiler::{compile, Compiler, CompilerSpecification};
use std::{fs, time::Instant};

fn compile_file(srcpath: &str) {
    let specification = CompilerSpecification {
        target_file: srcpath,
    };
    let compiler = Compiler::new(specification);
    compiler.compile();
}

fn run_tests(testpath: &str) {
    let directory =
        fs::read_dir(testpath).expect(&format!("Failed to read directory: {}", testpath));

    let mut test_pass_count = 0;
    let mut test_cases = 0;

    for testcase in directory {
        let testcasepath = testcase
            .expect("Failed to get the path of the test case!")
            .path();

        if !testcasepath.is_dir() {
            let testprogramsrc = {
                let testcasepath = &testcasepath;
                fs::read_to_string(testcasepath).unwrap_or_else(|err| {
                    panic!(
                        "Failed to read file: {} with error: {}",
                        testcasepath.display(),
                        err
                    );
                })
            };
            if compile(&testprogramsrc, testcasepath.to_str().unwrap()) {
                test_pass_count += 1;
            }
            test_cases += 1;
        }
    }

    println!("Test cases passed: {}/{}", test_pass_count, test_cases);
}

fn main() {
    let start = Instant::now();

    // let testpath =
    //     "/Users/flameberry/Developer/writing-a-c-compiler-tests/tests/chapter_3/invalid_parse";
    // run_tests(testpath);

    let testpath = "Sandbox/test_gen.c";
    compile_file(testpath);

    let end = Instant::now() - start;
    println!("Compilation took {:?}", end);
}
