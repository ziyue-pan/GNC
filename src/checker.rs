use parser::GnalcAST;
use std::process::exit;
use colored::Colorize;

pub struct GnalcError {
    pub code: i32,
    pub description: String,
}


pub fn check(ast: &Vec<GnalcAST>) {
    let errors: Vec<GnalcError> = vec![];

    check_main_exist(ast, &errors);

    if !errors.is_empty() {
        exit(1);
    }
}

pub fn prompt(err: &GnalcError) {
    println!("{}{}{}{}",
             "[ERROR-".red().bold(),
             "]".red().bold(),
             err.code.to_string().as_str().yellow(),
             err.description.as_str().yellow())
}


fn check_main_exist(ast: &Vec<GnalcAST>, errors: &Vec<GnalcError>) {}


