use parser::{GNCAST, GNCType};
use std::process::exit;
use colored::{Colorize, ColoredString};

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
//
//
// pub fn print_ast(ast: &Vec<GNCAST>) {
//     println!("{}", "GNC".bright_red());
//     let len = ast.len();
//
//     for (i, e) in ast.iter().enumerate() {
//         let prefix = if i != len - 1 { "├── " } else { "└── " };
//         let child_prefix = if i != len - 1 { "│   " } else { "    " };
//         e.print(prefix, child_prefix);
//     }
// }
//
// impl GNCAST {
//     //      COLOR SCHEMES
//     // top: red
//     // function: red
//     // statement: blue
//     // expression: green
//     // type: green
//     // literal & identifier: yellow
//
//
//     pub fn print(&self, prefix: &str, child_prefix: &str) {
//         print!("{}", prefix); // print the prefix
//
//         match self {
//             GNCAST::Function(ty, name, _, _) => {
//                 println!("{}: {}", "function".red(), name.yellow());
//                 print_type(ty, &*(child_prefix.to_owned() + "├── "));
//             }
//             GNCAST::IfStatement(_, _, _) => {
//                 println!("{}", "if statement".blue());
//             }
//             GNCAST::ReturnStatement(_) => {
//                 println!("{}", "return statement".blue());
//             }
//             GNCAST::UnaryExpression(_, _) => {
//                 println!("{}", "unary expression".green());
//             }
//             GNCAST::BinaryExpression(_, _, _) => {
//                 println!("{}", "binary expression".green());
//             }
//             GNCAST::IntLiteral(_) => {
//                 println!("{}", "int literal".yellow());
//             }
//             GNCAST::Identifier(_) => {
//                 println!("{}", "identifier".yellow());
//             }
//             GNCAST::Declaration(_, _) => {
//                 println!("{}", "declaration".blue());
//             }
//             GNCAST::Assignment(_, _, _) => {
//                 println!("{}", "assignment".blue());
//             }
//         }
//     }
// }
//
// pub fn print_type(ty: &GNCType, prefix: &str) {
//     print!("{}", prefix);
//     match ty {
//         GNCType::Void => { println!("{}: {}", "type".green(), "void".yellow()); }
//         GNCType::Int => { println!("{}: {}", "type".green(), "int".yellow()); }
//     }
// }
