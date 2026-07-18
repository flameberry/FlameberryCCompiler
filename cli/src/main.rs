use colored::Colorize;
use fbcc::compiler::Compiler;
use fbcc::core::errors::{CompilerError, CompilerErrorKind};
use std::io;
use std::{
    fs,
    path::{Path, PathBuf},
    time::Instant,
};

struct CliOptions {
    paths: Vec<PathBuf>,
    dump_ast: bool,
    dump_ir: bool,
    dump_asm: bool,
    emit_asm: bool,
    output: Option<PathBuf>,
}

impl CliOptions {
    fn new() -> Self {
        Self {
            paths: Vec::new(),
            dump_ast: false,
            dump_ir: false,
            dump_asm: false,
            emit_asm: false,
            output: None,
        }
    }
}

fn parse_cli(args: Vec<String>) -> Result<CliOptions, io::Error> {
    let mut cli_options = CliOptions::new();
    let mut args = args.into_iter();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--dump-ast" => cli_options.dump_ast = true,
            "--dump-ir" => cli_options.dump_ir = true,
            "--dump-asm" => cli_options.dump_asm = true,
            "--emit-asm" => cli_options.emit_asm = true,
            "-o" => {
                let path = args.next().ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "-o requires a path argument")
                })?;
                cli_options.output = Some(PathBuf::from(path));
            }

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

fn format_error(error: &CompilerError, path: &Path, line_str: &str) -> String {
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

fn compile_file(path: &PathBuf, cli_options: &CliOptions) -> bool {
    let source = fs::read_to_string(path).unwrap();
    let result = Compiler::new().compile(
        source.as_str(),
        cli_options.dump_ast,
        cli_options.dump_ir,
        cli_options.dump_asm,
    );

    match result {
        Ok(asm) => {
            // Write the `.s` when asked: `-o <path>`, else default to `<input>.s`.
            if cli_options.emit_asm || cli_options.output.is_some() {
                let out_path = cli_options
                    .output
                    .clone()
                    .unwrap_or_else(|| path.with_extension("s"));
                if let Err(e) = fs::write(&out_path, asm) {
                    eprintln!("failed to write {}: {e}", out_path.display());
                    return false;
                }
            }
            true
        }
        Err(error) => {
            if let Some(loc) = error.location {
                let line = source.lines().nth(loc.line - 1).unwrap();
                eprintln!("{}", format_error(&error, path, line));
            } else {
                eprintln!("error: {}", error.message);
            }
            false
        }
    }
}

fn run(cli_options: &CliOptions) -> bool {
    let mut success = true;
    for path in &cli_options.paths {
        success &= compile_file(&path, cli_options);
    }
    success
}

fn main() {
    let cli_options = parse_cli(std::env::args().skip(1).collect()).unwrap();
    let start = Instant::now();

    let success = run(&cli_options);

    let end = Instant::now() - start;
    println!("Compilation took {:?}", end);

    if !success {
        std::process::exit(1);
    }
}
