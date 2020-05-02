use super::ir::{Mnemonic::*, *};
use super::token::Token;
use logos::{Filter, Lexer};

/// Attempt to insert an instruction in the machine.
/// Fails if already occupied or operands appear first.
///
/// Don't imline to avoidf bloating the cache.
#[inline(never)]
fn insert_mnem(lex: &mut Lexer<Token>, ins: Mnemonic) -> Filter<()> {
    if lex.extras.ins.is_some() {
        lex.extras.err = "multiple mnemonics on one line";
        Filter::Emit(())
    } else if !lex.extras.op.is_none() {
        lex.extras.err = "Mnemonic must appear before operand";
        Filter::Emit(())
    } else {
        lex.extras.ins = Some(ins);
        lex.extras.start_line = false;
        Filter::Skip
    }
}

/// Generate a function for instruction-accepting states.
macro_rules! mnem {
    ($name:ident, $mnem:ident) => {
        pub fn $name(lex: &mut Lexer<Token>) -> Filter<()> {
            insert_mnem(lex, $mnem)
        }
    };
}

mnem!(adc, Adc);
mnem!(and, And);
mnem!(asl, Asl);
mnem!(bcc, Bcc);
mnem!(bcs, Bcs);
mnem!(beq, Beq);
mnem!(bit, Bit);
mnem!(bmi, Bmi);
mnem!(bne, Bne);
mnem!(bpl, Bpl);
mnem!(brk, Brk);
mnem!(bvc, Bvc);
mnem!(bvs, Bvs);
mnem!(clc, Clc);
mnem!(cld, Cld);
mnem!(cli, Cli);
mnem!(clv, Clv);
mnem!(cmp, Cmp);
mnem!(cpx, Cpx);
mnem!(cpy, Cpy);
mnem!(dec, Dec);
mnem!(dex, Dex);
mnem!(dey, Dey);
mnem!(eor, Eor);
mnem!(inc, Inc);
mnem!(inx, Inx);
mnem!(iny, Iny);
mnem!(jmp, Jmp);
mnem!(jsr, Jsr);
mnem!(lda, Lda);
mnem!(ldx, Ldx);
mnem!(ldy, Ldy);
mnem!(lsr, Lsr);
mnem!(nop, Nop);
mnem!(ora, Ora);
mnem!(pha, Pha);
mnem!(php, Php);
mnem!(pla, Pla);
mnem!(plp, Plp);
mnem!(rol, Rol);
mnem!(ror, Ror);
mnem!(rti, Rti);
mnem!(rts, Rts);
mnem!(sbc, Sbc);
mnem!(sec, Sec);
mnem!(sed, Sed);
mnem!(sei, Sei);
mnem!(sta, Sta);
mnem!(stx, Stx);
mnem!(sty, Sty);
mnem!(tax, Tax);
mnem!(tay, Tay);
mnem!(tsx, Tsx);
mnem!(txa, Txa);
mnem!(txs, Txs);
mnem!(tya, Tya);
mnem!(dfb, Dfb);
mnem!(dfw, Dfw);
mnem!(hlt, Hlt);
mnem!(sct, Sct);
