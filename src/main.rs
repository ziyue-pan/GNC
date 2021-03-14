extern crate clap;
extern crate pest;

#[macro_use]
extern crate pest_derive;

use clap::{App, Arg};
use parser::AstNode;
use std::path::Path;
use std::borrow::Borrow;

mod parser;
// 导入parser module
mod codegen;    // 导入codegen

fn main() {
    let mut app = App::new("gnalcc")
        .version("0.1.0")
        .author("iamNCJ ~ MartinNose ~ Ziyue")
        .about("gnalc is going to fuck c-lang")
        .arg(Arg::with_name("version").short("v").long("version").help("Show version of gnalcc"))
        .arg(Arg::with_name("FILE").short("c").help("File ready to be compiled").index(1));

    let app_gnalcc = app.clone().get_matches();

    if let Some(file_path) = app_gnalcc.value_of("FILE") {
        // TODO(zc): 在parse前检测文件后缀是否为.c0
        let ast = parser::parse(file_path).unwrap();
        codegen::gen(&ast, file_path);
    } else {
        app.print_help().unwrap();
    }
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_basic_parse() {
        let file_path = "./test/basic.c";
        let ast = parser::parse(file_path).unwrap();
        codegen::gen(&ast, file_path);

    }
}