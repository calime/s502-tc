//! This module conmtains all the callbacks for handling tokens that appear in the operand.

use super::ir::*;
use super::token::Token;
use logos::{Filter, Lexer};

/// Take the high byte of the preceding reference.
///
/// This goes after the number instead of before because each state transition
/// naturally acts on the current state.
pub fn byte_high(lex: &mut Lexer<Token>) -> Filter<()> {
    match lex.extras.op {
        Some(OpState::Plain(OpVal::Ref(mut rf)))
        | Some(OpState::Imme(OpVal::Ref(mut rf)))
        | Some(OpState::MaybeInd(IndOp::Other(OpVal::Ref(mut rf))))
            if rf.which_byte == ByteSelect::Both =>
        {
            lex.extras
                .sections
                .get_mut(match &lex.extras.active {
                    Some(active) => active,
                    None => {
                        lex.extras.err = "no section has been set";
                        return Filter::Emit(());
                    }
                })
                .unwrap()
                .references
                .last_mut()
                .unwrap()
                .which_byte = ByteSelect::High;
            // rf.which_byte = ByteSelect::High
        }
        _ => {
            lex.extras.err = "invalid placement of byte selector";
            return Filter::Emit(());
        }
    };
    Filter::Skip
}

/// Take the low byte of the preceding reference.
///
/// This goes after the number instead of before because each state transition
/// naturally acts on the current state.
pub fn byte_low(lex: &mut Lexer<Token>) -> Filter<()> {
    match lex.extras.op {
        Some(OpState::Plain(OpVal::Ref(mut rf)))
        | Some(OpState::Imme(OpVal::Ref(mut rf)))
        | Some(OpState::MaybeInd(IndOp::Other(OpVal::Ref(mut rf))))
            if rf.which_byte == ByteSelect::Both =>
        {
            lex.extras
                .sections
                .get_mut(match &lex.extras.active {
                    Some(active) => active,
                    None => {
                        lex.extras.err = "no section has been set";
                        return Filter::Emit(());
                    }
                })
                .unwrap()
                .references
                .last_mut()
                .unwrap()
                .which_byte = ByteSelect::Low;
            // rf.which_byte = ByteSelect::Low
        }
        _ => {
            lex.extras.err = "invalid placement of byte selector";
            return Filter::Emit(());
        }
    };
    Filter::Skip
}

/// Accept the accumulator operand. Fails if not the first in the operand.
pub fn acc(lex: &mut Lexer<Token>) -> Filter<()> {
    if lex.extras.op.is_some() {
        lex.extras.err = "Accumulator argument must appear alone";
        Filter::Emit(())
    } else {
        lex.extras.op = Some(OpState::Acc);
        Filter::Skip
    }
}

/// Recognizes the X register.
pub fn xreg(lex: &mut Lexer<Token>) -> Filter<()> {
    lex.extras.op = match &lex.extras.op {
        Some(state) => Some(match state {
            OpState::Idx(val) => match val {
                OpVal::Word(_) => OpState::AbsX(*val),
                OpVal::Byte(_) => OpState::ZpgX(*val),
                OpVal::Ref(rf) if rf.which_byte == ByteSelect::Both => OpState::AbsX(*val),
                OpVal::Ref(rf) if rf.which_byte != ByteSelect::Both => OpState::ZpgX(*val),
                _ => {
                    lex.extras.err = "invalid placement of Y";
                    return Filter::Emit(());
                }
            },
            OpState::MaybeInd(IndOp::XindComma(val)) => OpState::MaybeInd(IndOp::Xind(*val)),
            _ => {
                lex.extras.err = "invalid placement of X";
                return Filter::Emit(());
            }
        }),
        _ => {
            lex.extras.err = "invalid placement of X";
            return Filter::Emit(());
        }
    };

    Filter::Skip
}

/// Recognizes the X register.
pub fn yreg(lex: &mut Lexer<Token>) -> Filter<()> {
    lex.extras.op = match &lex.extras.op {
        Some(state) => Some(match state {
            OpState::Idx(val) => match val {
                OpVal::Word(_) => OpState::AbsY(*val),
                OpVal::Byte(_) => OpState::ZpgY(*val),
                OpVal::Ref(rf) if rf.which_byte == ByteSelect::Both => OpState::AbsY(*val),
                OpVal::Ref(rf) if rf.which_byte != ByteSelect::Both => OpState::ZpgY(*val),
                _ => {
                    lex.extras.err = "invalid placement of Y";
                    return Filter::Emit(());
                }
            },
            OpState::MaybeInd(IndOp::IndYComma(val)) => match val {
                OpVal::Byte(_) => OpState::IndY(*val),

                OpVal::Ref(rf) if rf.which_byte != ByteSelect::Both => OpState::IndY(*val),
                _ => {
                    lex.extras.err = "invalid placement of Y";
                    return Filter::Emit(());
                }
            },
            _ => {
                lex.extras.err = "invalid placement of Y";
                return Filter::Emit(());
            }
        }),
        _ => {
            lex.extras.err = "invalid placement of Y";
            return Filter::Emit(());
        }
    };

    Filter::Skip
}

