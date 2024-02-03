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