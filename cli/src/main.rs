use colored::Colorize;
use fbcc::common::errors::{CompilerError, CompilerErrorKind};
use fbcc::compiler::Compiler;
use std::io;
use std::{
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

struct CliOptions {
    paths: Vec<PathBuf>,
    dump_ast: bool,
}

impl CliOptions {
    fn new() -> Self {
        Self {
            paths: Vec::new(),
            dump_ast: false,
        }
    }
}

fn parse_cli(args: Vec<String>) -> Result<CliOptions, io::Error> {
    let mut cli_options = CliOptions::new();
    for arg in args {
        match arg.as_str() {
            "--dump-ast" => cli_options.dump_ast = true,

            _ => {
                let path = PathBuf::from(arg);

                if !path.exists() {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        format!("Path does not exist: {}", path.to_str().unwrap()),
                    ));
                }

                cli_options.paths.push(path);
            }
        }
    }
    Ok(cli_options)
}

pub fn format_error(error: &CompilerError, path: &Path, line_str: &str) -> String {
    let (line, col) = error.location.map_or((0, 0), |loc| (loc.line, loc.column));
    let file = path.display();
    let mut out = String::new();
    let kind_str = match error.kind {
        CompilerErrorKind::InternalError => "internal error",
        CompilerErrorKind::TokenizerError => "tokenizer error",
        CompilerErrorKind::SyntaxError => "syntax error",
        CompilerErrorKind::SemanticError => "semantic error",
    };

    // Final error line
    out += &format!(
        "{}: {}\n",
        "error".bold().red(),
        format!("[{}] {}", kind_str, error.message).yellow()
    );

    // Label line
    out += &format!(
        "  {} {}\n",
        "-->".bright_blue(),
        format!("{file}:{line}:{col}").bold().white()
    );

    // Source code line with gutter
    out += &format!("   {} {}\n", format!("{:>4} |", line).bright_black(), line_str);

    // Underline with caret (1-based to 0-based column fix)
    let mut underline = String::new();
    underline.push_str(&" ".repeat(col - 1));
    underline.push('^');

    out += &format!("   {} {}\n", "     |".bright_black(), underline.red());

    out
}

fn compile_file(path: &PathBuf, cli_options: &CliOptions) {
    let source = fs::read_to_string(path).unwrap();
    let result = Compiler::new().compile(source.as_str(), cli_options.dump_ast);

    if let Err(error) = result {
        if let Some(loc) = error.location {
            let line = source.lines().nth(loc.line - 1).unwrap();
            eprintln!("{}", format_error(&error, path, line));
        }
    }
}

fn run(cli_options: &CliOptions) {
    for path in &cli_options.paths {
        compile_file(&path, cli_options);
    }
}

fn main() {
    let cli_options = parse_cli(std::env::args().skip(1).collect()).unwrap();
    let start = Instant::now();

    run(&cli_options);

    let end = Instant::now() - start;
    println!("Compilation took {:?}", end);
}
