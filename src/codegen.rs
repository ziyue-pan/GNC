use inkwell::context::Context;
use inkwell::module::Module;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, VecDeque};
use parser::{GNCAST, GNCType, UnaryOperator, BinaryOperator, AssignOperation, GNCParameter};
use inkwell::targets::{Target, InitializationConfig, TargetMachine, RelocMode, CodeModel, FileType};
use inkwell::{IntPredicate};
use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::values::{PointerValue, IntValue, FunctionValue};
use inkwell::basic_block::BasicBlock;


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
    addr_map_stack: Vec<HashMap<String, PointerValue<'ctx>>>,

    //>>>>>>>>>>>>>>>>>>>>>>>>
    //      LLVM Blocks
    //<<<<<<<<<<<<<<<<<<<<<<<<

    // current function block
    current_function: Option<FunctionValue<'ctx>>,
    // break labels (in loop statements)
    break_labels: VecDeque<BasicBlock<'ctx>>,
    // continue labels (in loop statements)
    continue_labels: VecDeque<BasicBlock<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    // new LLVM context
    pub fn new(context: &'ctx Context, source_path: &'ctx str) -> CodeGen<'ctx> {
        let module_name = Path::new(source_path).file_stem().unwrap().to_str().unwrap().to_string();
        let module = context.create_module(module_name.as_str());
        let builder = context.create_builder();

        // set variable scope
        let mut addr_map_stack = Vec::new();
        let global_map: HashMap<String, PointerValue> = HashMap::new();
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
        }
    }

    // generate all code
    pub fn gen(&mut self, ast: &Vec<GNCAST>) {
        for node in ast {
            match node {
                GNCAST::Function(ref func_type,
                                 ref func_name,
                                 ref func_param,
                                 ref func_body) => {
                    self.gen_function(func_type, func_name, func_param, func_body);
                }
                // TODO Update global hashmap: addr_map_stack[addr_map_stack.len() - 1].insert(identifier, PointerValue);
                _ => { panic!(); }
            }
        }

        // set llvm bitcode path
        let mut llvm_bitcode_path = PathBuf::from(self.source_path);
        llvm_bitcode_path.set_extension("bc");
        self.module.write_bitcode_to_path(llvm_bitcode_path.as_path());

        // set llvm target
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

        // write assembly code
        let mut target_assembly_path = PathBuf::from(self.source_path);
        target_assembly_path.set_extension("asm");
        machine.write_to_file(&self.module, FileType::Assembly, target_assembly_path.as_ref()).unwrap();
    }


    fn gen_function(&mut self,
                    func_type: &GNCType,
                    func_name: &String,
                    func_param: &Vec<GNCParameter>,
                    func_body: &Vec<GNCAST>) {

        // TODO add function parameter
        // function parameter should be added in this llvm_func_type
        let llvm_func_type = match func_type {
            GNCType::Int => { self.context.i32_type().fn_type(&[], false) }
            _ => { self.context.i32_type().fn_type(&[], false) }
        };

        // push local map
        let local_map: HashMap<String, PointerValue> = HashMap::new();
        self.addr_map_stack.push(local_map);

        // create function
        let func = self.module.add_function(func_name.as_str(), llvm_func_type, None);
        self.current_function = Some(func);

        // create function block
        let func_block = self.context.append_basic_block(func, "entry");
        self.builder.position_at_end(func_block);

        // generate IR for statements inside the function body
        for statement in func_body {
            self.gen_statement(statement);
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
        self.current_function = None
    }


    //>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    fn get_point_value(&self, identifier: &String) -> PointerValue {
        for map in self.addr_map_stack.iter().rev() {
            match map.get(identifier) {
                Some(addr) => { return *addr; }
                _ => {}
            }
        }
        panic!(identifier.to_string() + " not found!");
    }

    fn save_ptr_val(&mut self, identifier: &String, ptr_val: PointerValue<'ctx>) {
        match self.addr_map_stack.last_mut() {
            Some(map) => {
                map.insert(identifier.to_string(), ptr_val);
            }
            _ => { panic!(identifier.to_string() + " not found. Addr HashMap Stack overflow"); }
        }
    }

    fn gen_statement(&mut self, statement: &GNCAST) {
        // println!("in gen_statement {:?}", statement);
        match statement {
            GNCAST::ReturnStatement(ref ptr_to_expr) => {
                if ptr_to_expr.is_some() {
                    let expr = ptr_to_expr.as_ref().as_ref().unwrap();
                    let expr_val = self.gen_expression(&expr);
                    self.builder.build_return(Some(&expr_val));
                } else {
                    self.builder.build_return(None);
                }
            }
            GNCAST::Declaration(ref data_type, ref identifier) => {
                match data_type {
                    GNCType::Int => {
                        let point_value = self.builder.build_alloca(
                            self.context.i32_type(), identifier);
                        self.save_ptr_val(identifier, point_value);
                    }
                    // TODO More Types
                    _ => {
                        panic!("Invalid Type")
                    }
                }
            }
            GNCAST::Assignment(ref op,
                               ref identifier,
                               ref expr) => {
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
                );

                self.builder.build_store(self.get_point_value(identifier), val);
            }
            GNCAST::IfStatement(ref cond,
                                ref if_statements,
                                ref else_statements) => {
                self.gen_if_statement(cond, if_statements, else_statements);
            }
            GNCAST::WhileStatement(ref is_do_while,
                                   ref cond,
                                   ref while_statements) => {
                if *is_do_while {
                    self.gen_do_while_statements(cond, while_statements);
                } else {
                    self.gen_while_statements(cond, while_statements);
                }
            }
            _ => {
                panic!("Invalid Statement");
            }
        }
    }


    //>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>


    // generate if-else statements
    fn gen_if_statement(&mut self, cond: &Box<GNCAST>,
                        if_statements: &Box<GNCAST>,
                        else_statements: &Box<GNCAST>) {
        // get current function
        let func = self.current_function.unwrap();

        // get condition
        let cond_val = self.gen_expression(cond);

        // append 3 blocks
        let if_block = self.context.append_basic_block(func,
                                                       "if_block");
        let else_block = self.context.append_basic_block(func,
                                                         "else_block");
        let merge_block = self.context.append_basic_block(func,
                                                          "merge_block");

        // build condition statement
        self.builder.build_conditional_branch(cond_val, if_block, else_block);

        // build if_block
        self.builder.position_at_end(if_block);
        self.build_block(if_statements);
        if self.no_terminator() {
            self.builder.build_unconditional_branch(merge_block);
        }

        // build else_block
        self.builder.position_at_end(else_block);
        self.build_block(else_statements);
        if self.no_terminator() {
            self.builder.build_unconditional_branch(merge_block);
        }

        self.builder.position_at_end(merge_block);
    }


    // generate while statements
    fn gen_while_statements(&mut self, cond: &Box<GNCAST>, while_statements: &Box<GNCAST>) {
        let func = self.current_function.unwrap();

        let before_block = self.context.append_basic_block(func, "before_while");
        let while_block = self.context.append_basic_block(func, "while");
        let after_block = self.context.append_basic_block(func, "after_loop");

        // push labels
        // linking to the blocks
        self.continue_labels.push_back(while_block);
        self.break_labels.push_back(after_block);

        // unconditional branch to before_block
        self.builder.build_unconditional_branch(before_block);


        // build before block
        self.builder.position_at_end(before_block);
        let cond_val = self.gen_expression(cond);

        // build while conditional branch
        self.builder.build_conditional_branch(cond_val,
                                              while_block,
                                              after_block);
        self.builder.position_at_end(while_block);

        // build while block
        self.build_block(while_statements);
        if self.no_terminator() {
            self.builder.build_unconditional_branch(before_block);
        }

        // position to after block
        self.builder.position_at_end(after_block);

        self.break_labels.pop_back();
        self.continue_labels.pop_back();
    }

    // generate do-while statements
    fn gen_do_while_statements(&mut self, cond: &Box<GNCAST>, while_statements: &Box<GNCAST>) {}

    //>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

    // generate expressions
    fn gen_expression(&self, expression: &GNCAST) -> IntValue {
        match expression {
            GNCAST::Identifier(ref identifier) => {
                self.gen_identifier(identifier)
            }
            GNCAST::IntLiteral(ref int_literal) => {
                return self.context.i32_type().const_int(*int_literal as u64, true);
            }
            GNCAST::UnaryExpression(ref op, ref expr) => {
                self.gen_unary_expression(op, expr)
            }
            GNCAST::BinaryExpression(ref op, ref lhs, ref rhs) => {
                self.gen_binary_expression(op, lhs, rhs)
            }
            _ => { panic!("Invalid Expression Type") }
        }
    }


    // generate identifier and fetch value
    fn gen_identifier(&self, identifier: &String) -> IntValue {
        return self.builder.build_load(self.get_point_value(identifier), "load_val").into_int_value();
    }


    // generate unary expressions
    fn gen_unary_expression(&self, op: &UnaryOperator, expr: &Box<GNCAST>) -> IntValue {
        return match op {
            UnaryOperator::UnaryMinus => {
                self.builder.build_int_neg(self.gen_expression(&*expr), "building neg")
            }
            UnaryOperator::LogicalNot => {
                let res = self.builder.build_int_compare(
                    IntPredicate::EQ,
                    self.context.i32_type().const_int(0 as u64, true),
                    self.gen_expression(&*expr), "build logical not");

                let res = self.builder.build_int_cast(res, self.context.i32_type(), "logical not casting");
                let res = self.builder.build_int_sub(self.context.i32_type().const_int(0 as u64, true), res, "logical not");
                res
            }
            UnaryOperator::BitwiseComplement => {
                let res = self.builder.build_not(self.gen_expression(&*expr), "build not");
                res
            }
        };
    }

    // generate binary expression
    fn gen_binary_expression(&self, op: &BinaryOperator, lhs: &Box<GNCAST>, rhs: &Box<GNCAST>) -> IntValue {
        let lhs_v = self.gen_expression(lhs);
        let rhs_v = self.gen_expression(rhs);
        match op {
            BinaryOperator::Add => self.builder.build_int_add(lhs_v, rhs_v, "i32 add"),
            BinaryOperator::Subtract => self.builder.build_int_sub(lhs_v, rhs_v, "i32 sub"),
            BinaryOperator::Multiply => self.builder.build_int_mul(lhs_v, rhs_v, "i32 mul"),
            BinaryOperator::Divide => self.builder.build_int_signed_div(lhs_v, rhs_v, "i32 signed div"),
            BinaryOperator::Modulus => self.builder.build_int_signed_rem(lhs_v, rhs_v, "mod"),
            BinaryOperator::ShiftRight => self.builder.build_right_shift(lhs_v, rhs_v, true, "shr"),
            BinaryOperator::ShiftLeft => self.builder.build_left_shift(lhs_v, rhs_v, "shl"),
            BinaryOperator::NotEqual => self.builder.build_int_compare(IntPredicate::NE, lhs_v, rhs_v, "ne"),
            BinaryOperator::Equal => self.builder.build_int_compare(IntPredicate::EQ, lhs_v, rhs_v, "eq"),
            BinaryOperator::GreaterThan => self.builder.build_int_compare(IntPredicate::SGT, lhs_v, rhs_v, "gt"),
            BinaryOperator::GreaterEqual => self.builder.build_int_compare(IntPredicate::SGE, lhs_v, rhs_v, "ge"),
            BinaryOperator::LessThan => self.builder.build_int_compare(IntPredicate::SLT, lhs_v, rhs_v, "lt"),
            BinaryOperator::LessEqual => self.builder.build_int_compare(IntPredicate::SLE, lhs_v, rhs_v, "le"),
            BinaryOperator::BitwiseAnd => self.builder.build_and(lhs_v, rhs_v, "and"),
            BinaryOperator::ExclusiveOr => self.builder.build_xor(lhs_v, rhs_v, "xor"),
            BinaryOperator::InclusiveOr => self.builder.build_or(lhs_v, rhs_v, "or"),
            BinaryOperator::LogicalAnd => self.builder.build_and(
                self.builder.build_int_cast(lhs_v, self.context.bool_type(), "cast i32 to i1"),
                self.builder.build_int_cast(rhs_v, self.context.bool_type(), "cast i32 to i1"),
                "logical and",
            ),
            BinaryOperator::LogicalOr => self.builder.build_or(
                self.builder.build_int_cast(lhs_v, self.context.bool_type(), "cast i32 to i1"),
                self.builder.build_int_cast(rhs_v, self.context.bool_type(), "cast i32 to i1"),
                "logical or",
            ),
            BinaryOperator::FetchRHS => rhs_v
        }
    }


    //>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>


    fn build_block(&mut self, block: &Box<GNCAST>) {
        match **block {
            GNCAST::BlockStatement(ref statements) => {
                for statement in statements {
                    self.gen_statement(statement);
                }
            }
            _ => { panic!() }
        }
    }

    fn no_terminator(&self) -> bool {
        let block = self.builder.get_insert_block();
        let terminator = block.unwrap().get_terminator();
        return terminator.is_none();
    }
}
