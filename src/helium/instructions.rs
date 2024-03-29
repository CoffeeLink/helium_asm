use crate::helium::instructions::AsmInstruction::{
    Add, AddWithCarry, And, BitCheck, Call, CallCarry, CallEquals, CallGreaterThan,
    CallGreaterThanEq, CallInterrupt, CallLessThan, CallLessThanEq, CallNotEquals,
    CallNotGreaterThan, CallNotLessThan, CallOverflow, CallZero, Compare, CompareSigned, Decrement,
    Halt, Increment, Jump, JumpCarry, JumpEquals, JumpGreaterThan, JumpGreaterThanEq, JumpLessThan,
    JumpLessThanEq, JumpNotEquals, JumpNotGreaterThan, JumpNotLessThan, JumpOverflow, JumpZero,
    Load, LoadProgramMemory, Move, Negative, NoOperation, Not, Or, Pop, Push, Reset, Return,
    ReturnCarry, ReturnEquals, ReturnGreaterThan, ReturnGreaterThanEq, ReturnLessThan,
    ReturnLessThanEq, ReturnNotEquals, ReturnNotGreaterThan, ReturnNotLessThan, ReturnOverflow,
    ReturnZero, SetBit, SetInterruptAddress, ShiftLeft, ShiftRight, Store, StoreProgramMemory, Sub,
    SubWithCarry, WaitUntilInterrupt, Xor,
};
use std::fmt::{Display, Formatter};
use crate::helium::tokens::Token;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum AsmInstruction {
    Halt,
    NoOperation,
    Reset,

    CallInterrupt,
    ReturnInterrupt,
    WaitUntilInterrupt,
    SetInterruptAddress,

    Move,

    Store,
    StoreProgramMemory,

    Load,
    LoadProgramMemory,

    Push,
    Pop,

    Add,
    AddWithCarry,
    Sub,
    SubWithCarry,

    Negative,

    Increment,
    Decrement,

    And,
    Or,
    Xor,
    Not,

    ShiftLeft,
    ShiftRight,

    SetBit,
    BitCheck,

    Compare,
    CompareSigned,

    Jump,
    Call,
    Return,

    JumpEquals,
    CallEquals,
    ReturnEquals,
    JumpNotEquals,
    CallNotEquals,
    ReturnNotEquals,
    JumpLessThan,
    CallLessThan,
    ReturnLessThan,
    JumpLessThanEq,
    CallLessThanEq,
    ReturnLessThanEq,
    JumpNotLessThan,
    CallNotLessThan,
    ReturnNotLessThan,
    JumpGreaterThan,
    CallGreaterThan,
    ReturnGreaterThan,
    JumpGreaterThanEq,
    CallGreaterThanEq,
    ReturnGreaterThanEq,
    JumpNotGreaterThan,
    CallNotGreaterThan,
    ReturnNotGreaterThan,

    JumpZero,
    CallZero,
    ReturnZero,
    JumpOverflow,
    CallOverflow,
    ReturnOverflow,
    JumpCarry,
    CallCarry,
    ReturnCarry,
}

