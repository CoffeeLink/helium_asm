use crate::helium::instructions::AsmInstruction::{Add, AddWithCarry, And, BitCheck, Call, CallCarry, CallEquals, CallGreaterThan, CallGreaterThanEq, CallInterrupt, CallLessThan, CallLessThanEq, CallNotEquals, CallNotGreaterThan, CallNotLessThan, CallOverflow, CallZero, Compare, CompareSigned, Decrement, Halt, Increment, Jump, JumpCarry, JumpEquals, JumpGreaterThan, JumpGreaterThanEq, JumpLessThan, JumpLessThanEq, JumpNotEquals, JumpNotGreaterThan, JumpNotLessThan, JumpOverflow, JumpZero, Load, LoadProgramMemory, Move, Negative, NoOperation, Not, Or, Pop, Push, Reset, Return, ReturnCarry, ReturnEquals, ReturnGreaterThan, ReturnGreaterThanEq, ReturnLessThan, ReturnLessThanEq, ReturnNotEquals, ReturnNotGreaterThan, ReturnNotLessThan, ReturnOverflow, ReturnZero, SetBit, SetInterruptAddress, ShiftLeft, ShiftRight, Store, StoreProgramMemory, Sub, SubWithCarry, WaitUntilInterrupt, Xor};

#[derive(Debug)]
pub enum AsmInstruction {
    Halt,
    NoOperation,
    Reset,

    CallInterrupt,
    ReturnInterrupt,
    WaitUntilInterrupt,
    SetInterruptAddress,

    Move,

    Store, StoreProgramMemory,

    Load, LoadProgramMemory,

    Push, Pop,

    Add, AddWithCarry,
    Sub, SubWithCarry,

    Negative,

    Increment, Decrement,

    And, Or, Xor, Not,

    ShiftLeft, ShiftRight,

    SetBit, BitCheck,

    Compare, CompareSigned,

    Jump, Call, Return,

    JumpEquals,         CallEquals,         ReturnEquals,
    JumpNotEquals,      CallNotEquals,      ReturnNotEquals,
    JumpLessThan,       CallLessThan,       ReturnLessThan,
    JumpLessThanEq,     CallLessThanEq,     ReturnLessThanEq,
    JumpNotLessThan,    CallNotLessThan,    ReturnNotLessThan,
    JumpGreaterThan,    CallGreaterThan,    ReturnGreaterThan,
    JumpGreaterThanEq,  CallGreaterThanEq,  ReturnGreaterThanEq,
    JumpNotGreaterThan, CallNotGreaterThan, ReturnNotGreaterThan,

    JumpZero, CallZero, ReturnZero,
    JumpOverflow, CallOverflow, ReturnOverflow,
    JumpCarry, CallCarry, ReturnCarry
}

impl AsmInstruction {
    pub fn match_instruction(word : &str) -> Option<Self> {
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
            _ => None
        }
    }
}