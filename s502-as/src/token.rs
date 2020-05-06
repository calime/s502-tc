use logos::Logos;

use super::callbacks::*;
use super::ir::Program;

#[derive(Logos)]
#[logos(extras = Program)]
pub enum Token {
    #[token("adc", adc)]
    Adc,
    #[token("and", and)]
    And,
    #[token("asl", asl)]
    Asl,
    #[token("bcc", bcc)]
    Bcc,
    #[token("bcs", bcs)]
    Bcs,
    #[token("beq", beq)]
    Beq,
    #[token("bit", bit)]
    Bit,
    #[token("bmi", bmi)]
    Bmi,
    #[token("bne", bne)]
    Bne,
    #[token("bpl", bpl)]
    Bpl,
    #[token("brk", brk)]
    Brk,
    #[token("bvc", bvc)]
    Bvc,
    #[token("bvs", bvs)]
    Bvs,
    #[token("clc", clc)]
    Clc,
    #[token("cld", cld)]
    Cld,
    #[token("cli", cli)]
    Cli,
    #[token("clv", clv)]
    Clv,
    #[token("cmp", cmp)]
    Cmp,
    #[token("cpx", cpx)]
    Cpx,
    #[token("cpy", cpy)]
    Cpy,
    #[token("dec", dec)]
    Dec,
    #[token("dex", dex)]
    Dex,
    #[token("dey", dey)]
    Dey,
    #[token("eor", eor)]
    Eor,
    #[token("inc", inc)]
    Inc,
    #[token("inx", inx)]
    Inx,
    #[token("iny", iny)]
    Iny,
    #[token("jmp", jmp)]
    Jmp,
    #[token("jsr", jsr)]
    Jsr,
    #[token("lda", lda)]
    Lda,
    #[token("ldx", ldx)]
    Ldx,
    #[token("ldy", ldy)]
    Ldy,
    #[token("lsr", lsr)]
    Lsr,
    #[token("nop", nop)]
    Nop,
    #[token("ora", ora)]
    Ora,
    #[token("pha", pha)]
    Pha,
    #[token("php", php)]
    Php,
    #[token("pla", pla)]
    Pla,
    #[token("plp", plp)]
    Plp,
    #[token("rol", rol)]
    Rol,
    #[token("ror", ror)]
    Ror,
    #[token("rti", rti)]
    Rti,
    #[token("rts", rts)]
    Rts,
    #[token("sbc", sbc)]
    Sbc,
    #[token("sec", sec)]
    Sec,
    #[token("sed", sed)]
    Sed,
    #[token("sei", sei)]
    Sei,
    #[token("sta", sta)]
    Sta,
    #[token("stx", stx)]
    Stx,
    #[token("sty", sty)]
    Sty,
    #[token("tax", tax)]
    Tax,
    #[token("tay", tay)]
    Tay,
    #[token("tsx", tsx)]
    Tsx,
    #[token("txa", txa)]
    Txa,
    #[token("txs", txs)]
    Txs,
    #[token("tya", tya)]
    Tya,
    #[token("dfb", dfb)]
    Dfb,
    #[token("dfw", dfw)]
    Dfw,
    #[token("hlt", hlt)]
    Hlt,
    #[token("sct", sct)]
    Sct,
    #[token("a", acc)]
    A,
    #[token("x", xreg)]
    X,
    #[token("y", yreg)]
    Y,
    #[token(",", comma)]
    Comma,
    #[token("#", imme)]
    Imme,
    #[token("(", lparen)]
    Lparen,
    #[token(")", rparen)]
    Rparen,
    #[token("<", byte_high)]
    ByteHi,
    #[token(">", byte_low)]
    ByteLo,
    #[token("!", vis_object)]
    VisObj,
    #[token("!!", vis_global)]
    VisGlobal,
    #[regex("\\$[0-9a-fA-F]+", number)]
    #[regex("%[0-1]+", number)]
    #[regex("@[0-7]+", number)]
    #[regex("[0-9]+", number)]
    Num,
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", label)]
    #[regex("\\.[a-zA-Z0-9_]+", child_label)]
    Ident,
    #[regex("\n", eol)]
    Eol,
    #[error]
    #[regex(";.*\n", eol)]
    #[regex(r"[ \t]+", logos::skip)]
    Error,
}
