extern crate inkwell;

use self::inkwell::builder::Builder;
use self::inkwell::context::Context;
use self::inkwell::execution_engine::{ExecutionEngine, JitFunction};
use self::inkwell::module::Module;
use self::inkwell::{types, OptimizationLevel};
use self::inkwell::targets::{Target, InitializationConfig, TargetMachine, RelocMode, CodeModel, FileType};


use std::collections::{hash_map, HashMap};
use std::path::{Path, PathBuf};


pub fn gen(pairs: &pest::iterators::Pairs<Rule>, file_path: &str) {

    // provide llvm environment
    let module_name = Path::new(file_path).file_stem().unwrap().to_str().unwrap();
    let context = Context::create();
    let module = context.create_module(module_name);
    let builder = context.create_builder();

    for pair in pairs {
        match pair.as_rule() {
            Rule::gnalc => {     // 此处指定AST的入口规则（不改变）
                parse_ast(&pair, &context, &module)
            }
            _ => { println!("The rule is: {}", pair); }
        }
    }


    // write bitcode to file
    let mut llvm_bitcode_path = PathBuf::from(file_path);
    llvm_bitcode_path.set_extension("bc");
    module.write_bitcode_to_path(llvm_bitcode_path.as_path());

    Target::initialize_native(&InitializationConfig::default()).expect("Failed to initialize native target");

    let triple = TargetMachine::get_default_triple();
    let cpu = TargetMachine::get_host_cpu_name().to_string();
    let features = TargetMachine::get_host_cpu_features().to_string();

    let target = Target::from_triple(&triple).unwrap();
    let machine = target
        .create_target_machine(
            &triple,
            &cpu,
            &features,
            OptimizationLevel::None,
            RelocMode::Default,
            CodeModel::Default,
        )
        .unwrap();

    // create a module and do JIT stuff
    let mut target_assembly_path = PathBuf::from(file_path);
    target_assembly_path.set_extension("asm");
    machine.write_to_file(&module, FileType::Assembly, target_assembly_path.as_ref()).unwrap();
}


fn parse_ast(pair: &pest::iterators::Pair<Rule>, context: &Context, module: &Module) {
    match pair.as_rule() {
        Rule::gnalc => {
            for external_declaration in pair.into_inner() {
                parse_ast(&external_declaration, context, module)
            }
        }
    }
}


// pub fn gen(node: &Vec<AstNode>, file_path: &str) {
//
//     // provide llvm environment
//     let module_name = Path::new(file_path).file_stem().unwrap().to_str().unwrap();
//     let context = Context::create();
//     let module = context.create_module(module_name);
//     let builder = context.create_builder();
//
//
//
//     for ast in node {   // 遍历AST规则
//         match ast {
//             AstNode::Print(ref ptr_to_ast_nodes) => {
//                 println!("Print rule has been parsed!")
//             }
//             AstNode::ExternalDeclarationList(ref vec_of_ast_node) => {
//                 // parse_ast(vec_of_ast_node, context, module);
//                 println!("AST Parsed! Parse an External Declaration List.")
//             }
//
//             AstNode::Function(ref ref_type, ref param, ref funct_name, ref ptr_to_body) => {
//                 let i64_type = context.i64_type();
//                 let fn_type = i64_type.fn_type(&[], false);
//                 module.add_function(funct_name, fn_type, None);
//                 println!("AST Parsed! Generate a function.")
//             }
//             _ => { println!("Yet to Specified."); }
//         }
//     }
//
//     // write bitcode to file
//     let mut llvm_bitcode_path = PathBuf::from(file_path);
//     llvm_bitcode_path.set_extension("bc");
//     module.write_bitcode_to_path(llvm_bitcode_path.as_path());
//
//     Target::initialize_native(&InitializationConfig::default()).expect("Failed to initialize native target");
//
//     let triple = TargetMachine::get_default_triple();
//     let cpu = TargetMachine::get_host_cpu_name().to_string();
//     let features = TargetMachine::get_host_cpu_features().to_string();
//
//     let target = Target::from_triple(&triple).unwrap();
//     let machine = target
//         .create_target_machine(
//             &triple,
//             &cpu,
//             &features,
//             OptimizationLevel::None,
//             RelocMode::Default,
//             CodeModel::Default,
//         )
//         .unwrap();
//
//     // create a module and do JIT stuff
//     let mut target_assembly_path = PathBuf::from(file_path);
//     target_assembly_path.set_extension("asm");
//     machine.write_to_file(&module, FileType::Assembly, target_assembly_path.as_ref()).unwrap();
// }


// TODO 遍历AST规则
// fn parse_ast(node: &Vec<AstNode>, context: &Context, module: &Module) {
//     for ast in node {   // 遍历AST规则
//         match ast {
//             AstNode::ExternalDeclarationList(ref vec_of_ast_node) => {
//                 parse_ast(vec_of_ast_node, context, module);
//                 println!("AST Parsed! Parse an External Declaration List.")
//             }
//             AstNode::Function(ref ref_type, ref param, ref funct_name, ref ptr_to_body) => {
//                 let i64_type = context.i64_type();
//                 let fn_type = i64_type.fn_type(&[], false);
//                 module.add_function(funct_name, fn_type, None);
//                 println!("AST Parsed! Generate a function.")
//             }
//             _ => { println!("Yet to Specified."); }
//         }
//     }
// }
