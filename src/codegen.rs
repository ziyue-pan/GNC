use inkwell::context::Context;
use inkwell::module::Module;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, VecDeque};
use parser::{GNCAST, UnaryOperator, BinaryOperator, AssignOperation, GNCParameter};
use inkwell::targets::{Target, InitializationConfig, TargetMachine, RelocMode, CodeModel, FileType};
use inkwell::{IntPredicate, FloatPredicate};
use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::values::{PointerValue, FunctionValue, BasicValue, BasicValueEnum, InstructionOpcode};
use inkwell::basic_block::BasicBlock;
use inkwell::types::{BasicTypeEnum, BasicType, FunctionType};
use checker::{GNCErr};
use anyhow::Result;
use types::GNCType;


//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//              Global Configuration
// define global context for LLVM code generator
//<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
pub struct CodeGen<'ctx> {
    source_path: &'ctx str,
    // module_name: String,
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    addr_map_stack: Vec<HashMap<String, (GNCType, PointerValue<'ctx>)>>,

    //>>>>>>>>>>>>>>>>>>>>>>>>
    //      LLVM Blocks
    //<<<<<<<<<<<<<<<<<<<<<<<<

    // current function block
    current_function: Option<(FunctionValue<'ctx>, Option<GNCType>)>,
    // break labels (in loop statements)
    break_labels: VecDeque<BasicBlock<'ctx>>,
    // continue labels (in loop statements)
    continue_labels: VecDeque<BasicBlock<'ctx>>,
    // hashset for functions
    function_map: HashMap<String, (Option<GNCType>, Vec<GNCType>)>,
    // hashset for global variable
    global_variable_map: HashMap<String, (GNCType, PointerValue<'ctx>)>,
}

impl<'ctx> CodeGen<'ctx> {
    // new LLVM context
    pub fn new(context: &'ctx Context, source_path: &'ctx str) -> CodeGen<'ctx> {
        let module_name = Path::new(source_path).file_stem().unwrap().to_str().unwrap().to_string();
        let module = context.create_module(module_name.as_str());
        let builder = context.create_builder();

        // set variable scope
        let mut addr_map_stack = Vec::new();
        let global_map: HashMap<String, (GNCType, PointerValue<'ctx>)> = HashMap::new();
        addr_map_stack.push(global_map); // push global variable hashmap

        CodeGen { // return value
            source_path,
            // module_name,
            context,
            module,
            builder,
            addr_map_stack,
            current_function: None,
            break_labels: VecDeque::new(),
            continue_labels: VecDeque::new(),
            function_map: HashMap::new(),
            global_variable_map: HashMap::new(),
        }
    }

    // generate all code
    pub fn gen(&mut self, ast: &Vec<GNCAST>) -> Result<()> {
        // first scan
        for node in ast {
            match node {
                GNCAST::Function(ref func_type,
                                 ref func_name,
                                 ref func_param, _) => {
                    self.gen_function_proto(func_type, func_name, func_param)?;
                }
                GNCAST::GlobalDeclaration(
                    ref var_type,
                    ref var_name,
                    ref ptr_to_init) => {
                    self.gen_global_variable(var_type, var_name, ptr_to_init)?;
                }
                _ => { panic!(); }
            }
        }

        // second scan
        for node in ast {
            match node {
                GNCAST::Function(ref func_type,
                                 ref func_name,
                                 ref func_param,
                                 ref func_body) => {
                    self.gen_function_def(func_type, func_name, func_param, func_body)?;
                }
                _ => {}
            }
        }

        // set llvm bitcode path
        let mut llvm_bitcode_path = PathBuf::from(self.source_path);
        llvm_bitcode_path.set_extension("bc");
        self.module.write_bitcode_to_path(llvm_bitcode_path.as_path());

        // set llvm target
        Target::initialize_native(&InitializationConfig::default())
            .expect("Failed to initialize native target");

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

        // write assembly code
        let mut target_assembly_path = PathBuf::from(self.source_path);
        target_assembly_path.set_extension("asm");
        machine.write_to_file(&self.module,
                              FileType::Assembly,
                              target_assembly_path.as_ref()).unwrap();
        Ok(())
    }


    // generate global variable
    fn gen_global_variable(&mut self,
                           var_type: &GNCType,
                           var_name: &String,
                           ptr_to_init: &Box<GNCAST>) -> Result<()> {
        // check
        if self.global_variable_map.contains_key(var_name) {
            return Err(GNCErr::DuplicateGlobalVar(var_name.to_string()).into());
        }
        if self.function_map.contains_key(var_name) {
            return Err(GNCErr::Redefinition(var_name.to_string()).into());
        }

        if var_type == &GNCType::Void {
            return Err(GNCErr::VoidVariable(var_name.to_string()).into());
        }

        // generate global variable
        let global_value = self.module.add_global(
            var_type.to_basic_llvm_type(self.context),
            None,
            var_name.as_str(),
        );

        // TODO add const_val check
        let init_val_pair = self.gen_expression(&**ptr_to_init)?;
        let cast_ty = init_val_pair.0.default_cast(var_type)?;
        let cast_v = self.cast_value(&init_val_pair.0, &init_val_pair.1, &cast_ty)?;

        global_value.set_initializer(&cast_v);

        self.global_variable_map.insert(var_name.to_string(),
                                        (var_type.to_owned(), global_value.as_pointer_value()));
        Ok(())
    }


    // generate function proto
    fn gen_function_proto(&mut self,
                          ret_type: &GNCType,
                          func_name: &String,
                          func_param: &Vec<GNCParameter>) -> Result<()> {
//        println!("[DEBUG] generate function protocol");

        // cannot handle duplicate function
        if self.function_map.contains_key(func_name) {
            return Err(GNCErr::DuplicateFunction(func_name.to_string()).into());
        }
        if self.global_variable_map.contains_key(func_name) {
            return Err(GNCErr::Redefinition(func_name.to_string()).into());
        }

        // function parameter should be added in this llvm_func_type
        let mut llvm_params: Vec<BasicTypeEnum<'ctx>> = Vec::new();
        let mut params: Vec<GNCType> = Vec::new();

        for param in func_param {
            params.push(param.to_owned().param_type);
            llvm_params.push(param.param_type.to_basic_llvm_type(self.context));
        }

        let llvm_func_ty = self.to_return_type(ret_type, &llvm_params)?;

        // create function
        self.module.add_function(func_name.as_str(), llvm_func_ty, None);

        let ret_ty = if *ret_type != GNCType::Void {
            Some(ret_type.to_owned())
        } else {
            None
        };

        self.function_map.insert(func_name.to_owned(), (ret_ty, params));
        Ok(())
    }


    fn gen_function_def(&mut self,
                        func_type: &GNCType,
                        func_name: &String,
                        func_param: &Vec<GNCParameter>,
                        func_body: &Vec<GNCAST>) -> Result<()> {
//        println!("[DEBUG] generate function definition");
        // push local map
        let local_map: HashMap<String, (GNCType, PointerValue<'ctx>)> = HashMap::new();
        self.addr_map_stack.push(local_map);

        let func_option = self.module.get_function(func_name.as_str());
        if func_option.is_none() {
            panic!();
        }
        let func = func_option.unwrap();
        let func_ty = self.function_map.get(func_name).unwrap().to_owned().0;
        self.current_function = Some((func, func_ty));

        // create function block
        let func_block = self.context.append_basic_block(func, "entry");
        self.builder.position_at_end(func_block);

        // function parameter store
        for (i, arg) in func.get_param_iter().enumerate() {
            // get param name
            let arg_name = &(*func_param[i].param_name);
            // set param name
            arg.set_name(arg_name);

            let builder = self.context.create_builder();
            let func_entry = func.get_first_basic_block().unwrap();

            match func_entry.get_first_instruction() {
                Some(first_inst) => builder.position_before(&first_inst),
                None => builder.position_at_end(func_entry),
            }

            let ty = func_param[i].to_owned().param_type;

            // alloc variable on stack
            let alloca = builder.build_alloca(
                ty.to_basic_llvm_type(self.context),
                &arg_name,
            );
            self.gen_variable(&ty, &arg_name.to_string(), alloca)?;
        }

        // generate IR for statements inside the function body
        for statement in func_body {
            self.gen_statement(statement)?;
        }

        // build terminator for each non-terminated block
        let mut iter_block = func.get_first_basic_block();
        while iter_block.is_some() {
            let block = iter_block.unwrap();
            if block.get_terminator().is_none() {
                let terminator_builder = self.context.create_builder();
                terminator_builder.position_at_end(block);
                match func_type {
                    GNCType::Void => {
                        terminator_builder.build_return(None);
                    }
                    _ => {
                        let null_val = self.context.i32_type().const_zero();
                        terminator_builder.build_return(Some(&null_val));
                    }
                }
            }
            iter_block = block.get_next_basic_block();
        }

        self.addr_map_stack.pop();
        self.current_function = None;
        Ok(())
    }


//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>


    fn gen_variable(&mut self,
                    var_type: &GNCType,
                    identifier: &String,
                    ptr: PointerValue<'ctx>) -> Result<()> {
        let local_map = self.addr_map_stack.last_mut().unwrap();

        if local_map.contains_key(identifier) {
            return Err(GNCErr::DuplicateVar(identifier.to_string()).into());
        }

        local_map.insert(identifier.to_string(), (var_type.clone(), ptr));
        Ok(())
    }

    fn gen_statement(&mut self, statement: &GNCAST) -> Result<()> {
//        dbg!("statement", statement);
        match statement {
            GNCAST::ReturnStatement(ref ptr_to_expr) => {
//                dbg!("return statement");
                if ptr_to_expr.is_some() {
                    let func_pair = self.current_function.as_ref().unwrap().to_owned();

                    let ret_ty_opt = func_pair.1;

                    // return type mismatch
                    if ret_ty_opt.is_none() {
                        return Err(GNCErr::ReturnTypeMismatch().into());
                    }

                    // get return type and expr metadata
                    let ret_ty = ret_ty_opt.unwrap();
                    let expr = ptr_to_expr.as_ref().as_ref().unwrap();
                    let expr_pair = self.gen_expression(&expr)?;

                    // cast metadata
                    let cast_ty = expr_pair.0.default_cast(&ret_ty)?;
                    let ret_val = self.cast_value(&expr_pair.0, &expr_pair.1, &cast_ty)?;

                    self.builder.build_return(Some(&ret_val));
                } else {
                    self.builder.build_return(None);
                }
            }
            GNCAST::Declaration(ref data_type, ref identifier) => {
//                println!("[DEBUG] generate declaration");
                let point_value = self.builder.build_alloca(data_type.to_basic_llvm_type(self.context), identifier);
                self.gen_variable(data_type, identifier, point_value)?;
            }
            GNCAST::FunctionCall(ref function_name,
                                 ref parameters) => {
                self.gen_function_call(function_name, parameters)?;
            }
            GNCAST::Assignment(ref op,
                               ref lhs,
                               ref rhs) => {
//                dbg!("generate assignment");
                let lvalue_pair = self.get_lvalue(lhs)?;

                let val_pair = self.gen_binary_expression(
                    &match op {
                        AssignOperation::Addition => BinaryOperator::Add,
                        AssignOperation::BitwiseAnd => BinaryOperator::BitwiseAnd,
                        AssignOperation::Subtraction => BinaryOperator::Subtract,
                        AssignOperation::Multiplication => BinaryOperator::Multiply,
                        AssignOperation::Division => BinaryOperator::Divide,
                        AssignOperation::Modulus => BinaryOperator::Modulus,
                        AssignOperation::ShiftLeft => BinaryOperator::ShiftLeft,
                        AssignOperation::ShiftRight => BinaryOperator::ShiftRight,
                        AssignOperation::ExclusiveOr => BinaryOperator::ExclusiveOr,
                        AssignOperation::InclusiveOr => BinaryOperator::InclusiveOr,
                        _ => BinaryOperator::FetchRHS
                    }, lhs, rhs,
                )?;

                // cast
                let cast_ty = val_pair.0.default_cast(&lvalue_pair.0)?;
                let cast_v = self.cast_value(&val_pair.0, &val_pair.1, &cast_ty)?;

                self.builder.build_store(lvalue_pair.1, cast_v);
            }
            GNCAST::IfStatement(ref cond,
                                ref if_statements,
                                ref else_statements) => {
                self.gen_if_statement(cond, if_statements, else_statements)?;
            }
            GNCAST::WhileStatement(ref is_do_while,
                                   ref cond,
                                   ref while_statements) => {
                self.gen_while_statements(*is_do_while, cond, while_statements)?;
            }
            GNCAST::ContinueStatement => {
                if self.continue_labels.is_empty() {
                    panic!();
                }
                let continue_block = self.continue_labels.back().unwrap();
                self.builder.build_unconditional_branch(*continue_block);
            }
            GNCAST::BreakStatement => {
                if self.break_labels.is_empty() {
                    panic!();
                }
                let break_block = self.break_labels.back().unwrap();
                self.builder.build_unconditional_branch(*break_block);
            }
            GNCAST::ForStatement(ref init_clauses,
                                 ref cond,
                                 ref step,
                                 ref for_statements) => {
                self.gen_for_statement(init_clauses, cond, step, for_statements)?;
            }
            _ => {
                panic!("Invalid Statement");
            }
        }
        Ok(())
    }


    fn gen_for_statement(&mut self,
                         initial_statements: &Vec<GNCAST>,
                         cond: &Box<Option<GNCAST>>,
                         step: &Box<Option<GNCAST>>,
                         for_statements: &Box<GNCAST>) -> Result<()> {
        let func = self.current_function.as_ref().unwrap().0;

        let before_block = self.context.append_basic_block(func,
                                                           "before_block");
        let loop_block = self.context.append_basic_block(func,
                                                         "for_block");
        let step_block = self.context.append_basic_block(func,
                                                         "step_block");
        let after_block = self.context.append_basic_block(func,
                                                          "after_block");

        self.continue_labels.push_back(step_block);
        self.break_labels.push_back(after_block);

        // generate initial clauses
        for init_clause in initial_statements {
            self.gen_statement(init_clause)?;
        }
        self.builder.build_unconditional_branch(before_block);

        // build before block
        self.builder.position_at_end(before_block);
        let cond_expr = cond.as_ref().as_ref();

        // generate for condition
        if cond_expr.is_none() {
            self.builder.build_unconditional_branch(loop_block);
        } else {
            let cond = self.gen_expression(cond_expr.unwrap())?;

            self.builder.build_conditional_branch(cond.1.into_int_value(),
                                                  loop_block,
                                                  after_block);
        }

        // generate for-loop body
        self.builder.position_at_end(loop_block);
        self.gen_block_statements(for_statements)?;
        self.builder.build_unconditional_branch(step_block);

        // generate step-clause
        self.builder.position_at_end(step_block);
        let step_statement = step.as_ref().as_ref();
        if step_statement.is_some() {
            self.gen_statement(step_statement.unwrap())?;
        }

        if self.no_terminator() {
            self.builder.build_unconditional_branch(before_block);
        }

        // generate after block
        self.builder.position_at_end(after_block);

        self.continue_labels.pop_back();
        self.break_labels.pop_back();

        Ok(())
    }


    // generate if-else statements
    fn gen_if_statement(&mut self, cond: &Box<GNCAST>,
                        if_statements: &Box<GNCAST>,
                        else_statements: &Box<GNCAST>) -> Result<()> {
        // get current function
        let func = self.current_function.as_ref().unwrap().0;

        // get condition
        let cond = self.gen_expression(cond)?;

        // append 3 blocks
        let if_block = self.context.append_basic_block(func,
                                                       "if_block");
        let else_block = self.context.append_basic_block(func,
                                                         "else_block");
        let merge_block = self.context.append_basic_block(func,
                                                          "merge_block");

        // build condition statement
        self.builder.build_conditional_branch(cond.1.into_int_value(),
                                              if_block,
                                              else_block);

        // build if_block
        self.builder.position_at_end(if_block);
        self.gen_block_statements(if_statements)?;

        if self.no_terminator() {
            self.builder.build_unconditional_branch(merge_block);
        }

        // build else_block
        self.builder.position_at_end(else_block);
        self.gen_block_statements(else_statements)?;

        if self.no_terminator() {
            self.builder.build_unconditional_branch(merge_block);
        }

        self.builder.position_at_end(merge_block);

        Ok(())
    }


    // generate while or do-while statements
    fn gen_while_statements(&mut self,
                            is_do_while: bool,
                            cond: &Box<GNCAST>,
                            while_statements: &Box<GNCAST>) -> Result<()> {
//        println!("[DEBUG] generate {} statement", if is_do_while { "do_while" } else { "while" });

        let func = self.current_function.as_ref().unwrap().0;

        let before_block =
            self.context.append_basic_block(func, "before_while");
        let while_block =
            self.context.append_basic_block(func,
                                            if is_do_while { "do_while" } else { "while" });
        let after_block =
            self.context.append_basic_block(func,
                                            "after_loop");

        // push labels
        // linking to the blocks
        self.continue_labels.push_back(while_block);
        self.break_labels.push_back(after_block);

        // unconditional branch to before_block
        self.builder.build_unconditional_branch(before_block);


        // build before block
        self.builder.position_at_end(before_block);
        let cond_val = self.gen_expression(cond)?;

        if is_do_while {
            // build do-while unconditional branch
            self.builder.build_unconditional_branch(while_block);
        } else {
            // build while conditional branch
            self.builder.build_conditional_branch(cond_val.1.into_int_value(),
                                                  while_block,
                                                  after_block);
        }
        self.builder.position_at_end(while_block);

        // build while block
        self.gen_block_statements(while_statements)?;
        if self.no_terminator() {
            if is_do_while {
                let do_while_cond = self.gen_expression(cond)?;

                self.builder.build_conditional_branch(do_while_cond.1.into_int_value(),
                                                      before_block,
                                                      after_block);
            } else {
                self.builder.build_unconditional_branch(before_block);
            }
        }

        // position to after block
        self.builder.position_at_end(after_block);

        self.break_labels.pop_back();
        self.continue_labels.pop_back();

        Ok(())
    }


    // generate block statement (scope)
    fn gen_block_statements(&mut self, block: &Box<GNCAST>) -> Result<()> {
        let local_map: HashMap<String, (GNCType, PointerValue<'ctx>)>
            = HashMap::new();
        self.addr_map_stack.push(local_map);

        match **block {
            GNCAST::BlockStatement(ref statements) => {
                for statement in statements {
                    self.gen_statement(statement)?;
                }
            }
            _ => { panic!() }
        }

        self.addr_map_stack.pop();
        Ok(())
    }

    //>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
    //      generate expressions (type & value)
    //<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<


    fn gen_function_call(&self,
                         function_name: &String,
                         parameters: &Vec<GNCAST>) -> Result<(Option<GNCType>,
                                                              Option<BasicValueEnum<'ctx>>)> {
//        println!("[DEBUG] generate function call");

        // get function and return type
        let func_ty_opt = self.function_map.get(function_name);
        let llvm_func_opt = self.module.get_function(function_name);

        // handle not found error
        if func_ty_opt.is_none() || llvm_func_opt.is_none() {
            let err = GNCErr::MissingFunction(function_name.to_string());
            return Err(err.into());
        }

        // get parameter count
        let func = llvm_func_opt.unwrap();
        let func_param_count = func.get_type().count_param_types();

        // func types
        let func_ty_pair = func_ty_opt.unwrap().to_owned();

        // handle calling parameter count mismatch
        if parameters.len() != func_param_count as usize {
            let err = GNCErr::ParameterCountMismatch(
                function_name.to_string(),
                func_param_count as usize,
                parameters.len());
            return Err(err.into());
        }

        // prepare function call arguments
        let mut compiled_args: Vec<BasicValueEnum> =
            Vec::with_capacity(parameters.len());
        let param_types = func_ty_pair.1.to_owned();

        for (i, arg) in parameters.iter().enumerate() {
            let param_ty = param_types.get(i).unwrap();
            let arg_pair = self.gen_expression(arg)?;

            //  type cast
            let cast_ty = arg_pair.0.default_cast(&param_ty)?;
            let cast_v = self.cast_value(&arg_pair.0, &arg_pair.1, &cast_ty)?;

            compiled_args.push(cast_v);
        }

        let value = self.builder.build_call(llvm_func_opt.unwrap(),
                                            compiled_args.as_slice(),
                                            "").try_as_basic_value().left();


        // return type
        let ret_ty = func_ty_pair.0;

        if (ret_ty.is_some() && value.is_some()) || (ret_ty.is_none() && value.is_none()) {
            Ok((ret_ty, value))
        } else {
            let err = GNCErr::InvalidFunctionCall();
            return Err(err.into());
        }
    }

    // generate expressions
    fn gen_expression(&self, expression: &GNCAST)
                      -> Result<(GNCType, BasicValueEnum<'ctx>)> {
        match expression {
            GNCAST::Identifier(ref identifier) => {
//                dbg!("get variable pointer value");
                self.gen_deref_variable(identifier)
            }
            GNCAST::StringLiteral(ref s) => {
                Ok((GNCType::Pointer(Box::new(GNCType::Char)),
                    self.builder.build_global_string_ptr(s.as_str(), "str").as_basic_value_enum()))
            }
            GNCAST::BoolLiteral(ref bool_literal) => {
                Ok((GNCType::Bool, self.context.bool_type().const_int(*bool_literal as u64,
                                                                      false).as_basic_value_enum()))
            }
            GNCAST::IntLiteral(ref int_literal) => {
                let v = *int_literal;

                let ty = if v >= i8::MIN as i64 && v <= i8::MAX as i64 {
                    GNCType::Char
                } else if v >= i16::MIN as i64 && v <= i16::MAX as i64 {
                    GNCType::Short
                } else if v >= i32::MIN as i64 && v <= i32::MAX as i64 {
                    GNCType::Int
                } else {
                    GNCType::Long
                };
                Ok((ty.clone(), ty.to_basic_llvm_type(self.context).into_int_type()
                    .const_int(v as u64, true).as_basic_value_enum()))
            }
            GNCAST::FloatLiteral(ref float_literal) => {
                let v = *float_literal;

                let ty = if v >= f32::MIN as f64 && v <= f32::MAX as f64 {
                    GNCType::Float
                } else {
                    GNCType::Double
                };

                Ok((ty.clone(), ty.to_basic_llvm_type(self.context).into_float_type().const_float(v).as_basic_value_enum()))
            }
            GNCAST::UnaryExpression(ref op, ref expr) => {
                self.gen_unary_expression(op, expr)
            }
            GNCAST::BinaryExpression(ref op,
                                     ref lhs,
                                     ref rhs) => {
                self.gen_binary_expression(op, lhs, rhs)
            }
            GNCAST::FunctionCall(ref function_name,
                                 ref parameters) => {
                let call_pair = self.gen_function_call(function_name, parameters)?;

                if call_pair.0.is_some() && call_pair.1.is_some() {
                    Ok((call_pair.0.unwrap(), call_pair.1.unwrap()))
                } else {
                    let err = GNCErr::ReturnTypeMismatch();
                    return Err(err.into());
                }
            }
            GNCAST::CastExpression(ref cast_ty, ref expr) => {
//                dbg!("cast", cast_ty, expr);
                let cast_expr_pair = self.gen_expression(expr)?;
                let cast_v = self.cast_value(&cast_expr_pair.0,
                                             &cast_expr_pair.1,
                                             cast_ty)?;

                return Ok((cast_ty.to_owned(), cast_v));
            }
            _ => { return Err(GNCErr::UnknownExpression(expression.to_string()).into()); }
        }
    }


    // generate identifier and fetch value
    fn gen_deref_variable(&self, identifier: &String)
                          -> Result<(GNCType, BasicValueEnum<'ctx>)> {
        let deref = self.get_variable(identifier)?;

        let val = self.builder.build_load(deref.1, "load val");
        Ok((deref.0, val))
    }


    // get variable
    fn get_variable(&self, identifier: &String) -> Result<(GNCType, PointerValue<'ctx>)> {
        let mut lookup_rst = None;

        for map in self.addr_map_stack.iter().rev() {
            let rst = map.get(identifier);
            if rst.is_some() {
                lookup_rst = rst;
                break;
            }
        }

        if lookup_rst.is_none() {
            lookup_rst = self.global_variable_map.get(identifier);
        }

        if lookup_rst.is_none() {
            let err = GNCErr::MissingVariable(identifier.to_string());
            Err(err.into())
        } else {
            Ok(lookup_rst.unwrap().to_owned())
        }
    }


    // generate unary expressions
    fn gen_unary_expression(&self,
                            op: &UnaryOperator,
                            expr: &Box<GNCAST>)
                            -> Result<(GNCType, BasicValueEnum<'ctx>)> {
//        dbg!("generate unary expression", op, expr);

        // generate result
        let pair = self.gen_expression(expr)?;

        let ty = pair.0;
        let v = pair.1;

        return match op {
            UnaryOperator::UnaryMinus => {
                match ty {
                    GNCType::Bool | GNCType::Char | GNCType::Short | GNCType::Int | GNCType::Long => {
                        Ok((ty, self.builder.build_int_neg(
                            v.into_int_value(),
                            "int neg").as_basic_value_enum()))
                    }
                    GNCType::Float | GNCType::Double => {
                        Ok((ty, self.builder.build_float_neg(
                            v.into_float_value(),
                            "float neg").as_basic_value_enum(),
                        ))
                    }
                    _ => { Err(GNCErr::InvalidUnary().into()) }
                }
            }
            UnaryOperator::LogicalNot => {
                match ty {
                    GNCType::Bool | GNCType::Char | GNCType::Short | GNCType::Int | GNCType::Long => {
                        let res = self.builder.build_int_compare(
                            IntPredicate::EQ,
                            ty.to_basic_llvm_type(self.context).into_int_type()
                                .const_int(0 as u64, true),
                            v.into_int_value(), "int logical not",
                        );

                        let ret_ty = GNCType::Bool;

                        let res = self.builder.build_int_cast(
                            res,
                            ret_ty.to_basic_llvm_type(self.context).into_int_type(),
                            "logical not casting",
                        );
                        Ok((ret_ty, res.as_basic_value_enum()))
                    }
                    _ => { Err(GNCErr::InvalidUnary().into()) }
                }
            }
            UnaryOperator::BitwiseComplement => {
                match ty {
                    GNCType::Bool | GNCType::Char | GNCType::Short | GNCType::Int | GNCType::Long => {
                        Ok((ty, self.builder.build_not(
                            v.into_int_value(),
                            "not").as_basic_value_enum()))
                    }
                    _ => { Err(GNCErr::InvalidUnary().into()) }
                }
            }
            UnaryOperator::Dereference => {
                match ty {
                    GNCType::Pointer(ref ref_ty) => {
                        Ok((*ref_ty.clone(), self.builder.build_load(v.into_pointer_value(), "deref")))
                    }
                    _ => { Err(GNCErr::DereferenceNonPointer(ty).into()) }
                }
            }
            UnaryOperator::Reference => {
                match **expr {
                    GNCAST::Identifier(ref identifier) => {
                        let var_pair = self.get_variable(identifier)?;
                        Ok((GNCType::Pointer(Box::new(var_pair.0)), var_pair.1.as_basic_value_enum()))
                    }
                    _ => { Err(GNCErr::ReferenceNonVariable().into()) }
                }
            }
        };
    }

    // generate binary expression
    fn gen_binary_expression(&self,
                             op: &BinaryOperator,
                             lhs: &Box<GNCAST>,
                             rhs: &Box<GNCAST>)
                             -> Result<(GNCType, BasicValueEnum<'ctx>)> {
//        dbg!("binary expression", op, lhs, rhs);

        // generate (type, value) pair
        let lhs_pair = self.gen_expression(lhs)?;
        let rhs_pair = self.gen_expression(rhs)?;

        // types
        let lhs_ty = lhs_pair.0;
        let rhs_ty = rhs_pair.0;

        // default upcast
        let cast_ty = GNCType::binary_cast(&lhs_ty, &rhs_ty)?;

        let lhs_v = self.cast_value(&lhs_ty, &lhs_pair.1, &cast_ty)?;
        let rhs_v = self.cast_value(&rhs_ty, &rhs_pair.1, &cast_ty)?;

        match cast_ty {
            GNCType::Bool | GNCType::Char | GNCType::UChar | GNCType::Short | GNCType::UShort |
            GNCType::Int | GNCType::UInt | GNCType::Long | GNCType::ULong => {
                let int_lhs_v = lhs_v.into_int_value();
                let int_rhs_v = rhs_v.into_int_value();

                let int_val = match op {
                    BinaryOperator::Add => self.builder.build_int_add(int_lhs_v, int_rhs_v, "int add"),
                    BinaryOperator::Subtract => self.builder.build_int_sub(int_lhs_v, int_rhs_v, "int sub"),
                    BinaryOperator::Multiply => self.builder.build_int_mul(int_lhs_v, int_rhs_v, "int mul"),
                    BinaryOperator::Divide => self.builder.build_int_signed_div(int_lhs_v, int_rhs_v, "i32 signed div"),
                    BinaryOperator::Modulus => self.builder.build_int_signed_rem(int_lhs_v, int_rhs_v, "mod"),
                    BinaryOperator::ShiftRight => self.builder.build_right_shift(int_lhs_v, int_rhs_v, true, "shr"),
                    BinaryOperator::ShiftLeft => self.builder.build_left_shift(int_lhs_v, int_rhs_v, "shl"),
                    BinaryOperator::NotEqual => self.builder.build_int_compare(IntPredicate::NE, int_lhs_v, int_rhs_v, "ne"),
                    BinaryOperator::Equal => self.builder.build_int_compare(IntPredicate::EQ, int_lhs_v, int_rhs_v, "eq"),
                    BinaryOperator::GreaterThan => self.builder.build_int_compare(IntPredicate::SGT, int_lhs_v, int_rhs_v, "gt"),
                    BinaryOperator::GreaterEqual => self.builder.build_int_compare(IntPredicate::SGE, int_lhs_v, int_rhs_v, "ge"),
                    BinaryOperator::LessThan => self.builder.build_int_compare(IntPredicate::SLT, int_lhs_v, int_rhs_v, "lt"),
                    BinaryOperator::LessEqual => self.builder.build_int_compare(IntPredicate::SLE, int_lhs_v, int_rhs_v, "le"),
                    BinaryOperator::BitwiseAnd => self.builder.build_and(int_lhs_v, int_rhs_v, "and"),
                    BinaryOperator::ExclusiveOr => self.builder.build_xor(int_lhs_v, int_rhs_v, "xor"),
                    BinaryOperator::InclusiveOr => self.builder.build_or(int_lhs_v, int_rhs_v, "or"),
                    BinaryOperator::LogicalAnd => self.builder.build_and(
                        self.builder.build_int_cast(int_lhs_v, self.context.bool_type(), "cast i32 to i1"),
                        self.builder.build_int_cast(int_rhs_v, self.context.bool_type(), "cast i32 to i1"),
                        "logical and",
                    ),
                    BinaryOperator::LogicalOr => self.builder.build_or(
                        self.builder.build_int_cast(int_lhs_v, self.context.bool_type(), "cast i32 to i1"),
                        self.builder.build_int_cast(int_rhs_v, self.context.bool_type(), "cast i32 to i1"),
                        "logical or",
                    ),
                    BinaryOperator::FetchRHS => int_rhs_v
                };
                Ok((cast_ty.to_owned(), int_val.as_basic_value_enum()))
            }
            GNCType::Float | GNCType::Double => {
                let fp_lhs_v = lhs_v.into_float_value();
                let fp_rhs_v = rhs_v.into_float_value();

                if op.is_compare() {
                    let cmp_val = match op {
                        BinaryOperator::Equal => Some(self.builder
                            .build_float_compare(FloatPredicate::OEQ,
                                                 fp_lhs_v, fp_rhs_v, "fp eq")),
                        BinaryOperator::NotEqual => Some(self.builder
                            .build_float_compare(FloatPredicate::ONE,
                                                 fp_lhs_v, fp_rhs_v, "fp ne")),
                        BinaryOperator::LessThan => Some(self.builder
                            .build_float_compare(FloatPredicate::OLT,
                                                 fp_lhs_v, fp_rhs_v, "fp lt")),
                        BinaryOperator::LessEqual => Some(self.builder
                            .build_float_compare(FloatPredicate::OLE,
                                                 fp_lhs_v, fp_rhs_v, "fp lt")),
                        BinaryOperator::GreaterThan => Some(self.builder
                            .build_float_compare(FloatPredicate::OGT,
                                                 fp_lhs_v, fp_rhs_v, "fp lt")),
                        BinaryOperator::GreaterEqual => Some(self.builder
                            .build_float_compare(FloatPredicate::OGE,
                                                 fp_lhs_v, fp_rhs_v, "fp lt")),
                        _ => { None }
                    };

                    return Ok((GNCType::Bool, cmp_val.unwrap().as_basic_value_enum()));
                } else {
                    let fp_val = match op {
                        BinaryOperator::Add => Some(self.builder
                            .build_float_add(fp_lhs_v, fp_rhs_v, "fp add")),
                        BinaryOperator::Subtract => Some(self.builder
                            .build_float_sub(fp_lhs_v, fp_rhs_v, "fp sub")),
                        BinaryOperator::Multiply => Some(self.builder
                            .build_float_mul(fp_lhs_v, fp_rhs_v, "fp mul")),
                        BinaryOperator::Divide => Some(self.builder
                            .build_float_div(fp_lhs_v, fp_rhs_v, "fp div")),
                        BinaryOperator::Modulus => Some(self.builder
                            .build_float_rem(fp_lhs_v, fp_rhs_v, "fp mod")),
                        BinaryOperator::FetchRHS => Some(fp_rhs_v),
                        _ => { None }
                    };

                    if fp_val.is_some() {
                        Ok((cast_ty.to_owned(), fp_val.unwrap().as_basic_value_enum()))
                    } else {
                        return Err(GNCErr::InvalidOperation(cast_ty, op.clone()).into());
                    }
                }
            }
            GNCType::Pointer(_) => unsafe {
                if op.is_compare() && lhs_ty.is_ptr_ty() && rhs_ty.is_ptr_ty() {
                    let lhs_ptr_v = lhs_v.into_pointer_value();
                    let rhs_ptr_v = rhs_v.into_pointer_value();

                    let ptr_diff = self.builder.build_ptr_diff(
                        lhs_ptr_v, rhs_ptr_v, "ptr diff",
                    );

                    let cmp_val = match op {
                        BinaryOperator::NotEqual => self.builder.build_int_compare(
                            IntPredicate::NE, ptr_diff,
                            self.context.i64_type().const_zero(), "ptr ne",
                        ),
                        BinaryOperator::Equal => self.builder.build_int_compare(
                            IntPredicate::EQ, ptr_diff,
                            self.context.i64_type().const_zero(), "ptr eq",
                        ),
                        _ => { return Err(GNCErr::InvalidOperation(cast_ty, op.clone()).into()); }
                    };

                    let cmp_val = self.builder.build_cast(
                        InstructionOpcode::Trunc, cmp_val,
                        self.context.bool_type(), "ptr cmp cast",
                    );

                    return Ok((GNCType::Bool, cmp_val));
                } else if *op == BinaryOperator::FetchRHS {
                    Ok((cast_ty, rhs_v))
                } else if *op == BinaryOperator::Add {
                    let val_pair = if lhs_ty.is_ptr_ty() && rhs_ty.is_int_ty() {
                        (lhs_v, rhs_pair.1)
                    } else if lhs_ty.is_int_ty() && rhs_ty.is_ptr_ty() {
                        (rhs_v, lhs_pair.1)
                    } else {
                        return Err(GNCErr::InvalidOperation(cast_ty, op.clone()).into());
                    };

                    let idx = vec![val_pair.1.into_int_value()];

                    Ok((cast_ty, self.builder.build_gep(
                        val_pair.0.into_pointer_value(), idx.as_ref(), "add gep",
                    ).as_basic_value_enum()))
                } else if *op == BinaryOperator::Subtract && lhs_ty.is_ptr_ty() && rhs_ty.is_int_ty() {
                    let idx = vec![self.builder.build_int_neg(rhs_v.into_int_value(), "int neg")];

                    Ok((cast_ty, self.builder.build_gep(
                        lhs_v.into_pointer_value(), idx.as_ref(), "sub gep",
                    ).as_basic_value_enum()))
                } else {
                    return Err(GNCErr::InvalidOperation(cast_ty, op.clone()).into());
                }
            }
            _ => { return Err(GNCErr::InvalidOperation(cast_ty, op.clone()).into()); }
        }
    }

    // get left value
    fn get_lvalue(&self, lhs: &Box<GNCAST>) -> Result<(GNCType, PointerValue<'ctx>)> {
//        dbg!("get left value", lhs);

        return match **lhs {
            GNCAST::Identifier(ref identifier) => self.get_variable(identifier),
            GNCAST::UnaryExpression(ref op, ref expr) => {
                match op {
                    UnaryOperator::Dereference => {
                        let addr_pair = self.gen_expression(expr)?;
//                        dbg!(addr_pair.clone());
                        Ok((addr_pair.0.deref_ptr()?, addr_pair.1.into_pointer_value()))
                    }
                    _ => { Err(GNCErr::InvalidLeftValue().into()) }
                }
            }
            _ => { Err(GNCErr::InvalidLeftValue().into()) }
        };
    }


//>>>>>>>>>>>>>>>>>>>>>>>
//      Some utils
//<<<<<<<<<<<<<<<<<<<<<<<


    // check if a basic block has no terminator
    fn no_terminator(&self) -> bool {
        let block = self.builder.get_insert_block();
        let terminator = block.unwrap().get_terminator();
        return terminator.is_none();
    }

    // add void type as return type
    fn to_return_type(&self, in_type: &GNCType,
                      param_types: &Vec<BasicTypeEnum<'ctx>>) -> Result<FunctionType<'ctx>> {
        match in_type {
            GNCType::Void => Ok(self.context.void_type().fn_type(param_types, false)),
            _ => {
                let basic_type = in_type.to_basic_llvm_type(self.context);
                Ok(basic_type.fn_type(param_types, false))
            }
        }
    }


    fn cast_value(&self,
                  cur_ty: &GNCType,
                  cur_val: &BasicValueEnum<'ctx>,
                  cast_ty: &GNCType,
    ) -> Result<BasicValueEnum<'ctx>> {
        if cur_ty == cast_ty {
            return Ok(*cur_val);
        }

        // TODO add bool type compare
        Ok(self.builder.build_cast(
            self.cast_inst(cur_ty, cast_ty)?, *cur_val,
            cast_ty.to_basic_llvm_type(self.context), "cast",
        ))
    }


    fn cast_inst(&self, cur_ty: &GNCType, cast_ty: &GNCType) -> Result<InstructionOpcode> {
        let inst = match cur_ty {
            GNCType::Float => match cast_ty {
                GNCType::Char | GNCType::Short | GNCType::Int | GNCType::Long => InstructionOpcode::FPToSI,
                GNCType::UChar | GNCType::UShort | GNCType::UInt | GNCType::ULong => InstructionOpcode::FPToUI,
                GNCType::Double => InstructionOpcode::FPExt,
                _ => { return Err(GNCErr::InvalidCast(cur_ty.clone(), cast_ty.clone()).into()); }
            }
            GNCType::Double => match cast_ty {
                GNCType::Char | GNCType::Short | GNCType::Int | GNCType::Long => InstructionOpcode::FPToSI,
                GNCType::UChar | GNCType::UShort | GNCType::UInt | GNCType::ULong => InstructionOpcode::FPToUI,
                GNCType::Float => InstructionOpcode::FPTrunc,
                _ => { return Err(GNCErr::InvalidCast(cur_ty.clone(), cast_ty.clone()).into()); }
            }
            GNCType::Bool => match cast_ty {
                GNCType::Char | GNCType::UChar | GNCType::Short | GNCType::UShort | GNCType::Int | GNCType::UInt |
                GNCType::Long | GNCType::ULong => InstructionOpcode::ZExt,
                GNCType::Float | GNCType::Double => InstructionOpcode::UIToFP,
                _ => { return Err(GNCErr::InvalidCast(cur_ty.clone(), cast_ty.clone()).into()); }
            }
            GNCType::Char => match cast_ty {
                GNCType::Short | GNCType::Int | GNCType::Long => InstructionOpcode::SExt,
                GNCType::UChar | GNCType::UShort | GNCType::UInt | GNCType::ULong => InstructionOpcode::ZExt,
                GNCType::Float | GNCType::Double => InstructionOpcode::SIToFP,
                GNCType::Pointer(_) => InstructionOpcode::IntToPtr,
                _ => { return Err(GNCErr::InvalidCast(cur_ty.clone(), cast_ty.clone()).into()); }
            }
            GNCType::UChar => match cast_ty {
                GNCType::Char => InstructionOpcode::BitCast,
                GNCType::Short | GNCType::Int | GNCType::Long | GNCType::UShort | GNCType::UInt | GNCType::ULong =>
                    InstructionOpcode::ZExt,
                GNCType::Float | GNCType::Double => InstructionOpcode::UIToFP,
                GNCType::Pointer(_) => InstructionOpcode::IntToPtr,
                _ => { return Err(GNCErr::InvalidCast(cur_ty.clone(), cast_ty.clone()).into()); }
            }
            GNCType::Short => match cast_ty {
                GNCType::Char | GNCType::UChar => InstructionOpcode::Trunc,
                GNCType::Int | GNCType::Long => InstructionOpcode::SExt,
                GNCType::UShort => InstructionOpcode::BitCast,
                GNCType::UInt | GNCType::ULong => InstructionOpcode::ZExt,
                GNCType::Float | GNCType::Double => InstructionOpcode::SIToFP,
                GNCType::Pointer(_) => InstructionOpcode::IntToPtr,
                _ => { return Err(GNCErr::InvalidCast(cur_ty.clone(), cast_ty.clone()).into()); }
            }
            GNCType::UShort => match cast_ty {
                GNCType::UShort => InstructionOpcode::BitCast,
                GNCType::Char | GNCType::UChar => InstructionOpcode::Trunc,
                GNCType::Int | GNCType::Long | GNCType::UInt | GNCType::ULong => InstructionOpcode::ZExt,
                GNCType::Float | GNCType::Double => InstructionOpcode::UIToFP,
                GNCType::Pointer(_) => InstructionOpcode::IntToPtr,
                _ => { return Err(GNCErr::InvalidCast(cur_ty.clone(), cast_ty.clone()).into()); }
            }
            GNCType::Int => match cast_ty {
                GNCType::Char | GNCType::UChar | GNCType::Short | GNCType::UShort => InstructionOpcode::Trunc,
                GNCType::UInt => InstructionOpcode::BitCast,
                GNCType::ULong => InstructionOpcode::ZExt,
                GNCType::Long => InstructionOpcode::SExt,
                GNCType::Float | GNCType::Double => InstructionOpcode::SIToFP,
                GNCType::Pointer(_) => InstructionOpcode::IntToPtr,
                _ => { return Err(GNCErr::InvalidCast(cur_ty.clone(), cast_ty.clone()).into()); }
            }
            GNCType::UInt => match cast_ty {
                GNCType::Int => InstructionOpcode::BitCast,
                GNCType::Char | GNCType::UChar | GNCType::Short | GNCType::UShort => InstructionOpcode::Trunc,
                GNCType::Long | GNCType::ULong => InstructionOpcode::ZExt,
                GNCType::Float | GNCType::Double => InstructionOpcode::UIToFP,
                GNCType::Pointer(_) => InstructionOpcode::IntToPtr,
                _ => { return Err(GNCErr::InvalidCast(cur_ty.clone(), cast_ty.clone()).into()); }
            }
            GNCType::Long => match cast_ty {
                GNCType::Char | GNCType::UChar | GNCType::Short | GNCType::UShort | GNCType::UInt | GNCType::Int =>
                    InstructionOpcode::Trunc,
                GNCType::ULong => InstructionOpcode::BitCast,
                GNCType::Float | GNCType::Double => InstructionOpcode::SIToFP,
                GNCType::Pointer(_) => InstructionOpcode::IntToPtr,
                _ => { return Err(GNCErr::InvalidCast(cur_ty.clone(), cast_ty.clone()).into()); }
            }
            GNCType::ULong => match cast_ty {
                GNCType::Long => InstructionOpcode::BitCast,
                GNCType::Char | GNCType::UChar | GNCType::Short | GNCType::UShort | GNCType::UInt | GNCType::Int =>
                    InstructionOpcode::Trunc,
                GNCType::Float | GNCType::Double => InstructionOpcode::UIToFP,
                GNCType::Pointer(_) => InstructionOpcode::IntToPtr,
                _ => { return Err(GNCErr::InvalidCast(cur_ty.clone(), cast_ty.clone()).into()); }
            }
            GNCType::Pointer(_) => match cast_ty {
                GNCType::Char | GNCType::UChar | GNCType::Short | GNCType::UShort | GNCType::UInt | GNCType::Int |
                GNCType::Long | GNCType::ULong => InstructionOpcode::PtrToInt,
                _ => { return Err(GNCErr::InvalidCast(cur_ty.clone(), cast_ty.clone()).into()); }
            }
            _ => { return Err(GNCErr::InvalidCast(cur_ty.clone(), cast_ty.clone()).into()); }
        };
        Ok(inst)
    }
}