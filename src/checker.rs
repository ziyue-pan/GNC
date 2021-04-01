use parser::GNCAST;
use std::process::exit;
use colored::Colorize;

pub struct GNCError {
    pub code: i32,
    pub description: String,
}


pub fn check(ast: &Vec<GNCAST>) {
    let errors: Vec<GNCError> = vec![];

    check_main_exist(ast, &errors);

    if !errors.is_empty() {
        exit(1);
    }
}

pub fn prompt(err: &GNCError) {
    println!("{}{}{}{}",
             "[ERROR-".red().bold(),
             "]".red().bold(),
             err.code.to_string().as_str().yellow(),
             err.description.as_str().yellow())
}


fn check_main_exist(ast: &Vec<GNCAST>, errors: &Vec<GNCError>) {}


