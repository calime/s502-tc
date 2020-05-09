use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Range;

use super::formats::*;
use super::object::read_objects;
use super::script::read_script;

pub struct Linker {
    relocations: Vec<RelocGroup>,
    sections: HashMap<String, Section>,
    symbols: HashMap<String, usize>,
}

impl Linker {
    pub fn new<'a>(script: &'a str, objects: Vec<String>) -> Result<Self, String> {
        let (sections, symbols) = read_objects(objects)?;
        Ok(Linker {
            relocations: read_script(
                read_to_string(script).map_err(|_| "error reading linker script".to_string())?,
            )
            .map_err(|(line, err)| format!("error on line {}: {}", line, err))?,
            sections: sections,
            symbols: symbols,
        })
    }

    pub fn link(self) -> Result<(), String> {
        Ok(())
    }
}
