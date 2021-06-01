use inkwell::context::Context;
use inkwell::module::Module;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, VecDeque, HashSet};
use parser::{GNCAST, UnaryOperator, BinaryOperator, AssignOperation, GNCParameter};
use inkwell::targets::{Target, InitializationConfig, TargetMachine, RelocMode, CodeModel, FileType};
use inkwell::{IntPredicate, FloatPredicate};
use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::values::{PointerValue, FunctionValue, BasicValue, BasicValueEnum};
use inkwell::basic_block::BasicBlock;
use inkwell::types::{BasicTypeEnum, BasicType, FunctionType};
use checker::{GNCErr};
use anyhow::Result;
use types::{GNCType, Type};


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
    addr_map_stack: Vec<HashMap<String, (Type<'ctx>, PointerValue<'ctx>)>>,

    //>>>>>>>>>>>>>>>>>>>>>>>>
    //      LLVM Blocks
    //<<<<<<<<<<<<<<<<<<<<<<<<

    // current function block
    current_function: Option<FunctionValue<'ctx>>,
    // break labels (in loop statements)
    break_labels: VecDeque<BasicBlock<'ctx>>,
    // continue labels (in loop statements)
    continue_labels: VecDeque<BasicBlock<'ctx>>,
    // hashset for functions
    function_map: HashMap<String, Option<Type<'ctx>>>,
    // hashset for global variable
    global_variable_map: HashMap<String, (Type<'ctx>, PointerValue<'ctx>)>,
}

impl<'ctx> CodeGen<'ctx> {
    // new LLVM context
    pub fn new(context: &'ctx Context, source_path: &'ctx str) -> CodeGen<'ctx> {
        let module_name = Path::new(source_path).file_stem().unwrap().to_str().unwrap().to_string();
        let module = context.create_module(module_name.as_str());
        let builder = context.create_builder();

        // set variable scope
        let mut addr_map_stack = Vec::new();
        let global_map: HashMap<String, (Type<'ctx>, PointerValue<'ctx>)> = HashMap::new();
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
        if self.global_variable_map.contains_key(var_name) {
            let err = GNCErr::DuplicateGlobalVar(var_name.to_string());
            return Err(err.into());
        }

        let ty = self.to_basic_type(var_type)?;

        let global_value = self.module.add_global(ty.llvm_ty,
                                                  None,
                                                  var_name.as_str());

        // TODO add const_val check
        // TODO add type cast
        let init_val = self.gen_expression(&**ptr_to_init)?;

        global_value.set_initializer(&(init_val.1));

