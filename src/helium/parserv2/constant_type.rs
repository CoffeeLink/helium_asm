

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ConstantType {
    #[default] Label,
    Value(u16)
}