use crate::helium::instructions::Instruction;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum ProgramElement {
    Instruction(Instruction),
    Identifier(String),
    Immediate(u16),
}

impl ProgramElement {
    pub fn is_instruction(&self) -> bool {
        matches!(self, ProgramElement::Instruction(_))
    }
}

impl Display for ProgramElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProgramElement::Instruction(ins) => {
                write!(f, "{}", ins)
            }
            ProgramElement::Identifier(ident) => {
                write!(f, "{};", ident)
            }
            ProgramElement::Immediate(im) => {
                write!(f, "{}", im)
            }
        }
    }
}