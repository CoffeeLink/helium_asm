use std::fmt::{Display, Formatter};

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ConstantType {
    #[default]
    Unknown,
    Value(u16),
    Label,
}

impl Display for ConstantType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstantType::Unknown => {
                write!(f, "Unknown")
            }
            ConstantType::Value(v) => {
                write!(f, "{}", v)
            }
            ConstantType::Label => {
                write!(f, "Label")
            }
        }
    }
}
