//! In addition to the Tilt and assembly files, the S502 Toolchain has two internal file formats.
//!
//! Object files contain code symbol information for the linker to combine into a single binary.
//! Symbol files contain labels and their addresses in memory extracted from previously linked
//! binaries.
//!
//! The assembler outputs object files which are expected to have the extension `.65o`,
//! and it has the following format (all ASCII bytes strings are null-terminated):
//!
//! ### Object Header
//! ```text
//! num_sections: u32
//! ```
//!
//! followed by sections:
//!
//! ### Section Header
//! ```text
//! name: 32 ASCII bytes
//! size: u32
//! num_parents: u32
//! num_references: u32
//! ```
//! `name` is a simple identifier. `size` refers to the number of bytes in its payload.
//! `num_parents` is the number of parent labels in the section, and `num_references` is
//! the number of references to labels in the section.
//!
//! Following the section header is a flattened tree of parent and child labels, and they appear
//! in order of their offset into the section. The first label is guaranteed to be a parent
//! with the form:
//!
//! ### Parent Label
//! ```text
//! name: 32 ASCII bytes
//! num_children: u32
//! offset: u32
//! visibility: u32
//! ```
//! `offset` is the offset into the section's payload that this label is located at.
//! `visibility`  dictates from where this label may be referred to:
//! ```text
//! 1 -> from anywhere in this object
//! 2 -> globally from any object
//! ```
//! After a parent label is a number of child labels with the form:
//!
//! ### Child Label
//! ```text
//! name: 32 ASCII bytes
//! offset: u32
//! visibility: u32
//! ```
//! `offset` is the same as in the parent label, though `visibility` is slightly changed:
//! ```text
//! 0 -> May only be referred to from under the same parent. That is,
//!   let ref = offset of reference to this label
//!   let parent = offset of this label's parent
//!   let next = offset of the next parent label
//!   (parent <= ref < next) must hold in order to resolve the reference.
//! 1 -> from anywhere in this object
//! 2 -> globally from any object
//! ```
//! After the child labels of a parent is the next parent label.
//!
//! After all of the labels is the reference block. Each reference has the form:
//!
//! ### Reference
//! ```text
//! referred: 64 ASCII bytes
//! offset: u32
//! which_byte: u16
//! branch: u16
//! ```
//! `referred` is the fully qualified name of the label being referred to. If it refers to
//! a child label then it will follow the parent label and a period, for example `reset.skip_config`.
//! `offset` is the offset into the section at which to put the address of the referred label.
//! `which_byte` dictates which byte from the label's address to put into `offset`:
//! ```text
//! 0 -> both
//! 1 -> high byte
//! 2 -> low byte
//! ```
//! `branch` tells if the opcode before it was a branch instruction. If its value is `1` then
//! `which_byte` is ignored and the difference between `reference's offset + 1` and the label's
//! address is inserted, and it must fit in one signed byte.
//!
//! After the references is the section's payload. Only `sect_header.size` bytes are significant
//! but it is padded to a 4 byte boundary. The number of bytes to read may be calculated
//! `size + ((4 - (size & 3)) & 3)`.
//!
//! ## Symbol Table
//! The linker may output symbol tables while linking objects together. This is convenient for
//! resolving references to binaries that were linked separately and loaded elsewhere in memory
//! without having to link against the actual object. The symbol table has the form:
//!
//! ### Symbol Table Header
//! ```text
//! num:symbols: u32
//! ```
//!
//! ###Symbol
//! ```text
//! name: 64 ASCII bytes
//! address: u32
//! ```

use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Range;

/// A section as read from the object file.
pub struct Section {
    pub code: [u8; 65536],
    pub labels: HashMap<String, Label>,
    pub references: Vec<Reference>,
    /// The base address of this section.
    pub base: usize,
    pub size: usize,
    /// The objects from which this section was taken.
    pub objects: HashMap<String, Range<usize>>,
}

#[derive(Clone)]
pub struct Label {
    pub vis: Visibility,
    pub offset: usize,
}

#[derive(Clone)]
pub struct Reference {
    pub referred: String,
    pub offset: usize,
    pub which_byte: ByteSelect,
    pub branch: bool,
}

#[derive(Clone, Copy, PartialEq, num_derive::FromPrimitive)]
#[repr(u32)]
pub enum ByteSelect {
    Both = 0,
    High = 1,
    Low = 2,
}

#[derive(Clone, Copy, PartialEq, num_derive::FromPrimitive)]
#[repr(u32)]
pub enum Visibility {
    Hidden = 0,
    Object = 1,
    Global = 2,
}

/// A group of section relocations.
///
/// All sections listed in the same line gets put into one group. Groups without
/// a unique offset will have the same offset as the previous group and it will be
/// adjusted later.
pub struct RelocGroup {
    /// The sections in this group in the order they're listed.
    pub relocations: Vec<String>,
    // The offset the group starts at.
    pub address: usize,
    /// Maximum allowed size of the group if specified.
    pub max_size: Option<usize>,
}

impl PartialEq for RelocGroup {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}

impl Eq for RelocGroup {}

impl PartialOrd for RelocGroup {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for RelocGroup {
    fn cmp(&self, other: &RelocGroup) -> Ordering {
        self.address.cmp(&other.address)
    }
}
