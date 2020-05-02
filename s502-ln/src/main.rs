mod formats;
mod input;

use input::*;

fn main() {
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
}
