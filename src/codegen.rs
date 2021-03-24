use inkwell::context::Context;
use inkwell::module::Module;
use std::path::Path;
use parser::{GnalcAST, GnalcType};
use inkwell::types::FunctionType;

pub struct CodeGen<'ctx> {
    module_name: String,
    context: &'ctx Context,
    module: Module<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, source_path: &str) -> CodeGen<'ctx> {
        let module_name = Path::new(source_path).file_stem().unwrap().to_str().unwrap().to_string();
        let module = context.create_module(module_name.as_str());

        CodeGen {
            module_name,
            context,
            module,
        }
    }


    pub fn gen(&self, ast: &Vec<GnalcAST>) {
        for node in ast {
            match node {
                GnalcAST::Function(ref func_type,
                                   ref func_name,
                                   ref func_param,
                                   ref func_body) => {
                    let llvm_func_type = match func_type {
                        GnalcType::Int => { self.context.i32_type().fn_type(&[], false) }
                        _ => { self.context.i32_type().fn_type(&[], false) }
                    };
                    self.module.add_function(func_name.as_str(), llvm_func_type, None);
                }
                _ => {}
            }
        }
    }
}


// extern crate inkwell;
//
// use std::io::Read;
// use std::fs::{File};
// use std::path::{Path, PathBuf};
//
// use pest::Parser;
// use pest::error::Error;
//
// #[derive(Parser)]
// #[grammar = "./gnalc.pest"]
// struct GnalcParser;
//
// use self::inkwell::builder::Builder;
// use self::inkwell::context::Context;
// use self::inkwell::execution_engine::{ExecutionEngine, JitFunction};
// use self::inkwell::module::{Module, Linkage};
// use self::inkwell::{types, OptimizationLevel};
// use self::inkwell::types::FunctionType;
// use self::inkwell::targets::{Target, InitializationConfig, TargetMachine, RelocMode, CodeModel, FileType};
// use self::inkwell::attributes::AttributeLoc::Function;
//
//
//
// pub fn run(source_path: &str) {
//     let mut source_file: File = File::open(source_path).expect("Unable to open source file!");
//     let mut source_content: String = String::new();
//     source_file.read_to_string(&mut source_content).expect("Unable to read the file!");
//
//
//     // let mut code_gen: CodeGen;
//     // code_gen.module_name = Path::new(source_path).file_stem().unwrap().to_str().unwrap().to_string();
//     // code_gen.context = Context::create();
//     // code_gen.module = code_gen.context.create_module(code_gen.module_name.as_str());
//
//     let context = Context::create();
//     let code_gen = CodeGen::new(&context, source_path);
//     // code_gen.initialize_code_gen(source_path);
//     code_gen.gen(&mut pairs);
// }


//     // pub fn initialize_code_gen(&'ctx mut self, file_path: &str) {
//     //     self.module_name = Path::new(file_path).file_stem().unwrap().to_str().unwrap().to_string();
//     //     self.context = Context::create();
//     //     self.module = self.context.create_module(self.module_name.as_str());
//     // }
//
//
//     pub fn gen(&'ctx self, pairs: &mut pest::iterators::Pairs<Rule>) {
//         for pair in pairs {
//             match pair.as_rule() {
//                 Rule::gnalc => {     // 此处指定AST的入口规则（不改变）
//                     // ast_handler(&pair, &context, &module)
//                 }
//                 _ => { println!("The rule is: {}", pair); }
//             }
//         }
//     }
//
//
//     // pub fn write_target(&'ctx mut self, file_path: &str) {
//     //     let mut llvm_bitcode_path = PathBuf::from(file_path);
//     //     llvm_bitcode_path.set_extension("bc");
//     //     self.module.write_bitcode_to_path(llvm_bitcode_path.as_path());
//     //
//     //     Target::initialize_native(&InitializationConfig::default()).expect("Failed to initialize native target");
//     //
//     //     let triple = TargetMachine::get_default_triple();
//     //     let cpu = TargetMachine::get_host_cpu_name().to_string();
//     //     let features = TargetMachine::get_host_cpu_features().to_string();
//     //
//     //     let target = Target::from_triple(&triple).unwrap();
//     //     let machine = target
//     //         .create_target_machine(
//     //             &triple,
//     //             &cpu,
//     //             &features,
//     //             OptimizationLevel::None,
//     //             RelocMode::Default,
//     //             CodeModel::Default,
//     //         )
//     //         .unwrap();
//     //
//     //     // create a module and do JIT stuff
//     //     let mut target_assembly_path = PathBuf::from(file_path);
//     //     target_assembly_path.set_extension("asm");
//     //     machine.write_to_file(&self.module, FileType::Assembly, target_assembly_path.as_ref()).unwrap();
//     // }
// }
//
// //
// //
// // fn ast_handler(pair: &pest::iterators::Pair<Rule>, context: &Context, module: &Module) {
// //     match pair.as_rule() {
// //         Rule::gnalc => {
// //             for external_declaration in pair.into_inner() {
// //                 ast_handler(&external_declaration, context, module)
// //             }
// //         }
// //         Rule::external_declaration => {
// //             for function in pair.into_inner() {
// //                 ast_handler(&function, context, module)
// //             }
// //         }
// //         Rule::function => {
// //             function_handler(pair, context, module);
// //         }
// //         _ => {
// //             panic!("unsupported parsing result: {}", pair);
// //         }
// //     }
// // }
// //
// //
// // fn function_handler(pair: &pest::iterators::Pair<Rule>, context: &Context, module: &Module) {
// //     let function_name: String;
// //     let mut function_type = context.i64_type().fn_type(&[], false);
// //
// //     for pair in pair.into_inner() {
// //         match pair.as_rule() {
// //             // Rule::data_type => { function_type = function_type_handler(&pair, context); }
// //             Rule::identifier => { function_name = identifier_handler(&pair); }
// //             _ => {}
// //         }
// //         println!("The rule parsed is {}", pair)
// //     }
// //     // name, type, linkage
// //     module.add_function(function_name.as_str(), function_type, None);
// // }
// //
// // // complete four handler
// // // fn function_type_handler(pair: &pest::iterators::Pair<Rule>, context: &Context) -> FunctionType {
// // //     return context.i64_type().fn_type(&[], false);
// // // }
// //
// // fn identifier_handler(pair: &pest::iterators::Pair<Rule>) -> String {
// //     return String::new();
// // }
// //
// //
// // fn function_parameter_list_handler() {}
// //
// // fn function_block_handler() {}
