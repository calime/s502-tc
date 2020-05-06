#![feature(process_exitcode_placeholder)]

mod formats;
mod linker;
mod object;
mod script;

use std::collections::HashMap;
use std::process::ExitCode;

use linker::Linker;

fn main() -> ExitCode {
    let arg_matches = clap::App::new("s502-as 0.1")
        .arg(
            clap::Arg::with_name("output symbol tables")
                .short("s")
                .long("symbols")
                .help("Output a symbol table for each source file"),
        )
        .arg(
            clap::Arg::with_name("output combined symbol table")
                .short("c")
                .long("combined-symbols")
                .takes_value(true)
                .help("Output a single symbol table of all source files combined"),
        )
        .arg(
            clap::Arg::with_name("output file")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("Name for output file (default <source>.65o)"),
        )
        .arg(
            clap::Arg::with_name("linker script")
                .required(true)
                .help("Linker script to pass to the linker (ignored if only assembling)"),
        )
        .arg(
            clap::Arg::with_name("objects")
                .multiple(true)
                .required(true)
                .help("The object file names (*.65o)"),
        )
        .get_matches();

    let script = match arg_matches.value_of("linker script") {
        Some(script) => script,
        None => return ExitCode::FAILURE,
    };

    let linker = Linker::new(script, arg_matches.values_of_lossy("objects").unwrap()).expect("uh");

    ExitCode::SUCCESS
}
