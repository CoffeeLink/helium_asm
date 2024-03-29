use crate::helium::parsing::constant_type::ConstantType;
use crate::helium::parsing::constant_type::ConstantType::Value;
use lazy_static::lazy_static;
use std::collections::BTreeMap;
lazy_static! {
    pub static ref DEFAULT_CONSTANTS: BTreeMap<String, ConstantType> = {
        let mut m: BTreeMap<String, ConstantType> = BTreeMap::new();

        m.insert("A".into(), Value(0));
        m.insert("rA".into(), Value(0));
        m.insert("r0".into(), Value(0));

        m.insert("B".into(), Value(1));
        m.insert("rB".into(), Value(1));
        m.insert("r1".into(), Value(1));

        m.insert("C".into(), Value(2));
        m.insert("rC".into(), Value(2));
        m.insert("r2".into(), Value(2));

        m.insert("X".into(), Value(3));
        m.insert("rX".into(), Value(3));
        m.insert("r3".into(), Value(3));

        m.insert("Y".into(), Value(4));
        m.insert("rY".into(), Value(4));
        m.insert("r4".into(), Value(4));

        m.insert("F".into(), Value(5));
        m.insert("rF".into(), Value(5));
        m.insert("r5".into(), Value(5));

        m.insert("SP".into(), Value(6));
        m.insert("rSP".into(), Value(6));
        m.insert("r6".into(), Value(6));

        m
    };
}
