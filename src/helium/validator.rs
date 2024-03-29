use std::fmt::{Display, Formatter};
use std::iter::zip;
use crate::helium::parsing::ProgramElement;
use crate::helium::errors::HeliumError;
use crate::helium::instructions::{Argument, AsmInstruction, Instruction};
use crate::helium::parsing::ProgramTree;
use crate::helium::position::Position;
use crate::helium::validator::ArgType::{Any, Register};

#[allow(dead_code)]
enum ArgType {
    Register,
    Integer,
    Any
}

impl PartialEq<Argument> for ArgType {
    fn eq(&self, other: &Argument) -> bool {
        match self {
            Register => { other.is_register() }
            ArgType::Integer => { other.is_integer() }
            Any => { true }
        }
    }
}

impl Display for ArgType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Register => write!(f, "Register"),
            ArgType::Integer => write!(f, "Int"),
            Any => write!(f, "Reg/Int")
        }
    }
}

fn validate(instruction: &Instruction, expectation: Vec<ArgType>) -> Vec<HeliumError> {
    // check arg count, check arg types, return.
    if instruction.args.len() != expectation.len() {
        return if instruction.args.len() > expectation.len() {
            let last = instruction.tokens_used.last().unwrap().position.unwrap();

            let first_chr = instruction.tokens_used[expectation.len() + 1].position.unwrap().chr_start;
            let last_chr = last.chr_start;
            let len_between = (last_chr - first_chr) + last.length;

            let pos = Position::new(
                instruction.tokens_used.first().unwrap().position.unwrap().line,
                first_chr,
                len_between
            );

            vec![HeliumError::new(
                format!("Too many arguments, expected: {}, got: {}.", expectation.len(), instruction.args.len()),
                pos
            )]
        } else {
            let first_pos = instruction.tokens_used.first().unwrap().position.unwrap();

            vec![HeliumError::new(
                format!("Not enough arguments, expected: {}, got: {}.", expectation.len(), instruction.args.len()),
                first_pos
            )]
        }
    }

    let mut errors : Vec<HeliumError> = vec![];
    for (index, (arg, exp)) in zip(instruction.clone().args, expectation).enumerate() {
        if exp != arg {

            let pos = Position::new(
                instruction.tokens_used[index + 1].position.unwrap().line,
                instruction.tokens_used[index + 1].position.unwrap().chr_start,
                instruction.tokens_used[index + 1].position.unwrap().length
            );

            errors.push(HeliumError::new(
                format!("Invalid argument type, expected: {}, got: {}.", exp, arg.kind()),
                pos
            ))
        }
    }

    errors
}

fn validate_instruction(instruction: &Instruction) -> Vec<HeliumError> {
    match instruction.kind {
        AsmInstruction::Halt |
        AsmInstruction::NoOperation |
        AsmInstruction::Reset |
        AsmInstruction::ReturnInterrupt |
        AsmInstruction::Return |
        AsmInstruction::ReturnCarry |
        AsmInstruction::ReturnOverflow |
        AsmInstruction::ReturnEquals |
        AsmInstruction::ReturnGreaterThan |
        AsmInstruction::ReturnGreaterThanEq |
        AsmInstruction::ReturnLessThan |
        AsmInstruction::ReturnLessThanEq |
        AsmInstruction::ReturnNotEquals |
        AsmInstruction::ReturnNotGreaterThan |
        AsmInstruction::ReturnNotLessThan |
        AsmInstruction::ReturnZero |
        AsmInstruction::CallInterrupt |
        AsmInstruction::WaitUntilInterrupt => validate(instruction, vec![]),

        AsmInstruction::SetInterruptAddress => validate(instruction, vec![Any]),

        AsmInstruction::Move => validate(instruction, vec![Register, Any]),

        AsmInstruction::Store |
        AsmInstruction::StoreProgramMemory |
        AsmInstruction::Load |
        AsmInstruction::LoadProgramMemory => validate(instruction, vec![Register, Any]),

        AsmInstruction::Push |
        AsmInstruction::Pop => validate(instruction, vec![Register]),

        AsmInstruction::Add |
        AsmInstruction::AddWithCarry |
        AsmInstruction::Sub |
        AsmInstruction::SubWithCarry => validate(instruction, vec![Register, Register, Register]),

        AsmInstruction::Negative |
        AsmInstruction::Increment |
        AsmInstruction::Decrement => validate(instruction, vec![Register]),

        AsmInstruction::And |
        AsmInstruction::Or |
        AsmInstruction::Xor => validate(instruction, vec![Register, Register, Register]),

        AsmInstruction::Not => validate(instruction, vec![Register, Register]),

        AsmInstruction::ShiftLeft |
        AsmInstruction::ShiftRight => validate(instruction, vec![Register, Register, Register]),

        AsmInstruction::SetBit => validate(instruction, vec![Register, Any, Any]),
        AsmInstruction::BitCheck => validate(instruction, vec![Register, Any, Register]),

        AsmInstruction::Compare |
        AsmInstruction::CompareSigned => validate(instruction, vec![Register, Register]),

        AsmInstruction::Jump |
        AsmInstruction::Call |
        AsmInstruction::JumpEquals |
        AsmInstruction::CallEquals |
        AsmInstruction::JumpNotEquals |
        AsmInstruction::CallNotEquals |
        AsmInstruction::JumpLessThan |
        AsmInstruction::CallLessThan |
        AsmInstruction::JumpLessThanEq |
        AsmInstruction::CallLessThanEq |
        AsmInstruction::JumpNotLessThan |
        AsmInstruction::CallNotLessThan |
        AsmInstruction::JumpGreaterThan |
        AsmInstruction::CallGreaterThan |
        AsmInstruction::JumpGreaterThanEq |
        AsmInstruction::CallGreaterThanEq |
        AsmInstruction::JumpNotGreaterThan |
        AsmInstruction::CallNotGreaterThan |
        AsmInstruction::JumpZero |
        AsmInstruction::CallZero |
        AsmInstruction::JumpOverflow |
        AsmInstruction::CallOverflow |
        AsmInstruction::JumpCarry |
        AsmInstruction::CallCarry => validate(instruction, vec![Any]),
    }
}

pub fn validate_tree(tree : &ProgramTree) -> Vec<HeliumError> {
    let mut errors : Vec<HeliumError> = vec![];
    for segment in tree.clone().segments {
        for element in segment.elements {
            if !element.is_instruction() { continue }
            let ins = match element { ProgramElement::Instruction(i)=>i, _=> unreachable!() };
            errors.extend(validate_instruction(&ins));
        }
    }
    errors
}