        self.global_variable_map.insert(var_name.to_string(),
                                        (ty, global_value.as_pointer_value()));
        Ok(())
    }


    // generate function proto
    fn gen_function_proto(&mut self,
                          ret_type: &GNCType,
                          func_name: &String,
                          func_param: &Vec<GNCParameter>) -> Result<()> {
        println!("[DEBUG] generate function protocol");

        // cannot handle duplicate function
        if self.function_map.contains_key(func_name) {
            let err = GNCErr::DuplicateFunction(func_name.to_string());

            return Err(err.into());
        }

        // function parameter should be added in this llvm_func_type
        let mut param_types: Vec<BasicTypeEnum<'ctx>> = Vec::new();
        for param in func_param {
            let ty = self.to_basic_type(&param.param_type)?;
            param_types.push(ty.llvm_ty);
        }

        let llvm_func_ty = self.to_return_type(ret_type, &param_types)?;

        // create function
        self.module.add_function(func_name.as_str(), llvm_func_ty, None);

        let func_ty = if *ret_type != GNCType::Void {
            Some(self.to_basic_type(ret_type)?)
        } else {
            None
        };

        self.function_map.insert(func_name.to_owned(), func_ty);
        Ok(())
    }


    fn gen_function_def(&mut self,
                        func_type: &GNCType,
                        func_name: &String,
                        func_param: &Vec<GNCParameter>,
                        func_body: &Vec<GNCAST>) -> Result<()> {
        println!("[DEBUG] generate function definition");
        // push local map
        let local_map: HashMap<String, (Type<'ctx>, PointerValue<'ctx>)> = HashMap::new();
        self.addr_map_stack.push(local_map);

        let func_option = self.module.get_function(func_name.as_str());
        if func_option.is_none() {
            panic!();
        }
        let func = func_option.unwrap();
        self.current_function = Some(func);

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

            let ty = self.to_basic_type(&func_param[i].param_type)?;

            // alloc variable on stack
            let alloca = builder.build_alloca(
                ty.llvm_ty,
                &arg_name,
            );
            self.gen_variable(ty, &arg_name.to_string(), alloca);
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
                    var_type: Type<'ctx>,
                    identifier: &String,
                    ptr: PointerValue<'ctx>) {
        match self.addr_map_stack.last_mut() {
            Some(map) => {
                map.insert(identifier.to_string(), (var_type, ptr));
            }
            _ => { panic!(identifier.to_string() + " not found. Addr HashMap Stack overflow"); }
        }
    }

    fn gen_statement(&mut self, statement: &GNCAST) -> Result<()> {
        // println!("in gen_statement {:?}", statement);
        match statement {
            GNCAST::ReturnStatement(ref ptr_to_expr) => {
                println!("[DEBUG] generate return statement");

                if ptr_to_expr.is_some() {
                    let func = self.current_function.unwrap();

                    let ty_opt = func.get_type().get_return_type();

                    // return type mismatch
                    if ty_opt.is_none() {
                        return Err(GNCErr::ReturnTypeMismatch().into());
                    }

                    // TODO add type cast

                    let ty = ty_opt.unwrap();

                    let expr = ptr_to_expr.as_ref().as_ref().unwrap();
                    let expr_val = self.gen_expression(&expr)?;

                    self.builder.build_return(Some(&expr_val.1));
                } else {
                    self.builder.build_return(None);
                }
            }
            GNCAST::Declaration(ref data_type, ref identifier) => {
                println!("[DEBUG] generate declaration");
                let ty = self.to_basic_type(data_type)?;

                let point_value = self.builder.build_alloca(ty.llvm_ty, identifier);
                self.gen_variable(ty, identifier, point_value);
            }
            GNCAST::FunctionCall(ref function_name,
                                 ref parameters) => {
                self.gen_function_call(function_name, parameters)?;
            }
            GNCAST::Assignment(ref op,
                               ref identifier,
                               ref expr) => {
                println!("[DEBUG] generate assignment");
                let ptr = self.get_variable(identifier)?;

                let val = self.gen_binary_expression(
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
                    },
                    &Box::new(GNCAST::Identifier(identifier.to_owned())),
                    expr,
                )?;

                // TODO add type cast
                // check variable and value type

                self.builder.build_store(ptr.1, val.1);
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
        let func = self.current_function.unwrap();

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
        let func = self.current_function.unwrap();

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
        println!("[DEBUG] generate {} statement", if is_do_while { "do_while" } else { "while" });

        let func = self.current_function.unwrap();

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
        let local_map: HashMap<String, (Type<'ctx>, PointerValue<'ctx>)>
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
                         parameters: &Vec<GNCAST>) -> Result<(Option<Type<'ctx>>,
                                                              Option<BasicValueEnum<'ctx>>)> {
        println!("[DEBUG] generate function call");

        // get function and return type
        let ret_ty_opt = self.function_map.get(function_name);
        let func_option = self.module.get_function(function_name);

        // handle not found error
        if ret_ty_opt.is_none() || func_option.is_none() {
            let err = GNCErr::MissingFunction(function_name.to_string());
            return Err(err.into());
        }

        let func = func_option.unwrap();
        let func_param_count = func.get_type().count_param_types();

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


        for (i, arg) in parameters.iter().enumerate() {
            let func_param = func.get_nth_param(i as u32).unwrap().get_type();

            let arg_val = self.gen_expression(arg)?;


            compiled_args.push(arg_val.1);
        }

        let value = self.builder.build_call(func_option.unwrap(),
                                            compiled_args.as_slice(),
                                            "").try_as_basic_value().left();


        let ret_ty = *ret_ty_opt.unwrap();

        // TODO fix function call

        if (ret_ty.is_some() && value.is_some()) || (ret_ty.is_none() && value.is_none()) {
            Ok((ret_ty, value))
        } else {
            let err = GNCErr::InvalidFunctionCall();
            return Err(err.into());
        }
    }

    // generate expressions
    fn gen_expression(&self, expression: &GNCAST)
                      -> Result<(Type<'ctx>, BasicValueEnum<'ctx>)> {
        match expression {
            GNCAST::Identifier(ref identifier) => {
                self.gen_deref_variable(identifier)
            }
            GNCAST::BoolLiteral(ref bool_literal) => {
                Ok((Type {
                    ty: GNCType::Bool,
                    llvm_ty: self.context.bool_type().as_basic_type_enum(),
                }, self.context.bool_type().const_int(*bool_literal as u64,
                                                      false).as_basic_value_enum()))
            }
            GNCAST::IntLiteral(ref int_literal) => {
                let v = *int_literal;

                let ty = if v == 0 || v == 1 {
                    Type {
                        ty: GNCType::Bool,
                        llvm_ty: self.context.bool_type().as_basic_type_enum(),
                    }
                } else if v >= i8::MIN as i64 && v <= i8::MAX as i64 {
                    Type {
                        ty: GNCType::Byte,
                        llvm_ty: self.context.i8_type().as_basic_type_enum(),
                    }
                } else if v >= i16::MIN as i64 && v <= i16::MAX as i64 {
                    Type {
                        ty: GNCType::Short,
                        llvm_ty: self.context.i16_type().as_basic_type_enum(),
                    }
                } else if v >= i32::MIN as i64 && v <= i32::MAX as i64 {
                    Type {
                        ty: GNCType::Int,
                        llvm_ty: self.context.i32_type().as_basic_type_enum(),
                    }
                } else {
                    Type {
                        ty: GNCType::Long,
                        llvm_ty: self.context.i64_type().as_basic_type_enum(),
                    }
                };
                Ok((ty, ty.llvm_ty.into_int_type()
                    .const_int(v as u64, true).as_basic_value_enum()))
            }
            GNCAST::FloatLiteral(ref float_literal) => {
                let v = *float_literal;

                let ty = if v >= f32::MIN as f64 && v <= f32::MAX as f64 {
                    Type {
                        ty: GNCType::Float,
                        llvm_ty: self.context.f32_type().as_basic_type_enum(),
                    }
                } else {
                    Type {
                        ty: GNCType::Double,
                        llvm_ty: self.context.f64_type().as_basic_type_enum(),
                    }
                };

                Ok((ty, ty.llvm_ty.into_float_type().const_float(v).as_basic_value_enum()))
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
                let call_opt = self.gen_function_call(function_name, parameters)?;

                if call_opt.0.is_some() && call_opt.1.is_some() {
                    Ok((call_opt.0.unwrap(), call_opt.1.unwrap()))
                } else {
                    let err = GNCErr::ReturnTypeMismatch();
                    return Err(err.into());
                }
            }
            _ => { panic!("Invalid Expression Type") }
        }
    }


    // generate identifier and fetch value
    fn gen_deref_variable(&self, identifier: &String)
                          -> Result<(Type<'ctx>, BasicValueEnum<'ctx>)> {
        let deref = self.get_variable(identifier)?;

        let val = self.builder.build_load(deref.1, "load val");
        Ok((deref.0, val))
    }


    fn get_variable(&self, identifier: &String) -> Result<(Type<'ctx>, PointerValue<'ctx>)> {
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
            Ok(*(lookup_rst.unwrap()))
        }
    }


    // generate unary expressions
    fn gen_unary_expression(&self,
                            op: &UnaryOperator,
                            expr: &Box<GNCAST>)
                            -> Result<(Type<'ctx>, BasicValueEnum<'ctx>)> {
        println!("[DEBUG] generate unary expression");

        // generate result
        let pair = self.gen_expression(expr)?;

        let ty = pair.0;
        let v = pair.1;

        return match op {
            UnaryOperator::UnaryMinus => {
                match ty.ty {
                    GNCType::Bool |
                    GNCType::Byte |
                    GNCType::Short |
                    GNCType::Int |
                    GNCType::Long => {
                        Ok((ty, self.builder.build_int_neg(
                            v.into_int_value(),
                            "int neg").as_basic_value_enum()))
                    }
                    GNCType::Float |
                    GNCType::Double => {
                        Ok((ty, self.builder.build_float_neg(
                            v.into_float_value(),
                            "float neg").as_basic_value_enum(),
                        ))
                    }
                    _ => { Err(GNCErr::InvalidUnary().into()) }
                }
            }
            UnaryOperator::LogicalNot => {
                match ty.ty {
                    GNCType::Bool |
                    GNCType::Byte |
                    GNCType::Short |
                    GNCType::Int |
                    GNCType::Long => {
                        let res = self.builder.build_int_compare(
                            IntPredicate::EQ,
                            ty.llvm_ty.into_int_type().const_int(0 as u64, true),
                            v.into_int_value(), "int logical not");

                        let ret_ty = Type {
                            ty: GNCType::Bool,
                            llvm_ty: self.context.bool_type().as_basic_type_enum(),
                        };

                        let res = self.builder.build_int_cast(res,
                                                              ret_ty.llvm_ty.into_int_type(),
                                                              "logical not casting");
                        Ok((ret_ty, res.as_basic_value_enum()))
                    }
                    _ => { Err(GNCErr::InvalidUnary().into()) }
                }
            }
            UnaryOperator::BitwiseComplement => {
                match ty.ty {
                    GNCType::Bool |
                    GNCType::Byte |
                    GNCType::Short |
                    GNCType::Int |
                    GNCType::Long => {
                        Ok((ty, self.builder.build_not(
                            v.into_int_value(),
                            "not").as_basic_value_enum()))
                    }
                    _ => { Err(GNCErr::InvalidUnary().into()) }
                }
            }
        };
    }

    // generate binary expression
    fn gen_binary_expression(&self,
                             op: &BinaryOperator,
                             lhs: &Box<GNCAST>,
                             rhs: &Box<GNCAST>)
                             -> Result<(Type<'ctx>, BasicValueEnum<'ctx>)> {
        println!("[DEBUG] generate binary expression");

        // generate (type, value) pair
        let lhs_pair = self.gen_expression(lhs)?;
        let rhs_pair = self.gen_expression(rhs)?;

        // lhs (type, value)
        let lhs_ty = lhs_pair.0;
        let lhs_v = lhs_pair.1;

        // rhs (type, value)
        let rhs_ty = rhs_pair.0;
        let rhs_v = rhs_pair.1;

        // TODO default upcast
        
        match rhs_ty {
            Type::IntType(_) => {
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
                Ok((rhs_ty, int_val.as_basic_value_enum()))
            }
            Type::FloatType(_) => {
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
                        _ => { panic!() }
                    };

                    return Ok((Type {
                        ty: GNCType::Bool,
                        llvm_ty: self.context.bool_type().as_basic_type_enum(),
                    }, cmp_val.unwrap().as_basic_value_enum()));
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
                        Ok((rhs_ty, fp_val.unwrap().as_basic_value_enum()))
                    } else {
                        let err = GNCErr::InvalidFloatingPointOperation();
                        return Err(err.into());
                    }
                }
            }
            _ => { panic!() }
        }
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

    fn to_basic_type(&self, in_type: &GNCType)
                     -> Result<Type<'ctx>> {
        let basic_ty = match in_type {
            GNCType::Bool => self.context.bool_type().as_basic_type_enum(),
            GNCType::Byte => self.context.i8_type().as_basic_type_enum(),
            GNCType::UnsignedByte => self.context.i8_type().as_basic_type_enum(),
            GNCType::Short => self.context.i16_type().as_basic_type_enum(),
            GNCType::UnsignedShort => self.context.i16_type().as_basic_type_enum(),
            GNCType::Int => self.context.i32_type().as_basic_type_enum(),
            GNCType::UnsignedInt => self.context.i32_type().as_basic_type_enum(),
            GNCType::Long => self.context.i64_type().as_basic_type_enum(),
            GNCType::UnsignedLong => self.context.i64_type().as_basic_type_enum(),
            GNCType::Float => self.context.f32_type().as_basic_type_enum(),
            GNCType::Double => self.context.f64_type().as_basic_type_enum(),
            _ => {
                let err = GNCErr::InvalidType(*in_type);
                return Err(err.into());
            }
        };

        return Ok(Type {
            ty: *in_type,
            llvm_ty: basic_ty,
        });
    }


    // add void type as return type
    fn to_return_type(&self,
                      in_type: &GNCType,
                      param_types: &Vec<BasicTypeEnum<'ctx>>) -> Result<FunctionType<'ctx>> {
        match in_type {
            GNCType::Void => Ok(self.context.void_type().fn_type(param_types, false)),
            _ => {
                let basic_type = self.to_basic_type(in_type)?;
                Ok(basic_type.fn_type(param_types, false))
            }
        }
    }
}
