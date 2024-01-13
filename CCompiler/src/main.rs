use std::fs;

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

        // Actual compilation stages
        let preprocessed_src = flameberrycc::preprocessor::preprocess(&testprogramsrc);
        let tokens = flameberrycc::lexer::lex(&preprocessed_src);

        // Display output for debugging
        println!("{}:", testcasepath.display());
        println!("Original:\n{}", testprogramsrc);
        println!("Preprocessed:\n{}", preprocessed_src);

        println!("Lexed:");
        for tk in &tokens {
            println!("{:?}", tk);
        }
        println!();
    }
}

fn main() {
    let testpath = "/Users/flameberry/Developer/writing-a-c-compiler-tests/tests/chapter_1/valid";
    run_tests(testpath);
}
