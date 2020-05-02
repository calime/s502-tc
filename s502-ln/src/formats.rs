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

pub struct Section {
    pub code: [u8; 65536],
    pub labels: Vec<Label>,
    pub references: Vec<Reference>,
    pub size: usize,
    pub last_parent: Option<usize>,
}

#[derive(Clone, Copy)]
pub struct Label {
    pub vis: Visibility,
    pub name: [u8; 32],
    pub num_children: u32,
    pub offset: usize,
}

#[derive(Clone, Copy)]
pub struct Reference {
    pub parent: [u8; 32],
    pub child: Option<[u8; 32]>,
    pub offset: usize,
    pub which_byte: ByteSelect,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum ByteSelect {
    Both = 0,
    High = 1,
    Low = 2,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Visibility {
    Hidden = 0,
    Object = 1,
    Global = 2,
}
