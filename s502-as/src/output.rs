use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use super::callbacks::eol;
use super::ir::*;
use super::token::Token;
use logos::{Filter, Logos};

/// Assemble a file and output its object.
pub fn asm(mut name: String, out_file: Option<String>) -> bool {
    // validate assembly extention
    if let None = Path::new(&name)
        .extension()
        .and_then(OsStr::to_str)
        .filter(|&s| s == "65a")
    {
        eprintln!("source file {} has wrong extension", name);
        return false;
    }
    // then get source
    let code = match std::fs::read_to_string(&name) {
        Ok(code) => code,
        Err(_) => {
            eprintln!("error reading file {}", name);
            return false;
        }
    };

    let mut lexer = Token::lexer(code.as_str());
    // all tokens are skipped on success, so an error emits a token
    match lexer.next() {
        Some(Token::Error) => {
            println!(
                "error on line {}: unrecognized token `{}`",
                lexer.extras.line,
                lexer.slice()
            );
            return false;
        }
        Some(_) => {
            println!("error on line {}: {}", lexer.extras.line, lexer.extras.err);
            return false;
        }
        None => (),
    }

    // if last line has no newline, process it manually,
    // if it does have a newline all registers will be empty and nothing will happen
    if let Filter::Emit(()) = eol(&mut lexer) {
        println!("error on line {}: {}", lexer.extras.line, lexer.extras.err);
        return false;
    }

    // then output an object for this program
    if let Err(_) = create_object(
        lexer.extras.sections,
        match out_file {
            None => {
                let _ = name.pop();
                name.push('o');
                name
            }
            Some(out) => out,
        },
    ) {
        eprintln!("error writing object file");
        return false;
    }

    true
}

/// Output a program to an object file.
pub fn create_object(sections: HashMap<[u8; 32], Section>, name: String) -> io::Result<()> {
    let mut obj_file = BufWriter::with_capacity(0x10000, File::create(name)?);
    // object header
    obj_file.write(&(sections.len() as u32).to_le_bytes())?;

    for (name, sect) in sections.into_iter() {
        // section header
        obj_file.write(&name)?;
        obj_file.write(&(sect.size as u32).to_le_bytes())?;
        obj_file.write(&(sect.num_parents as u32).to_le_bytes())?;
        obj_file.write(&(sect.references.len() as u32).to_le_bytes())?;

        // label block
        let mut label_iter = sect.labels.into_iter();
        for _ in 0..sect.num_parents {
            let parent = label_iter.next().unwrap();
            obj_file.write(&parent.name)?;
            obj_file.write(&(parent.num_children as u32).to_le_bytes())?;
            obj_file.write(&(parent.offset as u32).to_le_bytes())?;
            obj_file.write(&(parent.vis as u32).to_le_bytes())?;

            for _ in 0..(parent.num_children as usize) {
                let child = label_iter.next().unwrap();
                obj_file.write(&child.name)?;
                obj_file.write(&(parent.offset as u32).to_le_bytes())?;
                obj_file.write(&(parent.vis as u32).to_le_bytes())?;
            }
        }
        // reference block
        for rf in sect.references.into_iter() {
            // truncate padding
            let mut referred_buffer = [0; 64];
            let parent_length = rf.parent.iter().position(|&c| c == 0x00).unwrap();
            for i in 0..parent_length {
                referred_buffer[i] = rf.parent[i];
            }
            if let Some(child) = rf.child {
                referred_buffer[parent_length] = b'.';
                // guaranteed to have null terminator
                for i in 0..31 {
                    referred_buffer[parent_length + i + 1] = child[i];
                }
            }
            obj_file.write(&referred_buffer)?;
            obj_file.write(&(rf.offset as u32).to_le_bytes())?;
            obj_file.write(&(rf.which_byte as u16).to_le_bytes())?;
            obj_file.write(&(rf.branch as u16).to_le_bytes())?;
        }

        // pad code to 4 bytes
        obj_file.write(&sect.code[0..(sect.size + ((4 - (sect.size & 3)) & 3))])?;
    }

    obj_file.flush()?;
    Ok(())
}

// Invoke the linker.
// pub fn link<'a>(
//     files: Vec<String>,
//     symtab: bool,
//     combined_symtab: Option<&'a str>,
//     bin: &'a str,
//     script: &'a str,
// ) -> bool {
//     // validate linker script extention
//     if let None = Path::new(script)
//         .extension()
//         .and_then(OsStr::to_str)
//         .filter(|&s| s == "json")
//     {
//         eprintln!("linker script {} has wrong extension", script);
//         return false;
//     }
//     let mut args = Vec::with_capacity(files.len() + 4);
//     if symtab {
//         args.push("-s".to_string());
//     }
//     if let Some(combined) = combined_symtab {
//         args.push("-c".to_string());
//         args.push(combined.to_string());
//     }
//     args.push(script.to_string());
//     args.extend(files);
//     let args = args.iter().map(|s| s.as_str()).collect::<Vec<&str>>();

//     print!("{} ", bin);
//     for arg in &args {
//         print!("{} ", arg);
//     }
//     match match Command::new("echo").spawn() {
//         Ok(child) => child,
//         _ => return false,
//     }
//     .wait()
//     {
//         Ok(status) => status.success(),
//         _ => false,
//     }
// }
