use super::ir::*;
use super::token::Token;
use logos::{Filter, Lexer};

pub fn vis_object(lex: &mut Lexer<Token>) -> Filter<()> {
    if !lex.extras.start_line {
        lex.extras.err = "visibility modifier must be first in the line";
        Filter::Emit(())
    } else {
        lex.extras.start_line = false;
        lex.extras.vis = Some(Visibility::Object);
        Filter::Skip
    }
}

pub fn vis_global(lex: &mut Lexer<Token>) -> Filter<()> {
    if !lex.extras.start_line {
        lex.extras.err = "visibility modifier must be first in the line";
        Filter::Emit(())
    } else {
        lex.extras.start_line = false;
        lex.extras.vis = Some(Visibility::Global);
        Filter::Skip
    }
}

/// Process a root label, either at the beginning of the line or in the operand.
pub fn label(lex: &mut Lexer<Token>) -> Filter<()> {
    // validate the label name
    let lexed = lex.slice();
    let name = if lexed.len() > 31 {
        lex.extras.err = "identifier must be 31 chars or less";
        return Filter::Emit(());
    } else if lex.extras.sections[match &lex.extras.active {
        Some(active) => active,
        None => {
            lex.extras.err = "no section has been set";
            return Filter::Emit(());
        }
    }]
    .labels
    .len()
        > 255
    {
        lex.extras.err = "too many labels in section {}";
        return Filter::Emit(());
    } else {
        let mut s = [0; 32];

        for (idx, ch) in lexed.as_bytes().iter().enumerate() {
            s[idx] = *ch;
        }
        s
    };

    let sect = lex
        .extras
        .sections
        .get_mut(match &lex.extras.active {
            Some(active) => active,
            None => {
                lex.extras.err = "no section has been set";
                return Filter::Emit(());
            }
        })
        .unwrap();
    if lex.extras.ins.is_none() {
        // label at beginning of line, insert in section
        sect.last_parent = Some(sect.labels.len());
        sect.num_parents += 1;
        sect.labels.push(Label {
            vis: lex.extras.vis.unwrap_or(Visibility::Global),
            name: name,
            num_children: 0,
            offset: sect.size,
        });
        lex.extras.start_line = false;
    } else {
        // TODO add a field indicatig if branch
        let rf = Reference {
            parent: name,
            child: None,
            offset: sect.size + 1, // add 1 so it goes after the opcode
            which_byte: ByteSelect::Both,
        };
        sect.references.push(rf);
        // label is a reference in an operand
        let new_ref = OpVal::Ref(rf);
        lex.extras.op = Some(match &lex.extras.op {
            None => OpState::Plain(new_ref),
            Some(OpState::StartImme) => OpState::Imme(new_ref),
            Some(OpState::StartInd) => OpState::MaybeInd(IndOp::Other(new_ref)),
            _ => {
                lex.extras.err = "invalid placement of reference";
                return Filter::Emit(());
            }
        });
    }

    Filter::Skip
}

/// Process a child label, either at the beginning of the line or in the operand.
pub fn child_label(lex: &mut Lexer<Token>) -> Filter<()> {
    let lexed = &lex.slice()[1..];
    let name = if lexed.len() > 31 {
        lex.extras.err = "identifier must be 31 chars or less";
        return Filter::Emit(());
    } else if lex.extras.sections[match &lex.extras.active {
        Some(active) => active,
        None => {
            lex.extras.err = "no section has been set";
            return Filter::Emit(());
        }
    }]
    .labels
    .len()
        > 255
    {
        lex.extras.err = "too many labels in section {}";
        return Filter::Emit(());
    } else {
        let mut s = [0; 32];

        for (idx, ch) in lexed.as_bytes().iter().enumerate() {
            s[idx] = *ch;
        }
        s
    };

    let sect = lex
        .extras
        .sections
        .get_mut(match &lex.extras.active {
            Some(active) => active,
            None => {
                lex.extras.err = "no section has been set";
                return Filter::Emit(());
            }
        })
        .unwrap();
    if lex.extras.ins.is_none() {
        if !lex.extras.start_line {
            lex.extras.err = "child label must appear first in the line";
            return Filter::Emit(());
        }
        // beginning of line
        // put it under last parent
        match sect.last_parent {
            None => {
                lex.extras.err = "no parent label has been created yet";
                return Filter::Emit(());
            }
            Some(parent) => {
                // update last parent's children, add new label
                sect.labels[parent].num_children += 1;
                sect.labels.push(Label {
                    vis: lex.extras.vis.unwrap_or(Visibility::Hidden),
                    name: name,
                    num_children: 0,
                    offset: sect.size,
                });
            }
        }
    } else {
        // part of operand, equivalent to number
        lex.extras.op = match &lex.extras.op {
            Some(OpState::Plain(OpVal::Ref(mut rf))) => {
                rf.child = Some(name);
                Some(OpState::Plain(OpVal::Ref(rf)))
            }
            Some(OpState::Imme(OpVal::Ref(mut rf))) => {
                rf.child = Some(name);
                Some(OpState::Imme(OpVal::Ref(rf)))
            }
            Some(OpState::MaybeInd(IndOp::Other(OpVal::Ref(mut rf)))) if rf.child.is_none() => {
                rf.child = Some(name);
                Some(OpState::MaybeInd(IndOp::Other(OpVal::Ref(rf))))
            }
            _ => {
                lex.extras.err = "invalid placement of ch8ild label in operand";
                return Filter::Emit(());
            }
        };
        sect.references.last_mut().unwrap().child = Some(name);
    }
    Filter::Skip
}
