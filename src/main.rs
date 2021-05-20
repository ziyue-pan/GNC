extern crate clap;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate inkwell;
extern crate colored;
#[macro_use(lazy_static)]
extern crate lazy_static;
extern crate walkdir;


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
    use walkdir::WalkDir;
    use colored::{Colorize, ColoredString};
    use std::path::PathBuf;

    #[test]
    fn test_compile() {
        for entry in WalkDir::new("./test")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
        {
            let raw_path = entry.path().to_str();
            if raw_path.is_none() { continue; }

            let source_path = raw_path.unwrap();
            if !source_path.ends_with(".c") { continue; }

            let mut bitcode_path = String::from(source_path);
            bitcode_path.pop();
            bitcode_path.push_str("bc");

            println!(">>> Start compiling {} <<<", source_path.blue());
            // let ast = parser::parse(source_path);
            //
            // let context = Context::create();
            // let mut code_gen = CodeGen::new(&context, source_path);
            // code_gen.gen(&ast);
            //
            // Command::new("sh").arg("-c").arg("llvm-dis ".to_owned() + bitcode_path.as_str())
            //     .output().expect("file to disassemble llvm bitcode");
            println!(">>> done <<<")
        }
    }


    #[test]
    fn test_correctness() {
        // TODO
    }
}