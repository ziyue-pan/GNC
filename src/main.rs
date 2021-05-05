extern crate clap;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate inkwell;
extern crate colored;
#[macro_use(lazy_static)]
extern crate lazy_static;


use clap::{App, Arg};
use std::path::Path;
use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;
use std::process::Command;
use inkwell::context::Context;

mod parser;
mod codegen;
mod checker;

use codegen::CodeGen;
use checker::GNCError;

fn main() {
    let mut app = App::new("gncc")
        .version("0.1.0")
        .author("iamNCJ ~ MartinNose ~ Ziyue")
        .about("gnc is going to fuck c-lang")
        .arg(Arg::with_name("version").short("v").long("version").help("Show version of gncc"))
        .arg(Arg::with_name("FILE").short("c").help("File ready to be compiled").index(1));

    let app_gncc = app.clone().get_matches();

    if let Some(file_path) = app_gncc.value_of("FILE") {
        let split = file_path.split(".");
        let split = split.collect::<Vec<&str>>();

        if split.len() == 0 || split[split.len() - 1] != "c" {
            checker::prompt(&GNCError {
                code: 0,
                description: String::from("the source file extension must be `.c`!"),
            });
        }

        let ast = parser::parse(file_path);

        checker::check(&ast);

        let context = Context::create();
        let mut code_gen = CodeGen::new(&context, file_path);
        code_gen.gen(&ast);
    } else {
        app.print_help().unwrap();
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_basic() {
        let file_path = "./test/basic/basic.c";

        let ast = parser::parse(file_path);

        checker::check(&ast);

        let context = Context::create();
        let mut code_gen = CodeGen::new(&context, file_path);
        code_gen.gen(&ast);

        Command::new("sh").arg("-c").arg("llvm-dis test/basic/basic.bc").output().expect("failed to execute process");
    }

    #[test]
    fn test_unary() {
        let file_path = "./test/unary/unary.c";

        let ast = parser::parse(file_path);

        checker::check(&ast);

        let context = Context::create();
        let mut code_gen = CodeGen::new(&context, file_path);
        code_gen.gen(&ast);

        Command::new("sh").arg("-c").arg("llvm-dis test/unary/unary.bc").output().expect("failed to execute process");
    }

    #[test]
    fn test_add_mul() {
        let file_path = "./test/add_mul/add_mul.c";

        let ast = parser::parse(file_path);

        checker::check(&ast);

        let context = Context::create();
        let mut code_gen = CodeGen::new(&context, file_path);
        code_gen.gen(&ast);

        Command::new("sh").arg("-c").arg("llvm-dis test/add_mul/add_mul.bc").output().expect("failed to execute process");
    }
}