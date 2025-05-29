use fbcc::compile;
use std::io;
use std::{
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

fn run(cli_options: &CliOptions) {
    for path in &cli_options.paths {
        compile(path, cli_options.dump_ast);
    }
}

fn main() {
    let cli_options = parse_cli(std::env::args().skip(1).collect()).unwrap();
    let start = Instant::now();

    run(&cli_options);

    let end = Instant::now() - start;
    println!("Compilation took {:?}", end);
}
