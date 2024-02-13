use crate::helium::parsing::program_element::ProgramElement;

#[derive(Debug, Clone)]
pub struct ProgramSegment {
    pub name: String,
    pub origin: Option<u16>,
    pub elements: Vec<ProgramElement>,
}

impl ProgramSegment {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            origin: None,
            elements: vec![],
        }
    }
    pub fn with_origin(name: &str, origin: u16) -> Self {
        Self {
            name: name.to_string(),
            origin: Some(origin),
            elements: vec![],
        }
    }
}
