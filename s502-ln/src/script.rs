//! Linker scripts allow you to tell the linker how to organize the
//! sections in the object files.
//! ```
//! // set address to 0x1000 (default is 0)
//! . 0x1000
//! text
//! ```
//! This indicates that the `text` section is expected to be loaded
//! at address `0x1000` in memory.
//! ```
//! data 0x100
//! ```
//! Here the  `data` section immediately follows text. The `0x100` allows
//! the section to be a maximum of `0x100` bytes.
//! ```
//! one two 0x200
//! ```
//! Here the sections `one` and `two` *combined* are allowed `0x200` bytes.
//! ```
//! . 0x0f00
//! smth
//! ```
//! If the `smth` section is larger than `0x100` bytes then `text` will
//! overwrite it after address `0x1000`. You can set a size limit
//! as in the `data` section if you want to prevent that.
//! Because it is at the earliest offset, it will be at offset 0 in the
//! output binary file.

//
// You may also align a section:
// ```
// // align address to next 0x200 byte boundary (pad with 0's)
// . align 0x200
// some_other_section
// ```
// `align` is a context-sensitive keyword, so it's still a valid
// section name.

// 1. read script into hashmap<String, (u64, Option <u64>)>
// associate section name with offset (increase by size after each occurrence)
// and last allowed index if specified

// 2. read sections from objects, adding offset to each label and reference
// 2.5 read symbol tables into a labels hashset

// 3. merge labels into set from 2.5, and references into their own hashset, at this point check for duplicates

// 4. merge sections into one binary

// 5. resolve references. Check if byte before reference is a branch opcode then validate
// offset instead of using absolute address, and validate which_byte == both

// output binary from step 4

use std::collections::{BTreeSet, HashMap, HashSet};

use super::formats::RelocGroup;
use logos::{Lexer, Logos};

#[derive(Logos)]
// extras is (offset, line number)
#[logos(extras=(usize, usize))]
enum Token<'a> {
    #[token(".")]
    Period,
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

/// Reads a linker script into relocation groups.
pub fn read_script(script: String) -> Result<Vec<RelocGroup>, (usize, String)> {
    // let code = read_to_string(file).map_err(|_| "error reading linker script".to_string())?;
    let mut lexer = Token::lexer(&script);
    lexer.extras.1 += 1;
    let mut sect_names = HashSet::with_capacity(5);
    let mut reloc_table = Vec::with_capacity(5);

    while let Some(tok) = lexer.next() {
        match tok {
            Ident(sect) => reloc_table.push(read_group(&mut lexer, sect, &mut sect_names)?),
            Eol => lexer.extras.1 += 1,
            Period => set_offset(&mut lexer)?,
            _ => todo!(),
        }
    }

    // let idk = reloc_table.into_iter().collect::<BTreeSet<RelocGroup>>();

    Ok(reloc_table)
}

// Reads a line into a relocation group.
fn read_group<'a>(
    lexer: &mut Lexer<'a, Token<'a>>,
    first: &'a str,
    names: &mut HashSet<&'a str>,
) -> Result<RelocGroup, (usize, String)> {
    // ensure a section isn't listed more than once
    if !names.insert(first) {
        return Err((
            lexer.extras.1,
            format!(
                "section {} listed multiple times in the liinker script",
                first,
            ),
        ));
    }
    let mut group = RelocGroup {
        relocations: vec![first.to_string()],
        offset: lexer.extras.0,
        max_offset: None,
    };

    loop {
        match lexer.next() {
            Some(Ident(sect)) => {
                group.relocations.push(sect.to_string());
                if !names.insert(sect) {
                    println!(
                        "section {} listed multiple times in the liinker script",
                        sect
                    );
                }
            }
            Some(Number(size)) => group.max_offset = Some(group.offset + size),
            Some(Eol) => break,
            Some(Error) => {
                return Err((
                    lexer.extras.1,
                    format!("unrecognized token {}", lexer.slice()),
                ))
            }

            _ => {
                return Err((
                    lexer.extras.1,
                    format!("expected number or end of line, found `{}`", lexer.slice()),
                ))
            }
        }
    }
    Ok(group)
}

/// Reads a line setting the offset into memory.
fn set_offset<'a>(lexer: &mut Lexer<'a, Token<'a>>) -> Result<(), (usize, String)> {
    match lexer.next() {
        Some(Number(offset)) => lexer.extras.0 = offset,
        Some(Error) => {
            return Err((
                lexer.extras.1,
                format!("unrecognized token {}", lexer.slice()),
            ))
        }
        _ => {
            return Err((
                lexer.extras.1,
                "expected number in setting address".to_string(),
            ))
        }
    }
    Ok(())
}
