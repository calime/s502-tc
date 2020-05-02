#![feature(process_exitcode_placeholder)]

mod callbacks;
mod ir;
mod opcodes;
mod output;
mod token;

use std::process::ExitCode;

use output::*;

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
            clap::Arg::with_name("linker binary")
                .long("linker-bin")
                .takes_value(true)
                .help("Path to linker binary (default `s502-ln`)"),
        )
        .arg(
            clap::Arg::with_name("linker script")
                .short("l")
                .long("linker-script")
                .takes_value(true)
                .help("Linker script to pass to the linker (ignored if only assembling)"),
        )
        .arg(
            clap::Arg::with_name("output file")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("Name for output file (default <source>.65o)"),
        )
        .arg(
            clap::Arg::with_name("source")
                .required(true)
                .help("The source code file names (*.65a)"),
        )
        .get_matches();

    let file = arg_matches.value_of("source").unwrap();
    let out_file = arg_matches.value_of("output file");
    // let script = arg_matches.value_of("linker script");
    // let ln_bin = arg_matches.value_of("linker binary");
    // let asm_only = arg_matches.is_present("assemble only");
    // let symtab = arg_matches.is_present("symbols");
    // let combined_symtab = arg_matches.value_of("combined-symbols");

    // assemble the file
    if !asm(file.to_string(), out_file.map(|s| s.to_string())) {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
