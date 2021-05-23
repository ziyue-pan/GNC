use colored::{Colorize};
use std::process;

//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//          GNCError
//    Defines all the recoverable
// errors when GNC executing.
//<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

pub enum GNCError {
    InvalidSuffix,
    // invalid suffix
    MissingFunction,
    DuplicateFunction,
}

impl GNCError {
    pub fn msg(&self) -> &str {
        match self {
            GNCError::InvalidSuffix => "the source file extension must be `.c`!",
            GNCError::MissingFunction => "",
            GNCError::DuplicateFunction => "",
        }
    }


    pub fn prompt(&self) {
        println!("{} {}", "[ERROR]".red().bold(), self.msg());

        match self {
            GNCError::InvalidSuffix => {
                process::exit(1);
            }
            GNCError::MissingFunction => {}
            GNCError::DuplicateFunction => {}
        }
    }
}