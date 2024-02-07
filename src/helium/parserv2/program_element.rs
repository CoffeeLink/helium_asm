use crate::helium::instructions::Instruction;

#[derive(Debug, Clone)]
pub enum ProgramElement {
    Instruction(Instruction),
    Identifier(String),
    Immediate(u16)
}