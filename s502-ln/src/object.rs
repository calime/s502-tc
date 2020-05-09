use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

use super::formats::*;

/// Reads all object files, preserving the order in which everything is read.
///
/// This also accumulates the size of each section for use in calculating the
/// base addresses of each section.
pub fn read_objects(
    files: Vec<String>,
) -> Result<(HashMap<String, Section>, HashMap<String, usize>), String> {
    // TODO hashmap string -> usize section names to size
    let mut all_sections = HashMap::<String, Section>::with_capacity(5);
    let mut symbols = HashMap::with_capacity(128);
    for file in files {
        match Path::new(&file)
            .extension()
            .and_then(OsStr::to_str)
            .filter(|&s| s == "65o")
            .filter(|&s| s == "65s")
        {
            Some("65s") => {
                for (sym, addr) in read_symtab(&file)? {
                    if symbols.contains_key(&sym) {
                        return Err(format!(
                            "`{}` appears multiple times in the symbol tables",
                            sym
                        ));
                    }
                    let _ = symbols.insert(sym, addr);
                }
            }
            Some("65o") => {
                let obj_sections = read_object(&file)
                    .map_err(|_| format!("error reading object file {}", file))?;
                // merge new,y read sections with existing sections
                for (sect_name, mut sect) in obj_sections {
                    match all_sections.get_mut(&sect_name) {
                        Some(existing) => {
                            // ensure not too big
                            if existing.size + sect.size >= 0x10000 {
                                return Err(format!("section {} is too large", sect_name));
                            }
                            for i in 0..sect.size {
                                existing.code[i + existing.size] = sect.code[i];
                            }
                            // since sect will be appended to existing, add its offset
                            for (_, lab) in &mut sect.labels {
                                lab.offset += existing.size;
                            }
                            for rf in &mut sect.references {
                                rf.offset += existing.size;
                            }
                            // associate an object with which bytes it contributed to the section
                            existing
                                .objects
                                .insert(file.clone(), existing.size..(existing.size + sect.size));
                            existing.labels.extend(sect.labels);
                            existing.references.extend(sect.references);
                            existing.size += sect.size;
                        }
                        None => {
                            let _ = all_sections.insert(sect_name, sect);
                        }
                    }
                }
            }
            _ => {
                return Err(format!("file {} has wrong extension", file));
            }
        }
    }

    Ok((all_sections, symbols))
}

