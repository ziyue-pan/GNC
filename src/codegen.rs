use inkwell::context::Context;
use inkwell::module::Module;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use parser::{GNCAST, GNCType};
use inkwell::types::FunctionType;
use inkwell::targets::{Target, InitializationConfig, TargetMachine, RelocMode, CodeModel, FileType};
use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::values::PointerValue;


// define global context for LLVM code generator


pub struct CodeGen<'ctx> {
    source_path: &'ctx str,
    module_name: String,
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    addr_map_stack: Vec<HashMap<String, PointerValue<'ctx>>>
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, source_path: &'ctx str) -> CodeGen<'ctx> {
        let module_name = Path::new(source_path).file_stem().unwrap().to_str().unwrap().to_string();
        let module = context.create_module(module_name.as_str());
        let builder = context.create_builder();
        let mut addr_map_stack = Vec::new();
        let mut global_map: HashMap<String, PointerValue> = HashMap::new();
        addr_map_stack.push(global_map);

        CodeGen {
            source_path,
            module_name,
            context,
            module,
            builder,
            addr_map_stack,
        }
    }

    pub fn gen(&mut self, ast: &Vec<GNCAST>) {
        for node in ast {
            match node {
                GNCAST::Function(ref func_type,
                                   ref func_name,
                                   ref func_param,
                                   ref func_body) => {
                    let llvm_func_type = match func_type {
                        GNCType::Int => { self.context.i32_type().fn_type(&[], false) }
                        _ => { self.context.i32_type().fn_type(&[], false) }
                    };
                    let local_map: HashMap<String, PointerValue> = HashMap::new();
                    self.addr_map_stack.push(local_map);
                    let func = self.module.add_function(func_name.as_str(), llvm_func_type, None);
                    let func_block = self.context.append_basic_block(func, func_name);
                    self.builder.position_at_end(func_block);

                    for statement in func_body {
                        self.gen_statement(statement);
                    }
                    self.addr_map_stack.pop();
                }
                // TODO Update global hashmap: addr_map_stack[addr_map_stack.len() - 1].insert(identifier, PointerValue);
                _ => {}
            }
        }

        let mut llvm_bitcode_path = PathBuf::from(self.source_path);
        llvm_bitcode_path.set_extension("bc");
        self.module.write_bitcode_to_path(llvm_bitcode_path.as_path());

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

        let mut target_assembly_path = PathBuf::from(self.source_path);
        target_assembly_path.set_extension("asm");
        machine.write_to_file(&self.module, FileType::Assembly, target_assembly_path.as_ref()).unwrap();
    }

    fn get_point_value(&self, identifier: &String) -> PointerValue {
        for map in self.addr_map_stack.iter().rev() {
            match map.get(identifier) {
                Some(addr) => {return *addr}
                _ => {}
            }
        }
        panic!(identifier.to_string() + " not found!");
    }

    fn gen_statement(&mut self, statement: &GNCAST) {
        match statement {
            GNCAST::ReturnStatement(ref ptr_to_expr) => {
                match **ptr_to_expr {
                    GNCAST::IntLiteral(ref int_literal) => {
                        let i32_literal = self.context.i32_type().const_int(*int_literal as u64, true);
                        self.builder.build_return(Some(&i32_literal));
                    }

                    _ => {}
                }
            }
            GNCAST::Declaration(ref data_type, ref identifier ) => {
                match data_type {
                    GNCType::Int => {
                        let point_value = self.builder.build_alloca(self.context.i32_type(), identifier);
//                        self.get_current_map().insert(identifier.to_string(), point_value);
//                        match self.addr_map_stack.iter_mut().rev().next() {
                        match self.addr_map_stack.last_mut() {
                            Some(mut map) => { map.insert(identifier.to_string(), point_value); }
                            _ => {panic!(identifier.to_string() + " not found. Addr HashMap Stack overflow"); }
                        }

                    }
                    _ => {
                        panic!("Invalid Type")
                    }
                }
            }
            GNCAST::Assignment(ref identifier, ref ptr_to_expr) => {
                match **ptr_to_expr {
                    GNCAST::IntLiteral(ref int_literal) => {
                        let i32_literal = self.context.i32_type().const_int(*int_literal as u64, true);

                        self.builder.build_store(self.get_point_value(identifier), i32_literal);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

