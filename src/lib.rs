use std::env::args;

pub mod helium;

pub struct Config {
    input_file : String
}

impl Config {
    pub fn from_args() -> Result<Self, ()> {
        let mut args = args();
        args.next();
        let file = args.next().ok_or(())?;
        Ok(Self {
            input_file: file
        })
    }
    pub fn get_file(self) -> String {
        self.input_file
    }
}