/// Reads sections from one object.
fn read_object(file: &String) -> io::Result<HashMap<String, Section>> {
    let mut sections = HashMap::with_capacity(5);
    let mut obj_file = BufReader::with_capacity(0x10000, File::open(&file)?);
    let mut u16_buffer = [0; 2];
    let mut u32_buffer = [0; 4];
    let mut name_buffer = [0; 32];
    let mut ref_buffer = [0; 64];

    // read num_sections
    obj_file.read(&mut u32_buffer)?;
    for _ in 0..u32::from_le_bytes(u32_buffer) {
        // read section header
        obj_file.read(&mut name_buffer)?;
        let sect_name = unsafe { String::from(std::str::from_utf8_unchecked(&name_buffer)) };
        obj_file.read(&mut u32_buffer)?;
        let sect_size = u32::from_le_bytes(u32_buffer) as usize;
        obj_file.read(&mut u32_buffer)?;
        let num_labels = u32::from_le_bytes(u32_buffer);
        obj_file.read(&mut u32_buffer)?;
        let num_references = u32::from_le_bytes(u32_buffer);

        let mut labels = HashMap::with_capacity(64);
        let mut references = Vec::with_capacity(64);
        // iterate over root labels
        for _ in 0..num_labels {
            // root label has name, num_children, offset, then visibility
            obj_file.read(&mut name_buffer)?;
            let lab_name = unsafe { String::from(std::str::from_utf8_unchecked(&name_buffer)) };
            obj_file.read(&mut u32_buffer)?;
            let num_children = u32::from_le_bytes(u32_buffer);
            obj_file.read(&mut u32_buffer)?;
            let offset = u32::from_le_bytes(u32_buffer) as usize;
            obj_file.read(&mut u32_buffer)?;
            let vis = num::FromPrimitive::from_u32(u32::from_le_bytes(u32_buffer))
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, ""))?;

            labels.insert(
                lab_name.clone(),
                Label {
                    vis: vis,
                    offset: offset,
                },
            );

            for _ in 0..num_children {
                // each child has name, offset, and visiobillity
                obj_file.read(&mut name_buffer)?;
                let mut child_name = String::with_capacity(64);
                child_name.push_str(lab_name.as_str());
                child_name.push('.');
                child_name.push_str(unsafe { std::str::from_utf8_unchecked(&name_buffer) });

                obj_file.read(&mut u32_buffer)?;
                let offset = u32::from_le_bytes(u32_buffer) as usize;
                obj_file.read(&mut u32_buffer)?;
                let vis = num::FromPrimitive::from_u32(u32::from_le_bytes(u32_buffer))
                    .ok_or(io::Error::new(io::ErrorKind::InvalidData, ""))?;

                labels.insert(
                    child_name,
                    Label {
                        vis: vis,
                        offset: offset,
                    },
                );
            }
        }

        for _ in 0..num_references {
            // reference has fully-qualified name, offset to put into,
            // which byte of the label address to take, and if the preceding byte
            // is a branch instruction
            obj_file.read(&mut ref_buffer)?;
            let lab_name = unsafe { String::from(std::str::from_utf8_unchecked(&ref_buffer)) };
            obj_file.read(&mut u32_buffer)?;
            let offset = u32::from_le_bytes(u32_buffer) as usize;
            obj_file.read(&mut u16_buffer)?;
            let which = num::FromPrimitive::from_u16(u16::from_le_bytes(u16_buffer))
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, ""))?;
            obj_file.read(&mut u16_buffer)?;
            let branch = match u16::from_le_bytes(u16_buffer) {
                1 => true,
                _ => false,
            };

            references.push(Reference {
                referred: lab_name,
                offset: offset,
                which_byte: which,
                branch: branch,
            })
        }

        let mut sect = Section {
            labels: labels,
            references: references,
            base: 0,
            size: sect_size,
            code: [0; 65536],
            objects: HashMap::with_capacity(3),
        };

        // pad to 4 bytes
        obj_file.read(&mut sect.code[0..(sect_size + ((4 - (sect_size & 3)) & 3))])?;
        sections.insert(sect_name, sect);
    }

    Ok(sections)
}

/// Reads a symbol table file.
fn read_symtab(file: &String) -> Result<HashMap<String, usize>, String> {
    let mut u32_buffer = [0; 4];
    let mut name_buffer = [0; 64];
    let mut symbols = HashMap::with_capacity(32);
    let mut sym_file = BufReader::with_capacity(
        0x1000,
        File::open(&file).map_err(|_| format!("error reading symbol file {}", file))?,
    );

    sym_file
        .read(&mut u32_buffer)
        .map_err(|_| format!("error reading symbol file {}", file))?;
    for _ in 0..u32::from_le_bytes(u32_buffer) {
        sym_file
            .read(&mut name_buffer)
            .map_err(|_| format!("error reading symbol file {}", file))?;
        sym_file
            .read(&mut u32_buffer)
            .map_err(|_| format!("error reading symbol file {}", file))?;
        let label = unsafe { String::from(std::str::from_utf8_unchecked(&name_buffer)) };
        let address = u32::from_le_bytes(u32_buffer) as usize;
        if symbols.contains_key(&label) {
            return Err(format!(
                "`{}` appears multiple times in the symbol tables",
                label
            ));
        }
        symbols.insert(label, address);
    }

    Ok(symbols)
}
