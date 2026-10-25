//! Initial Jalapeño CLI. The synthesis toolkit is specified, not implemented yet.
use std::{env, process::ExitCode};

const HELP: &str = "Jalapeño — procedural audiovisual synthesis\n\n\
Usage: jalapeno [--help | --version]\n\n\
Options:\n  -h, --help     Show this help\n  -V, --version  Show the package version\n\n\
This initial scaffold does not render assets yet.\n\
The product scope and implementation roadmap live in the Decapod specs.";

fn main() -> ExitCode {
    let args: Vec<_> = env::args_os().skip(1).collect();
    match args.as_slice() {
        [] => println!("{HELP}"),
        [arg] if arg == "--help" || arg == "-h" => println!("{HELP}"),
        [arg] if arg == "--version" || arg == "-V" => {
            println!("jalapeno {}", env!("CARGO_PKG_VERSION"));
        }
        _ => {
            eprintln!("jalapeno: unsupported arguments; use --help");
            return ExitCode::from(2);
        }
    }
    ExitCode::SUCCESS
}
