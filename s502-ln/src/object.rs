use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Read};

use super::formats::*;

/// Reads all obhect files into a map of sections. Duplicate sections will
/// be merged in the order that they are read.
pub fn read_objects(files: Vec<String>) -> Result<HashMap<String, Section>, String> {
    let mut all_sections = HashMap::<String, Section>::with_capacity(5);
    for file in files {
        let obj_sections =
            read_object(&file).map_err(|_| format!("error reading object file {}", file))?;
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
                    for lab in &mut sect.labels {
                        lab.offset += existing.size;
                    }
                    for rf in &mut sect.references {
                        rf.offset += existing.size;
                    }
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

    Ok(all_sections)
}

/// Reads sections from one object.
fn read_object(file: &String) -> io::Result<HashMap<String, Section>> {
    let sections = HashMap::with_capacity(5);
    let mut obj_file = BufReader::with_capacity(0x10000, File::open(&file)?);
    let mut u32_buffer = [0u8; 4];
    let mut name_buffer = [0u8; 32];

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

        let mut labels = Vec::with_capacity(128);
        let mut references = Vec::with_capacity(128);
        // iterate over root labels
        for i in 0..num_labels {
            obj_file.read(&mut name_buffer)?;
            let lab_name = unsafe { String::from(std::str::from_utf8_unchecked(&name_buffer)) };
            println!("parent label {}", lab_name);
            obj_file.read(&mut u32_buffer)?;
            let num_children = u32::from_le_bytes(u32_buffer);
            obj_file.read(&mut u32_buffer)?;
            let offset = u32::from_le_bytes(u32_buffer) as usize;
            obj_file.read(&mut u32_buffer)?;
            let vis = num::FromPrimitive::from_u32(u32::from_le_bytes(u32_buffer))
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, ""))?;
            labels.push(Label {
                vis: vis,
                name: lab_name.clone(),
                offset: offset,
            });

            for _ in 0..num_children {
                obj_file.read(&mut name_buffer)?;
                let mut child_name = String::from(&lab_name);
                child_name.push('.');
                child_name.push_str(unsafe { std::str::from_utf8_unchecked(&name_buffer) });
                println!("child label {}", child_name);
                // offset
                obj_file.read(&mut u32_buffer)?;
                let offset = u32::from_le_bytes(u32_buffer) as usize;
                let vis = num::FromPrimitive::from_u32(u32::from_le_bytes(u32_buffer))
                    .ok_or(io::Error::new(io::ErrorKind::InvalidData, ""))?;
                labels.push(Label {
                    vis: vis,
                    name: child_name,
                    offset: offset,
                });
            }
        }

        for _ in 0..num_references {
            let mut ref_buffer = [0; 64];
            obj_file.read(&mut ref_buffer)?;
            let lab_name = unsafe { String::from(std::str::from_utf8_unchecked(&ref_buffer)) };
            println!("reference to label {}", lab_name);
            obj_file.read(&mut u32_buffer)?;
            let offset = u32::from_le_bytes(u32_buffer) as usize;
            obj_file.read(&mut u32_buffer)?;
            let which = num::FromPrimitive::from_u32(u32::from_le_bytes(u32_buffer))
                .ok_or(io::Error::new(io::ErrorKind::InvalidData, ""))?;
            references.push(Reference {
                referred: lab_name,
                offset: offset,
                which_byte: which,
            })
        }
        let mut sect = Section {
            labels: labels,
            references: references,
            size: sect_size,
            code: [0; 65536],
        };
        obj_file.read(&mut sect.code[0..sect_size])?;
        println!("{:x}", sect.code[0]);
    }

    Ok(sections)
}
