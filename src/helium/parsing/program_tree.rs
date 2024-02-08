use std::collections::{BTreeMap, BTreeSet, LinkedList};
use std::fmt::{Display, Formatter};
use crate::helium::parsing::constant_type::ConstantType;
use crate::helium::parsing::ConstantType::Unknown;
use crate::helium::parsing::default_constants::DEFAULT_CONSTANTS;
use crate::helium::parsing::program_segment::ProgramSegment;

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
            allow_defaults: true,
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

    /// Checks if all references/consts can be resolved.
    /// Returns Ok() if all references are resolved.
    /// If not, returns Err(yVec<String>) containing all unresolved references.
    pub fn check_all_resolved(&mut self) -> Result<(), Vec<String>> {
        let mut unresolved: Vec<String> = vec![];

        for (k, v) in self.constants.clone() {
            if v == Unknown {
                unresolved.push(k);
            }
        }

        if unresolved.is_empty() {
            Ok(())
        } else {
            Err(unresolved)
        }
    }

    /// Only run if its a root segment.
    pub fn complete(&mut self) {
        if self.allow_defaults {
            self.constants.extend(DEFAULT_CONSTANTS.clone());
        }
    }

    // includes
    pub fn has_include(&mut self, file_name: &str) -> bool {
        self.includes.contains(file_name) || (self.allow_defaults && DEFAULT_CONSTANTS.contains_key(file_name))
    }
    pub fn add_include(&mut self, file_name: String) {
        self.includes.insert(file_name);
    }
}

impl Display for ProgramTree {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ProgramTree of: {}", self.file_name)?;
        writeln!(f, "Config:")?;
        writeln!(f, "  Allow Defaults: {}", self.allow_defaults)?;
        writeln!(f, "  AutoLabelID: {}", self.auto_label_id)?;

        writeln!(f, "Constants: {:?}", self.constants)?;

        // Segments
        let seg_temp = self.segments.clone();
        let mut segments = seg_temp.iter().peekable();

        writeln!(f, "Segments:")?;
        while let Some(segment) = segments.next().cloned() {
            if segment.origin.is_some() {
                writeln!(f, "@{}\n{}:", segment.origin.unwrap(), segment.name)?;
            } else {
                writeln!(f, "{}:", segment.name)?;
            }

            if segment.elements.is_empty() {continue}

            for element in segment.elements {
                writeln!(f, "    {}", element)?;
            }
            if segments.peek().is_some() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}