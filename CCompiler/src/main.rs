use flameberrycc::parser::{display_translationunit, Parser};
use flameberrycc::tokenizer::Tokenizer;
use std::{fs, time::Instant};

/// Returns true if the compilation succeeded else false
fn compile(src: &str, srcpath: &str) -> bool {
    let mut tokenizer = Tokenizer::new(src);

    // Display output for debugging
    // println!("Original:\n{}", src);
    // println!("Preprocessed:\n{}", preprocessed_src);

    // loop {
    //     match tokenizer.next_token() {
    //         Ok(Some(token)) => println!("{:?}", token),
    //         Ok(None) => break,
    //         Err(err) => panic!("{}", err),
    //     }
    // }

    let mut parser = Parser::new(&mut tokenizer);
    match parser.parse() {
        Ok(translation_unit) => {
            // println!("{:?}", translation_unit),
            display_translationunit(&translation_unit);
            true
        }
        Err(err) => {
            println!("{}:{}", srcpath, err);
            false
        }
    }
}

fn compile_file(srcpath: &str) {
    let src = fs::read_to_string(srcpath);
    println!("Compiling {}...", srcpath);
    compile(
        &src.unwrap_or_else(|err| {
            panic!(
                "Failed to read source file: {}: with error: {}",
                srcpath, err
            )
        }),
        srcpath,
    );
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

    let testpath = "/Users/flameberry/Developer/writing-a-c-compiler-tests/tests/chapter_2/valid";
    run_tests(testpath);

    // let testpath = "Sandbox/test.c";
    // compile_file(testpath);

    let end = Instant::now() - start;
    println!("Time taken: {:?}", end);
}
