use flameberrycc::syntax_analyzer::{display_translationunit, SyntaxAnalyzer};
use flameberrycc::tokenizer::Tokenizer;
use std::{fs, time::Instant};

fn compile(src: &str, srcpath: &str) {
    let preprocessed_src = flameberrycc::preprocessor::preprocess(&src);
    let mut tokenizer = Tokenizer::new(&preprocessed_src);

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

    let mut parser = SyntaxAnalyzer::new(&mut tokenizer);
    match parser.parse() {
        Ok(translation_unit) => display_translationunit(&translation_unit),
        // Ok(translation_unit) => println!("{:?}", translation_unit),
        Err(err) => println!("{}:{}", srcpath, err),
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
    for testcase in directory {
        let testcasepath = testcase
            .expect("Failed to get the path of the test case!")
            .path();

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

        compile(&testprogramsrc, testcasepath.to_str().unwrap());
    }
}

fn main() {
    let start = Instant::now();

    // let testpath = "/Users/flameberry/Developer/writing-a-c-compiler-tests/tests/chapter_1/valid";
    // run_tests(testpath);

    let testpath = "Sandbox/test.c";
    compile_file(testpath);

    let end = Instant::now() - start;
    println!("Time taken: {:?}", end);
}
