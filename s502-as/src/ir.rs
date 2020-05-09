use enum_map::Enum;
use std::collections::HashMap;

pub struct Program {
    pub line: usize,
    pub sections: HashMap<[u8; 32], Section>,
    pub active: Option<[u8; 32]>,
    /// Visibility for current label.
    pub vis: Option<Visibility>,
    pub start_line: bool,
    pub ins: Option<Mnemonic>,
    pub op: Option<OpState>,
    pub err: &'static str,
}

pub struct Section {
    pub code: [u8; 65536],
    pub labels: Vec<Label>,
    pub references: Vec<Reference>,
    pub size: usize,
    pub last_parent: Option<usize>,
    pub num_parents: usize,
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
    pub branch: bool,
}

#[derive(Clone, Copy, PartialEq)]
#[repr(u16)]
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

pub enum OpState {
    /// Beginning of an indirect operand `(`.
    StartInd,
    /// Beginning of an immediate operand `#`.
    StartImme,
    /// Undecorated operand, may become absolute, zeropage, or relative.
    Plain(OpVal),
    /// Operand will be indirect `(val`.
    MaybeInd(IndOp),
    /// Operand will be indexed `val,`
    Idx(OpVal),

    /// Accumulator operand
    Acc,
    AbsX(OpVal),
    AbsY(OpVal),
    Imme(OpVal),
    /// May be abs,x or zpg,x depending on which_byte
    Ind(OpVal),
    Xind(OpVal),
    IndY(OpVal),
    ZpgX(OpVal),
    ZpgY(OpVal),
    Impl,
}

/// The type of operand that can appear inside parentheses.
pub enum IndOp {
    /// Either `($HHLL` or `$(LL`.
    Other(OpVal),
    /// Currently has the form `($LL,x`.
    Xind(OpVal),
    /// Currently has the form `($LL,`.
    XindComma(OpVal),
    /// Currently has the form `($LL)`.
    IndY(OpVal),
    /// Currently has the form `($LL),`.
    IndYComma(OpVal),
}

/// The type the operand's value can be.
#[derive(Clone, Copy)]
pub enum OpVal {
    Byte(u8),
    Word(u16),
    Ref(Reference),
}

impl Default for Program {
    fn default() -> Self {
        let mut prog = Program {
            line: 1,
            sections: HashMap::with_capacity(3),
            active: None,
            vis: None,
            start_line: true,
            ins: None,
            op: None,
            err: "",
        };

        prog.sections.insert([0; 32], Section::default());

        prog
    }
}

impl Default for Section {
    fn default() -> Self {
        Section {
            code: [0; 65536],
            labels: Vec::with_capacity(64),
            references: Vec::with_capacity(64),
            size: 0,
            last_parent: None,
            num_parents: 0,
        }
    }
}

/// A mnemonic, both for instructions and directives.
#[derive(Enum)]
pub enum Mnemonic {
    Adc,
    And,
    Asl,
    Bcc,
    Bcs,
    Beq,
    Bit,
    Bmi,
    Bne,
    Bpl,
    Brk,
    Bvc,
    Bvs,
    Clc,
    Cld,
    Cli,
    Clv,
    Cmp,
    Cpx,
    Cpy,
    Dec,
    Dex,
    Dey,
    Eor,
    Inc,
    Inx,
    Iny,
    Jmp,
    Jsr,
    Lda,
    Ldx,
    Ldy,
    Lsr,
    Nop,
    Ora,
    Pha,
    Php,
    Pla,
    Plp,
    Rol,
    Ror,
    Rti,
    Rts,
    Sbc,
    Sec,
    Sed,
    Sei,
    Sta,
    Stx,
    Sty,
    Tax,
    Tay,
    Tsx,
    Txa,
    Txs,
    Tya,
    Dfb,
    Dfw,
    Hlt,
    Sct,
}

impl Mnemonic {
    /// Checks if the mnemonic is a branch instruction.
    pub fn is_branch(&self) -> bool {
        use Mnemonic::*;
        match self {
            Bcc | Bcs | Beq | Bmi | Bne | Bpl | Bvc | Bvs => true,
            _ => false,
        }
    }
}

/// The address mode parsed.
#[derive(Enum, PartialEq, Clone, Copy)]
pub enum AddressMode {
    Acc,
    Abs,
    AbsX,
    AbsY,
    Imme,
    Impl,
    Ind,
    Xind,
    IndY,
    Zpg,
    ZpgX,
    ZpgY,
    // relative is missing because it gets parsed as zpg
}

impl OpState {
    /// Turn an opstate into an AddressMode because
    /// enum map only allows simple enums. An incomplete state returns None.
    /// Also take the value out of it.
    pub fn destruct(self) -> Option<(AddressMode, OpVal)> {
        use AddressMode::*;
        Some(match self {
            // dummy val
            OpState::Impl => (Impl, OpVal::Byte(0)),
            OpState::Acc => (Acc, OpVal::Byte(0)),

            OpState::Plain(OpVal::Byte(b)) => (Zpg, OpVal::Byte(b)),
            OpState::Plain(OpVal::Word(w)) => (Abs, OpVal::Word(w)),
            OpState::Plain(OpVal::Ref(rf)) if rf.which_byte != ByteSelect::Both => {
                (Zpg, OpVal::Byte(0))
            }
            OpState::Plain(OpVal::Ref(rf)) if rf.which_byte == ByteSelect::Both => {
                (Abs, OpVal::Word(0))
            }

            OpState::AbsX(OpVal::Word(w)) => (AbsX, OpVal::Word(w)),
            OpState::AbsX(OpVal::Ref(rf)) if rf.which_byte == ByteSelect::Both => {
                (AbsX, OpVal::Word(0))
            }

            OpState::AbsY(OpVal::Word(w)) => (AbsY, OpVal::Word(w)),
            OpState::AbsY(OpVal::Ref(rf)) if rf.which_byte == ByteSelect::Both => {
                (AbsY, OpVal::Word(0))
            }

            OpState::Imme(OpVal::Byte(b)) => (Imme, OpVal::Byte(b)),
            OpState::Imme(OpVal::Ref(rf)) if rf.which_byte != ByteSelect::Both => {
                (Imme, OpVal::Word(0))
            }

            OpState::Ind(OpVal::Word(w)) => (Ind, OpVal::Word(w)),
            OpState::Ind(OpVal::Ref(rf)) if rf.which_byte == ByteSelect::Both => {
                (Ind, OpVal::Word(0))
            }

            OpState::Xind(OpVal::Byte(b)) => (Xind, OpVal::Byte(b)),
            OpState::Xind(OpVal::Ref(rf)) if rf.which_byte != ByteSelect::Both => {
                (Xind, OpVal::Word(0))
            }

            OpState::IndY(OpVal::Byte(b)) => (IndY, OpVal::Byte(b)),
            OpState::IndY(OpVal::Ref(rf)) if rf.which_byte != ByteSelect::Both => {
                (IndY, OpVal::Word(0))
            }

            OpState::ZpgX(OpVal::Byte(b)) => (ZpgX, OpVal::Byte(b)),
            OpState::ZpgX(OpVal::Ref(rf)) if rf.which_byte != ByteSelect::Both => {
                (ZpgX, OpVal::Word(0))
            }

            OpState::ZpgY(OpVal::Byte(b)) => (ZpgY, OpVal::Byte(b)),
            OpState::ZpgY(OpVal::Ref(rf)) if rf.which_byte != ByteSelect::Both => {
                (ZpgY, OpVal::Word(0))
            }

            _ => return None,
        })
    }
}
