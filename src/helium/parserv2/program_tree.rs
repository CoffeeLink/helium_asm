use std::collections::{BTreeMap, BTreeSet};
use crate::helium::parserv2::constant_type::ConstantType;
use crate::helium::parserv2::program_segment::ProgramSegment;

#[derive(Debug, Default, Clone)]
pub struct ProgramTree {
    pub file_name : String, // the name of the file that has been parsed here.

    /// stores all constants (labels and constants)
    constants : BTreeMap<String, ConstantType>,
    pub segments : Vec<ProgramSegment>,

    /// All files that have been included so far
    includes : BTreeSet<String>,

    // Config and tree metadata.
    /// Stores the last value used by the auto segmentation by skipto.
    /// this is required because labels/segments must have a unique name.
    pub auto_label_id : u32,
    /// if true, all default constants will be used
    pub allow_defaults : bool,
}
impl ProgramTree {
    pub fn new(name : String) -> Self {
        Self {
            file_name: name,
            ..Default::default()
        }
    }
    
    // Const functions.
    pub fn has_const(&mut self, key: &str) -> bool {
        self.constants.contains_key(key)
    }
    pub fn add_const(&mut self, key: String, val: ConstantType) {
        self.constants.insert(key, val);
    }
    pub fn resolve_const(&mut self, key: &str) -> Option<&ConstantType> {
        self.constants.get(key)
    }
    
    // includes
    pub fn has_include(&mut self, file_name: &str) -> bool {
        self.includes.contains(file_name)
    }
    pub fn add_include(&mut self, file_name: String) {
        self.includes.insert(file_name);
    }
}