use inkwell::context::Context;
use inkwell::module::Module;
use std::path::{Path, PathBuf};
use parser::{GNCAST, GNCType};
use inkwell::types::FunctionType;
use inkwell::targets::{Target, InitializationConfig, TargetMachine, RelocMode, CodeModel, FileType};
use inkwell::OptimizationLevel;
use inkwell::builder::Builder;

// define global context for LLVM code generator
pub struct CodeGen<'ctx> {
    source_path: &'ctx str,
    module_name: String,
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, source_path: &'ctx str) -> CodeGen<'ctx> {
        let module_name = Path::new(source_path).file_stem().unwrap().to_str().unwrap().to_string();
        let module = context.create_module(module_name.as_str());
        let builder = context.create_builder();

        CodeGen {
            source_path,
            module_name,
            context,
            module,
            builder,
        }
    }


    pub fn gen(&self, ast: &Vec<GNCAST>) {
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
                    let func = self.module.add_function(func_name.as_str(), llvm_func_type, None);
                    let func_block = self.context.append_basic_block(func, func_name);
                    self.builder.position_at_end(func_block);

                    for statement in func_body {
                        self.gen_statement(statement);
                    }
                }
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

    fn gen_statement(&self, statement: &GNCAST) {
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
            _ => {}
        }
    }
}

