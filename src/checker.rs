use parser::GnalcAST;
use std::process::exit;

pub enum GnalcError {
    MissingMain(i32, String),
}


pub fn check(ast: &Vec<GnalcAST>) {
    let errors: Vec<GnalcError> = vec![];

    check_main_exist(ast, &errors);


    if !errors.is_empty() {
        exit(1);
    }
}


fn check_main_exist(ast: &Vec<GnalcAST>, errors: &Vec<GnalcError>) {}