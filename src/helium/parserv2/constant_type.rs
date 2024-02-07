

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ConstantType {
    #[default] Unknown,
    Value(u16),
    Label
}