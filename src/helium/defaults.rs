use std::collections::HashMap;
use crate::helium::parser::ConstantType;
use crate::helium::parser::ConstantType::Value;

// all const values for later use.

pub const DEFAULT_CONSTS: HashMap<String, ConstantType> = HashMap::from(
    [
        (String::from("A"), Value(0)),
        (String::from("rA"), Value(0)),
        (String::from("r0"), Value(0)),

        (String::from("B"), Value(1)),
        (String::from("rB"), Value(1)),
        (String::from("r1"), Value(1)),

        (String::from("C"), Value(2)),
        (String::from("rC"), Value(2)),
        (String::from("r2"), Value(2)),

        (String::from("X"), Value(3)),
        (String::from("rX"), Value(3)),
        (String::from("r3"), Value(3)),

        (String::from("Y"), Value(4)),
        (String::from("rY"), Value(4)),
        (String::from("r4"), Value(4)),

        (String::from("F"), Value(5)),
        (String::from("rF"), Value(5)),
        (String::from("r5"), Value(5)),

        (String::from("SP"), Value(6)),
        (String::from("rSP"), Value(6)),
        (String::from("r6"), Value(6)),
    ]
);