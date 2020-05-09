//! Linker scripts allow you to tell the linker how to organize the
//! sections in the object files.
//! ```text
//! // single line comments are allowed
//! 0x1000 text
//! ```
//! This indicates that the `text` section is expected to be loaded
//! at address `0x1000` in memory. If no start address is give at the beginning of
//! the script then the default is 0.
//! ```text
//! data 0x100
//! ```
//! Here the  `data` section immediately follows text. The `0x100` allows
//! the section to be a maximum of `0x100` bytes.
//! ```text
//! one two 0x200
//! ```
//! Here the sections `one` and `two` *combined* are allowed `0x200` bytes. `two`
//! immediately follows `one`.
//! ```text
//! 0x0f00 something
//! ```
//! If the `something` section is larger than `0x100` bytes then it will overwrite the
//! beginning of `text` because it is listed later in the script. You can set a size
//! limit as in the `data` section if you want to prevent that. Because it is at the
//! earliest address, it will be at offset 0 in the output binary file.

// NOTE probably not going to introduce this features
// You may also align a section:
// ```text
// // align address to next 0x200 byte boundary (pad with 0's)
// align 0x200 some_other_section
// ```
// `align` is a context-sensitive keyword, so it's still a valid
// section name.

use std::collections::HashSet;

use super::formats::RelocGroup;
use logos::{Lexer, Logos};

/// The tokens recognized in the linker script.
#[derive(Logos)]
// extras is (offset, line number)
#[logos(extras=(usize, usize))]
enum Token<'a> {
    // treat comment as eol cause it goes up to eol
    #[regex("//.*\n")]
    #[token("\n")]
    Eol,
    #[regex("0x[0-9a-fA-F]+", |lex| u16::from_str_radix(&lex.slice()[2..], 16).map(|num| num as usize))]
    #[regex("[0-9]+", |lex| u16::from_str_radix(lex.slice(), 10).map(|num| num as usize))]
    Number(usize),
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| if lex.slice().len() > 31 { None } else { Some(lex.slice()) })]
    Ident(&'a str),
    #[error]
    // unimportant whitespace
    #[regex(r"[ \t]+", logos::skip)]
    Error,
}

use Token::*;

// TODO maybe also create first and last part of address space at this point as a range
/// Reads a linker script into relocation groups.
pub fn read_script(script: String) -> Result<Vec<RelocGroup>, (usize, String)> {
    let mut lexer = Token::lexer(&script);
    // start line number at 1
    lexer.extras.1 += 1;
    //keep a set of sections names to ensure it's not listed multiple times
    let mut sect_names = HashSet::with_capacity(5);
    // list of relocation groups
    let mut reloc_table = Vec::with_capacity(5);

    // read beginning of line
    while let Some(tok) = lexer.next() {
        match tok {
            // start of group list
            Ident(_) | Number(_) => reloc_table.push(read_group(&mut lexer, tok, &mut sect_names)?),
            // empty line
            Eol => lexer.extras.1 += 1,
            Error => {
                return Err((
                    lexer.extras.1,
                    format!("unrecognized token {}", lexer.slice()),
                ))
            }
        }
    }

    Ok(reloc_table)
}

/// Reads a line into a relocation group.
fn read_group<'a>(
    lexer: &mut Lexer<'a, Token<'a>>,
    start: Token<'a>,
    names: &mut HashSet<&'a str>,
) -> Result<RelocGroup, (usize, String)> {
    let mut group = RelocGroup {
        relocations: Vec::with_capacity(2),
        address: lexer.extras.0,
        max_size: None,
    };

    // check first token of the line
    match start {
        // line begins with explicit address
        Number(addr) => group.address = addr,
        // begins with first section
        Ident(first) => {
            // ensure a section isn't listed more than once
            if !names.insert(first) {
                return Err((
                    lexer.extras.1,
                    format!(
                        "section {} listed multiple times in the linker script",
                        first,
                    ),
                ));
            }
            group.relocations.push(first.to_string());
        }
        // this function is not called for other variants
        _ => unsafe { std::hint::unreachable_unchecked() },
    };

    // read the rest of tbe line
    loop {
        match lexer.next() {
            // another section
            Some(Ident(sect)) => {
                if !names.insert(sect) {
                    return Err((
                        lexer.extras.1,
                        format!(
                            "section {} listed multiple times in the linker script",
                            sect
                        ),
                    ));
                }
                group.relocations.push(sect.to_string());
            }
            // max size given
            Some(Number(size)) => {
                group.max_size = Some(size);
                // expect Eol next
                match lexer.next() {
                    Some(Eol) | None => break Ok(group),
                    _ => {
                        return Err((
                            lexer.extras.1,
                            format!(
                                "expected end of line after group size, found {}",
                                lexer.slice()
                            ),
                        ));
                    }
                }
            }
            Some(Error) => {
                return Err((
                    lexer.extras.1,
                    format!("unrecognized token {}", lexer.slice()),
                ))
            }
            Some(Eol) | None => break Ok(group),
        }
    }
}