/// Recognizes the pound for immediate operands.
pub fn imme(lex: &mut Lexer<Token>) -> Filter<()> {
    if lex.extras.op.is_some() {
        lex.extras.err = "immediate pound must be first part of operand";
        Filter::Emit(())
    } else {
        lex.extras.op = Some(OpState::StartImme);
        Filter::Skip
    }
}

/// Recognizes numbers
pub fn number(lex: &mut Lexer<Token>) -> Filter<()> {
    let base = match lex.slice().as_bytes()[0] {
        b'%' => 2,
        b'@' => 8,
        b'$' => 16,
        _ => 10,
    };
    let s = if base == 10 {
        &lex.slice()
    } else {
        &lex.slice()[1..]
    };

    if let Ok(num) = u16::from_str_radix(s, base) {
        lex.extras.op = match (
            &lex.extras.op,
            // promote numbers <= 255 to word if padded with 0's
            match (base, s.len()) {
                (_, _) if num > 255 => OpVal::Word(num),
                (10, l) if l > 3 => OpVal::Word(num),
                (2, l) if l > 8 => OpVal::Word(num),
                (8, l) if l > 3 => OpVal::Word(num),
                (16, l) if l > 2 => OpVal::Word(num),
                (_, _) => OpVal::Byte(num as u8),
            },
        ) {
            // change state based on on context
            (None, state) => Some(OpState::Plain(state)),
            (Some(state), val) => Some(match (state, val) {
                (OpState::StartImme, OpVal::Byte(b)) => OpState::Imme(OpVal::Byte(b)),
                (OpState::StartInd, _) => OpState::MaybeInd(IndOp::Other(val)),
                _ => {
                    lex.extras.err = "invalid placement of number";
                    return Filter::Emit(());
                }
            }),
        };
        Filter::Skip
    } else {
        lex.extras.err = "invalid number";
        Filter::Emit(())
    }
}

/// Recognizes opaning parenthesis.
pub fn lparen(lex: &mut Lexer<Token>) -> Filter<()> {
    if lex.extras.op.is_some() {
        lex.extras.err = "left parenthesis must be first part of operand";
        Filter::Emit(())
    } else {
        lex.extras.op = Some(OpState::StartInd);
        Filter::Skip
    }
}

/// Recognizes closing parenthesis.
pub fn rparen(lex: &mut Lexer<Token>) -> Filter<()> {
    lex.extras.op = match &lex.extras.op {
        Some(OpState::MaybeInd(indop)) => Some(match indop {
            IndOp::Other(val) => match val {
                OpVal::Word(_) => OpState::Ind(*val),
                OpVal::Byte(_) => OpState::MaybeInd(IndOp::IndY(*val)),
                OpVal::Ref(rf) if rf.which_byte == ByteSelect::Both => OpState::Ind(*val),
                OpVal::Ref(rf) if rf.which_byte != ByteSelect::Both => {
                    OpState::MaybeInd(IndOp::IndY(*val))
                }
                _ => {
                    lex.extras.err = "invalid placement of right parenthesis";
                    return Filter::Emit(());
                }
            },
            IndOp::Xind(val) => OpState::Xind(*val),
            _ => {
                lex.extras.err = "invalid placement of right parenthesis";
                return Filter::Emit(());
            }
        }),
        _ => {
            lex.extras.err = "invalid placement of right parenthesis";
            return Filter::Emit(());
        }
    };
    Filter::Skip
}

/// Recognizes a comma
pub fn comma(lex: &mut Lexer<Token>) -> Filter<()> {
    lex.extras.op = match &lex.extras.op {
        Some(state) => Some(match state {
            // lead to absx absy zpgx zpgy
            OpState::Plain(val) => OpState::Idx(*val),
            OpState::MaybeInd(IndOp::Other(val)) => match val {
                OpVal::Byte(_) => OpState::MaybeInd(IndOp::XindComma(*val)),
                OpVal::Ref(rf) if rf.which_byte != ByteSelect::Both => {
                    OpState::MaybeInd(IndOp::XindComma(*val))
                }
                _ => {
                    lex.extras.err = "invalid placement of comma";
                    return Filter::Emit(());
                }
            },
            OpState::MaybeInd(IndOp::IndY(val)) => OpState::MaybeInd(IndOp::IndYComma(*val)),
            _ => {
                lex.extras.err = "invalid placement of comma";
                return Filter::Emit(());
            }
        }),
        _ => {
            lex.extras.err = "invalid placement of comma";
            return Filter::Emit(());
        }
    };

    Filter::Skip
}
