//! In addition to the Tilt and assembly files, the S502 Toolchain has two internal file formats.
//! The assembler outputs object files which are expected to have teh extention `.65o`,
//! and it has the following format:
//!
//! ### Object Header
//! ```
//! num:sections: u32
//! ```
//!
//! followed by sections:
//!
//! ### Section Header
//!

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

pub struct Section {
    pub code: [u8; 65536],
    pub labels: Vec<Label>,
    pub references: Vec<Reference>,
    pub size: usize,
}

#[derive(Clone)]
pub struct Label {
    pub vis: Visibility,
    pub name: String,
    pub offset: usize,
}

#[derive(Clone)]
pub struct Reference {
    pub referred: String,
    pub offset: usize,
    pub which_byte: ByteSelect,
}

#[derive(Clone, Copy, PartialEq, num_derive::FromPrimitive)]
#[repr(u32)]
pub enum ByteSelect {
    Both = 0,
    High = 1,
    Low = 2,
}

#[derive(Clone, Copy, num_derive::FromPrimitive)]
#[repr(u32)]
pub enum Visibility {
    Hidden = 0,
    Object = 1,
    Global = 2,
}

// pub struct Relocation {
//     offset: usize,
//     max_offset: Option<usize>,
// }

pub struct RelocGroup {
    pub relocations: Vec<String>,
    pub offset: usize,
    pub max_offset: Option<usize>,
}

// impl PartialEq for Relocation {
//     fn eq(&self, other: &Self) -> bool {
//         self.offset == other.offset
//     }
// }

// impl Eq for Relocation {}

// impl PartialOrd for Relocation {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl Ord for Relocation {
//     fn cmp(&self, other: &Relocation) -> Ordering {
//         self.offset.cmp(&other.offset)
//     }
// }

// impl PartialEq for Section {
//     fn eq(&self, other: &Self) -> bool {
//         self.name == other.name
//     }
// }

// impl Eq for Section {}

// impl Hash for Section {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.name.hash(state);
//     }
// }

// impl Section {
//     fn mergs(&mut self, other: Self) -> bool {
//         if self.size + other.size >= 0x10000 {
//             return false;
//         }
//         for i in 0..other.size {
//             self.code[i + self.size] = other.code[i];
//         }

//         true
//     }
// }
