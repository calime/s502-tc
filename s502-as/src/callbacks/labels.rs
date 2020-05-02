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
    } else if lex.extras.sections[&lex.extras.active].labels.len() > 255 {
        lex.extras.err = "too many labels in section {}";
        return Filter::Emit(());
    } else {
        let mut s = [0; 32];

        for (idx, ch) in lexed.as_bytes().iter().enumerate() {
            s[idx] = *ch;
        }
        s
    };

    if lex.extras.ins.is_none() {
        // label at beginning of line, insert in section
        let sect = lex.extras.sections.get_mut(&lex.extras.active).unwrap();
        sect.last_parent = Some(sect.labels.len());
        sect.labels.push(Label {
            vis: lex.extras.vis.unwrap_or(Visibility::Global),
            name: name,
            num_children: 0,
            offset: sect.size,
        });
        lex.extras.start_line = false;
    } else {
        // label is a reference in an operand
        let new_ref = OpVal::Ref(Reference {
            parent: name,
            child: None,
            offset: lex.extras.sections[&lex.extras.active].size,
            which_byte: ByteSelect::Both,
        });
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
    } else if lex.extras.sections[&lex.extras.active].labels.len() > 255 {
        lex.extras.err = "too many labels in section {}";
        return Filter::Emit(());
    } else {
        let mut s = [0; 32];

        for (idx, ch) in lexed.as_bytes().iter().enumerate() {
            s[idx] = *ch;
        }
        s
    };

    if lex.extras.ins.is_none() {
        // beginning of line
        let sect = lex.extras.sections.get_mut(&lex.extras.active).unwrap();
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
            Some(OpState::Plain(OpVal::Ref(mut reference))) => {
                // parent was specified explicitly
                reference.child = Some(name);
                Some(OpState::Plain(OpVal::Ref(reference)))
            }
            _ => {
                // imply reference to current parent
                // NOTE maybe even remove this, force explicit parent
                let sect = lex.extras.sections.get_mut(&lex.extras.active).unwrap();
                if let Some(last_parent) = sect.last_parent {
                    let new_ref = OpVal::Ref(Reference {
                        parent: sect.labels[last_parent].name,
                        child: Some(name),
                        offset: sect.size,
                        which_byte: ByteSelect::Both,
                    });
                    Some(match &lex.extras.op {
                        None => OpState::Plain(new_ref),
                        Some(OpState::StartImme) => OpState::Imme(new_ref),
                        Some(OpState::StartInd) => OpState::MaybeInd(IndOp::Other(new_ref)),
                        _ => {
                            lex.extras.err = "invalid placement of reference";
                            return Filter::Emit(());
                        }
                    })
                } else {
                    lex.extras.err = "no parent label has been created yet";
                    return Filter::Emit(());
                }
            }
        }
    }
    Filter::Skip
}
