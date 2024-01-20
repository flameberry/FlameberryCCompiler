pub fn preprocess(source: &str) -> String {
    let mut out = String::new();
    // Reserve memory to avoid allocations per line for big source files
    out.reserve(source.bytes().len());

    for line in source.split('\n') {
        // Trim to the start so that we can check for // comments or # preprocessor directives
        let trimmed = line.trim_start();

        // Ignore comments
        if trimmed.starts_with("//") {
        }
        // Expand the preprocessor macros
        else if trimmed.starts_with('#') {
            panic!("Preprocessor directives not supported yet!");
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    // Remove the unncessary '\n' at the end of the program
    out.pop();
    // Return the preprocessed source
    out
}
