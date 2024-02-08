use std::fmt::{Display, Formatter};
use crate::helium::instructions::Instruction;

#[derive(Debug, Clone)]
pub enum ProgramElement {
    Instruction(Instruction),
    Identifier(String),
    Immediate(u16)
}

impl Display for ProgramElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramElement::Instruction(ins) => {write!(f, "{}", ins)}
            ProgramElement::Identifier(ident) => {write!(f, "{};", ident)}
            ProgramElement::Immediate(im) => {write!(f, "{}", im)}
        }
    }
}