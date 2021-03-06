use super::ir::{AddressMode, AddressMode::*, Mnemonic, Mnemonic::*};
use enum_map::{enum_map, EnumMap};
use lazy_static::lazy_static;

lazy_static! {
/// Lookup opcode based on mnemonic and address mode. None means an invalid combination.
pub static ref OPCODES: EnumMap<Mnemonic, EnumMap<AddressMode, Option<u8>>> = enum_map! {
    Adc => enum_map! {Acc => None, Abs => Some(0x6d), AbsX => Some(0x7d), AbsY => Some(0x79), Imme => Some(0x69), Impl => None, Ind => None, Xind => Some(0x61), IndY => Some(0x71), Zpg => Some(0x65), ZpgX => Some(0x75), ZpgY => None},
    And => enum_map! {Acc => None, Abs => Some(0x2d), AbsX => Some(0x3d), AbsY => Some(0x39), Imme => Some(0x29), Impl => None, Ind => None, Xind => Some(0x21), IndY => Some(0x31), Zpg => Some(0x25), ZpgX => Some(0x35), ZpgY => None},
    Asl => enum_map! {Acc => Some(0x0a), Abs => Some(0x0e), AbsX => Some(0x1e), AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0x06), ZpgX => Some(0x16), ZpgY => None},
    Bcc => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0x90), ZpgX => None, ZpgY => None},
    Bcs => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0xb0), ZpgX => None, ZpgY => None},
    Beq => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0xf0), ZpgX => None, ZpgY => None},
    Bit => enum_map! {Acc => None, Abs => Some(0x2c), AbsX => None, AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0x24), ZpgX => None, ZpgY => None},
    Bmi => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0x30), ZpgX => None, ZpgY => None},
    Bne => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0xd0), ZpgX => None, ZpgY => None},
    Bpl => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0x10), ZpgX => None, ZpgY => None},
    Brk => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0x00), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Bvc => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0x50), ZpgX => None, ZpgY => None},
    Bvs => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0x70), ZpgX => None, ZpgY => None},
    Clc => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0x18), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Cld => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0xd8), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Cli => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0x58), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Clv => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0xb8), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Cmp => enum_map! {Acc => None, Abs => Some(0xcd), AbsX => Some(0xdd), AbsY => Some(0xd9), Imme => Some(0xc9), Impl => None, Ind => None, Xind => Some(0xc1), IndY => Some(0xd1), Zpg => Some(0xc5), ZpgX => Some(0xd5), ZpgY => None},
    Cpx => enum_map! {Acc => None, Abs => Some(0xec), AbsX => None, AbsY => None, Imme => Some(0xe0), Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0xe4), ZpgX => None, ZpgY => None},
    Cpy => enum_map! {Acc => None, Abs => Some(0xcc), AbsX => None, AbsY => None, Imme => Some(0xc0), Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0xc4), ZpgX => None, ZpgY => None},
    Dec => enum_map! {Acc => None, Abs => Some(0xce), AbsX => Some(0xde), AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0xc6), ZpgX => Some(0xd6), ZpgY => None},
    Dex => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0xca), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Dey => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0x88), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Eor => enum_map! {Acc => None, Abs => Some(0x4d), AbsX => Some(0x5d), AbsY => Some(0x59), Imme => Some(0x49), Impl => None, Ind => None, Xind => Some(0x41), IndY => Some(0x51), Zpg => Some(0x45), ZpgX => Some(0x55), ZpgY => None},
    Inc => enum_map! {Acc => None, Abs => Some(0xee), AbsX => Some(0xfe), AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0xe6), ZpgX => Some(0xf6), ZpgY => None},
    Inx => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0xe8), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Iny => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0xc8), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Jmp => enum_map! {Acc => None, Abs => Some(0x4c), AbsX => None, AbsY => None, Imme => None, Impl => None, Ind => Some(0x6c), Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Jsr => enum_map! {Acc => None, Abs => Some(0x20), AbsX => None, AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Lda => enum_map! {Acc => None, Abs => Some(0xad), AbsX => Some(0xbd), AbsY => Some(0xb9), Imme => Some(0xa9), Impl => None, Ind => None, Xind => Some(0xa1), IndY => Some(0xb1), Zpg => Some(0xa5), ZpgX => Some(0xb5), ZpgY => None},
    Ldx => enum_map! {Acc => None, Abs => Some(0xae), AbsX => Some(0xbe), AbsY => None, Imme => Some(0xa2), Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0xa6), ZpgX => Some(0xb6), ZpgY => Some(0xb6)},
    Ldy => enum_map! {Acc => None, Abs => Some(0xac), AbsX => Some(0xbc), AbsY => None, Imme => Some(0xa0), Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0xa4), ZpgX => Some(0xb4), ZpgY => None},
    Lsr => enum_map! {Acc => Some(0x4a), Abs => Some(0x4e), AbsX => Some(0x5e), AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0x46), ZpgX => Some(0x56), ZpgY => None},
    Nop => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0xea), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Ora => enum_map! {Acc => None, Abs => Some(0x0d), AbsX => Some(0x1d), AbsY => Some(0x19), Imme => Some(0x09), Impl => None, Ind => None, Xind => Some(0x01), IndY => Some(0x11), Zpg => Some(0x05), ZpgX => Some(0x15), ZpgY => None},
    Pha => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0x48), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Php => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0x08), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Pla => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0x68), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Plp => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0x28), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Rol => enum_map! {Acc => Some(0x2a), Abs => Some(0x2e), AbsX => Some(0x3e), AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0x26), ZpgX => Some(0x36), ZpgY => None},
    Ror => enum_map! {Acc => Some(0x6a), Abs => Some(0x6e), AbsX => Some(0x7e), AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0x66), ZpgX => Some(0x76), ZpgY => None},
    Rti => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0x40), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Rts => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0x60), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Sbc => enum_map! {Acc => None, Abs => Some(0xed), AbsX => Some(0xfd), AbsY => Some(0xf9), Imme => Some(0xe9), Impl => None, Ind => None, Xind => Some(0xe1), IndY => Some(0xf1), Zpg => Some(0xe5), ZpgX => Some(0xf5), ZpgY => None},
    Sec => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0x38), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Sed => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0xf8), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Sei => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0x78), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Sta => enum_map! {Acc => None, Abs => Some(0x8d), AbsX => Some(0x9d), AbsY => Some(0x99), Imme => None, Impl => None, Ind => None, Xind => Some(0x81), IndY => Some(0x91), Zpg => Some(0x85), ZpgX => Some(0x95), ZpgY => None},
    Stx => enum_map! {Acc => None, Abs => Some(0x8e), AbsX => None, AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0x86), ZpgX => None, ZpgY => Some(0x96)},
    Sty => enum_map! {Acc => None, Abs => Some(0x8c), AbsX => None, AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => Some(0x84), ZpgX => Some(0x94), ZpgY => None},
    Tax => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0xaa), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Tay => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0xa8), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Tsx => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0xba), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Txa => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0x8a), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Txs => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0x9a), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Tya => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => Some(0x98), Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Dfb => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Dfw => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Sct => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
    Hlt => enum_map! {Acc => None, Abs => None, AbsX => None, AbsY => None, Imme => None, Impl => None, Ind => None, Xind => None, IndY => None, Zpg => None, ZpgX => None, ZpgY => None},
};
}
