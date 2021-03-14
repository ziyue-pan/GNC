extern crate inkwell;

use self::inkwell::builder::Builder;
use self::inkwell::context::Context;
use self::inkwell::execution_engine::{ExecutionEngine, JitFunction};
use self::inkwell::module::Module;
use self::inkwell::{types, OptimizationLevel};
use self::inkwell::targets::{Target, InitializationConfig, TargetMachine, RelocMode, CodeModel, FileType};

use parser::{AstNode, Type};
use std::collections::{hash_map, HashMap};
use std::path::{Path, PathBuf};


pub fn gen(node: &Vec<AstNode>, file_path: &str) {

    // provide llvm environment
    let module_name = Path::new(file_path).file_stem().unwrap().to_str().unwrap();
    let context = Context::create();
    let module = context.create_module(module_name);
    let builder = context.create_builder();


    for ast in node {   // 遍历AST规则
        // TODO 遍历AST规则
    }

    // write bitcode to file
    // let mut llvm_bitcode_path = PathBuf::from(file_path);
    // llvm_bitcode_path.set_extension("bc");
    // module.write_bitcode_to_path(llvm_bitcode_path.as_path());

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
            OptimizationLevel::Aggressive,
            RelocMode::Default,
            CodeModel::Default,
        )
        .unwrap();

    // create a module and do JIT stuff
    let mut target_assembly_path = PathBuf::from(file_path);
    target_assembly_path.set_extension("asm");
    machine.write_to_file(&module, FileType::Assembly, target_assembly_path.as_ref()).unwrap();
}
