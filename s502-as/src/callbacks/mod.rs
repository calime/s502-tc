use super::ir::{Mnemonic::*, *};
use super::token::Token;
use logos::{Filter, Lexer};

pub mod labels;
pub mod mnemonics;
pub mod operands;

use crate::*;
pub use labels::*;
pub use mnemonics::*;
use opcodes::OPCODES;
pub use operands::*;
use OpState::*;
use OpVal::*;

/// Process an assembly statement at the end of each line and put it in the binary.
pub fn eol(lex: &mut Lexer<Token>) -> Filter<()> {
    // for inserting into current section
    macro_rules! insert_byte {
        ($byte:expr) => {{
            let idx = lex.extras.sections[&lex.extras.active].size;
            lex.extras
                .sections
                .get_mut(&lex.extras.active)
                .unwrap()
                .code[idx] = $byte;
            lex.extras
                .sections
                .get_mut(&lex.extras.active)
                .unwrap()
                .size += 1;
            if idx == 0xffff {
                lex.extras.err = "program is too large";
                return Filter::Emit(());
            }
        }};
    }
    macro_rules! insert_word {
        ($word:expr) => {{
            insert_byte!($word as u8);
            insert_byte!(($word >> 8) as u8);
        }};
    }

    let (ins, op) = match (lex.extras.ins.take(), lex.extras.op.take()) {
        // empty line
        (None, _) => return Filter::Skip,
        // get mnemonic and operand
        (Some(ins), op) => (ins, op.unwrap_or(Impl)),
    };

    // handle directives because they require specific operand types
    match ins {
        Dfb => {
            if let Plain(Byte(b)) = op {
                insert_byte!(b);
            } else if let Plain(Ref(rf)) = op {
                if rf.which_byte != ByteSelect::Both {
                    insert_byte!(0x00);
                } else {
                    lex.extras.err = "invalid operand type for dfb";
                    return Filter::Emit(());
                }
            } else {
                lex.extras.err = "invalid operand type for dfb";
                return Filter::Emit(());
            }
        }
        Dfw => {
            if let Plain(Word(w)) = op {
                insert_word!(w);
            } else if let Plain(Ref(rf)) = op {
                if rf.which_byte == ByteSelect::Both {
                    insert_word!(0x0000);
                } else {
                    lex.extras.err = "invalid operand type for dfw";
                    return Filter::Emit(());
                }
            } else {
                lex.extras.err = "invalid operand type for dfw";
                return Filter::Emit(());
            }
        }
        Sct => {
            if let Plain(Ref(rf)) = op {
                if rf.child.is_some() {
                    lex.extras.err = "sct directive requires a simple identifier name";
                    return Filter::Emit(());
                }

                lex.extras.active = rf.parent;
                if !lex.extras.sections.contains_key(&rf.parent) {
                    lex.extras.sections.insert(rf.parent, Section::default());
                }
            } else {
                lex.extras.err = "invalid operand type for sct";
                return Filter::Emit(());
            }
        }
        _ => (),
    }

    // get address mode for looking up opcode
    // and value to put into the binary
    let (mode, val) = match op.destruct() {
        Some(operand) => operand,
        None => {
            lex.extras.err = "invalid operand";
            return Filter::Emit(());
        }
    };

    // try to get the opcode
    insert_byte!(match OPCODES[ins][mode] {
        Some(opc) => opc,
        None => {
            lex.extras.err = "invalid instruction and address mode combination";
            return Filter::Emit(());
        }
    });

    // put in operand,
    // in the case of reference it just puts in a filler 0
    if mode != AddressMode::Impl && mode != AddressMode::Acc {
        match val {
            Byte(b) => insert_byte!(b),
            Word(w) => insert_word!(w),
            _ => unreachable!(),
        }
    }

    // reset machine for next string
    lex.extras.line += 1;
    lex.extras.vis = None;
    lex.extras.start_line = true;

    Filter::Skip
}
