use crate::helium::parserv2::program_element::ProgramElement;

#[derive(Debug, Clone)]
pub struct ProgramSegment {
    name : String,
    origin : Option<u32>,
    elements : Vec<ProgramElement>
}

impl ProgramSegment {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            origin: None,
            elements: vec![],
        }
    }
    pub fn with_origin(name : &str, origin : u32) -> Self {
        Self {
            name: name.to_string(),
            origin: Some(origin),
            elements: vec![],
        }
    }
}