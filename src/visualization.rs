extern crate wasm_bindgen;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate serde;
use serde::{Serialize};
use wasm_bindgen::prelude::*;
use pest::Parser;

mod parser;

#[derive(Debug, Serialize)]
struct VisTreeNode {
    id: String,
    label: String,
    children: Vec<VisTreeNode>
}

#[wasm_bindgen]
pub fn compile_result(code: &str) -> String {
    let mut pairs = parser::GNCParser::parse(parser::Rule::gnc, &code).unwrap_or_else(|e| panic!("{}", e));
    let gnc_pair = pairs.next().unwrap();
    let ast = parser::parse(gnc_pair);
    let serialized_ast = serde_json::to_string(&ast).unwrap();
    return serialized_ast;
}