use colored::{Color, Colorize};
use fbcc::analysis::node::Span;
use fbcc::compiler::Compiler;
use fbcc::core::errors::{CompilerError, CompilerErrorKind, Diagnostic, DiagnosticKind};
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
                let path = args
                    .next()
                    .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "-o requires a path argument"))?;
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

fn format_diagnostic(
    severity: &str,
    accent: Color,
    kind_str: &str,
    message: &str,
    span: Span,
    path: &Path,
    line_str: &str,
) -> String {
    let file = path.display();
    let (line, col) = (span.start.line, span.start.column);
    let mut out = String::new();

    out += &format!(
        "{}: {}\n",
        severity.bold().color(accent),
        format!("[{}] {}", kind_str, message).yellow()
    );

    out += &format!(
        "  {} {}\n",
        "-->".bright_blue(),
        format!("{file}:{line}:{col}").bold().white()
    );

    out += &format!("   {} {}\n", format!("{:>4} |", line).bright_black(), line_str);

    let line_len = line_str.chars().count();
    let start_idx = col.saturating_sub(1);

    let width = if span.end.line == span.start.line {
        span.end.column.saturating_sub(span.start.column)
    } else {
        line_len.saturating_sub(start_idx)
    };

    let width = width.min(line_len.saturating_sub(start_idx));

    let mut underline = String::new();
    for ch in line_str.chars().take(start_idx) {
        underline.push(if ch == '\t' { '\t' } else { ' ' });
    }
    // A column past the end of the line still needs its remaining padding.
    underline.push_str(&" ".repeat(start_idx.saturating_sub(line_len)));

    underline.push('^');
    underline.push_str(&"^".repeat(width.saturating_sub(1)));

    out += &format!("   {} {}\n", "     |".bright_black(), underline.color(accent));

    out
}

fn format_error(error: &CompilerError, path: &Path, line_str: &str) -> String {
    let span = error.span.unwrap_or_default();
    let kind_str = match error.kind {
        CompilerErrorKind::InternalError => "internal error",
        CompilerErrorKind::TokenizerError => "tokenizer error",
        CompilerErrorKind::SyntaxError => "syntax error",
        CompilerErrorKind::SemanticError => "semantic error",
    };

    format_diagnostic("error", Color::Red, kind_str, &error.message, span, path, line_str)
}

fn format_warning(warning: &Diagnostic, path: &Path, line_str: &str) -> String {
    let span = warning.span.unwrap_or_default();
    let kind_str = match warning.kind {
        DiagnosticKind::Warning => "warning",
    };

    format_diagnostic(
        "warning",
        Color::Yellow,
        kind_str,
        &warning.message,
        span,
        path,
        line_str,
    )
}

fn compile_file(path: &PathBuf, cli_options: &CliOptions) -> bool {
    let source = fs::read_to_string(path).unwrap();
    let (diagnostics, result) = Compiler::compile(
        source.as_str(),
        cli_options.dump_ast,
        cli_options.dump_ir,
        cli_options.dump_asm,
    );

    // Print any warnings collected during compilation (non-fatal).
    for warning in &diagnostics {
        let line = warning
            .span
            .and_then(|span| source.lines().nth(span.start.line.saturating_sub(1)))
            .unwrap_or("");
        eprintln!("{}", format_warning(warning, path, line));
    }

    match result {
        Ok(assembly) => {
            // Write the `.s` when asked: `-o <path>`, else default to `<input>.s`.
            if cli_options.emit_asm || cli_options.output.is_some() {
                let out_path = cli_options.output.clone().unwrap_or_else(|| path.with_extension("s"));
                if let Err(e) = fs::write(&out_path, assembly) {
                    eprintln!("failed to write {}: {e}", out_path.display());
                    return false;
                }
            }
            true
        }
        Err(error) => {
            if let Some(span) = error.span {
                // Saturate line 0 and tolerate out-of-range lines so a bogus
                // location degrades the diagnostic instead of panicking.
                let line = source.lines().nth(span.start.line.saturating_sub(1)).unwrap_or("");
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