impl AsmInstruction {
    pub fn match_instruction(word: &str) -> Option<Self> {
        match word {
            "mov" | "MOV" => Some(Move),

            "hlt" | "HLT" => Some(Halt),
            "nop" | "NOP" => Some(NoOperation),
            "rst" | "RST" => Some(Reset),

            "int" | "INT" => Some(CallInterrupt),
            "wai" | "WAI" => Some(WaitUntilInterrupt),
            "sia" | "SIA" => Some(SetInterruptAddress),

            "st" | "ST" => Some(Store),
            "stp" | "STP" => Some(StoreProgramMemory),
            "ld" | "LD" => Some(Load),
            "ldp" | "LDP" => Some(LoadProgramMemory),

            "psh" | "PSH" => Some(Push),
            "pop" | "POP" => Some(Pop),

            "add" | "ADD" => Some(Add),
            "adc" | "ADC" => Some(AddWithCarry),
            "sub" | "SUB" => Some(Sub),
            "sbc" | "SBC" => Some(SubWithCarry),

            "neg" | "NEG" => Some(Negative),

            "inc" | "INC" => Some(Increment),
            "dec" | "DEC" => Some(Decrement),

            "and" | "AND" => Some(And),
            "or" | "OR" => Some(Or),
            "xor" | "XOR" => Some(Xor),
            "not" | "NOT" => Some(Not),

            "shl" | "SHL" => Some(ShiftLeft),
            "shr" | "SHR" => Some(ShiftRight),

            "set" | "SET" => Some(SetBit),
            "bit" | "BIT" => Some(BitCheck),

            "cmp" | "CMP" => Some(Compare),
            "cms" | "CMS" => Some(CompareSigned),
            "jmp" | "JMP" => Some(Jump),

            "cal" | "CAL" => Some(Call),
            "ret" | "RET" => Some(Return),

            "jeq" | "JEQ" => Some(JumpEquals),
            "jne" | "JNE" => Some(JumpNotEquals),
            "jlt" | "JLT" => Some(JumpLessThan),
            "jle" | "JLE" => Some(JumpLessThanEq),
            "jnl" | "JNL" => Some(JumpNotLessThan),
            "jgt" | "JGT" => Some(JumpGreaterThan),
            "jge" | "JGE" => Some(JumpGreaterThanEq),
            "jng" | "JNG" => Some(JumpNotGreaterThan),
            "jez" | "JEZ" => Some(JumpZero),
            "jof" | "JOF" => Some(JumpOverflow),
            "jcr" | "JCR" => Some(JumpCarry),

            "ceq" | "CEQ" => Some(CallEquals),
            "cne" | "CNE" => Some(CallNotEquals),
            "clt" | "CLT" => Some(CallLessThan),
            "cle" | "CLE" => Some(CallLessThanEq),
            "cnl" | "CNL" => Some(CallNotLessThan),
            "cgt" | "CGT" => Some(CallGreaterThan),
            "cge" | "CGE" => Some(CallGreaterThanEq),
            "cng" | "CNG" => Some(CallNotGreaterThan),
            "cez" | "CEZ" => Some(CallZero),
            "cof" | "COF" => Some(CallOverflow),
            "ccr" | "CCR" => Some(CallCarry),

            "req" | "REQ" => Some(ReturnEquals),
            "rne" | "RNE" => Some(ReturnNotEquals),
            "rlt" | "RLT" => Some(ReturnLessThan),
            "rle" | "RLE" => Some(ReturnLessThanEq),
            "rnl" | "RNL" => Some(ReturnNotLessThan),
            "rgt" | "RGT" => Some(ReturnGreaterThan),
            "rge" | "RGE" => Some(ReturnGreaterThanEq),
            "rng" | "RNG" => Some(ReturnNotGreaterThan),
            "rez" | "REZ" => Some(ReturnZero),
            "rof" | "ROF" => Some(ReturnOverflow),
            "rcr" | "RCR" => Some(ReturnCarry),
            _ => None,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Instruction {
    pub kind: AsmInstruction,
    pub args: Vec<Argument>,

    pub tokens_used: Vec<Token>
}
impl Instruction {
    pub fn new(kind: AsmInstruction) -> Self {
        Self { kind, args: vec![], tokens_used: vec![]}
    }
}

#[derive(Debug, Clone)]
pub enum Argument {
    Register(u16),
    RegisterIdentifier(String),

    Immediate(u16),
    ImmediateIdentifier(String),
}

impl Argument {
    pub fn kind(&self) -> String {
        if self.is_register() {
            "Register".to_string()
        } else {
            "Integer".to_string()
        }
    }
    /// Returns true if the argument is a register.
    ///
    /// # Examples:
    /// register kinds:
    /// ```
    /// use helium_asm::helium::instructions::Argument;
    ///
    /// let a = Argument::Register(10);
    /// assert_eq!(a.is_register(), true);
    ///
    /// let b = Argument::RegisterIdentifier("ArgName".to_string());
    /// assert_eq!(b.is_register(), true);
    ///
    /// assert_eq!(a.is_integer(), false);
    /// assert_eq!(a.is_integer(), false);
    ///
    /// ```
    pub fn is_register(&self) -> bool {
        matches!(self, Argument::Register(_)) || matches!(self, Argument::RegisterIdentifier(_))
    }

    /// Returns true if the argument is an integer.
    ///
    /// # Examples:
    /// integer kinds:
    /// ```
    /// use helium_asm::helium::instructions::Argument;
    ///
    /// let a = Argument::Immediate(10);
    /// assert_eq!(a.is_integer(), true);
    ///
    /// let b = Argument::ImmediateIdentifier("ArgName".to_string());
    /// assert_eq!(b.is_integer(), true);
    ///
    /// assert_eq!(a.is_register(), false);
    /// assert_eq!(b.is_register(), false);
    ///
    /// ```
    pub fn is_integer(&self) -> bool {
        matches!(self, Argument::Immediate(_)) || matches!(self, Argument::ImmediateIdentifier(_))
    }

    pub fn calculate_len(&self) -> usize {
        match self {
            Argument::Register(i) => (*i as f64).log10().floor() as usize + 1,
            Argument::RegisterIdentifier(s) => s.len(),
            Argument::Immediate(i) => (*i as f64).log10().floor() as usize + 1,
            Argument::ImmediateIdentifier(s) => s.len()
        }
    }
}

impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Argument::Register(i) => write!(f, "${}", i),
            Argument::RegisterIdentifier(i) => write!(f, "${}", i),
            Argument::Immediate(i) => write!(f, "{}", i),
            Argument::ImmediateIdentifier(i) => write!(f, "{}", i),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut args = self.args.iter().peekable();
        let mut out = format!("{:?}", self.kind);
        while let Some(arg) = args.next() {
            out.extend(format!(" {}", arg).chars());
            if args.peek().is_some() {
                out.push(',');
            }
        }

        write!(f, "{};", out)
    }
}
