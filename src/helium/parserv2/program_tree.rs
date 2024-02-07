use std::collections::BTreeMap;

struct ProgramTree {
    file_name : String, // the name of the file that has been parsed here.

    constants : BTreeMap<&'static str, ()>
}
