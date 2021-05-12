use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest::error::Error;
use std::fs::File;
use std::io::Read;
use pest::prec_climber::PrecClimber;
use pest::prec_climber::Assoc;
use pest::prec_climber::Operator;
use parser::BinaryOperator::{LogicalOr, LogicalAnd};
use parser::UnaryOperator::UnaryMinus;
use parser::GNCAST::{UnaryExpression, BinaryExpression, Assignment};
use parser::AssignOperation::Simple;

#[derive(Parser)]
#[grammar = "./gnc.pest"]
struct GNCParser;


//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//      All the AST Enums
//<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
#[derive(Clone, Copy)]
pub enum GNCType {
    Void,
    Int,
}

pub struct GNCParameter {
    param_type: GNCType,
    param_name: String,
}

pub enum GNCStatement {
    Return(String),
}

pub enum UnaryOperator {
    UnaryMinus,
    LogicalNot,
    BitwiseComplement,
}

pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Modulus,
    Divide,
    Equal,
    ShiftRight,
    ShiftLeft,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    NotEqual,
    BitwiseAnd,
    ExclusiveOr,
    InclusiveOr,
    LogicalAnd,
    LogicalOr,
}

pub enum AssignOperation {
    Simple,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulus,
    BitwiseAnd,
    InclusiveOr,
    ExclusiveOr,
    ShiftLeft,
    ShiftRight,
}

pub enum GNCAST {
    // Function AST: return type, name, parameter list and code block
    Function(GNCType, String, Vec<GNCParameter>, Vec<GNCAST>),
    Block(Vec<GNCAST>),
    ReturnStatement(Box<GNCAST>),
    UnaryExpression(UnaryOperator, Box<GNCAST>),
    BinaryExpression(BinaryOperator, Box<GNCAST>, Box<GNCAST>),
    IntLiteral(i32),
    Identifier(String),
    Declaration(GNCType, String),
    Assignment(AssignOperation, String, Box<GNCAST>),
}

// implement method for GNCAST
impl GNCAST {
    // rust cannot infer type inside the enum
    // so I have to write this method to extract statements from function body
    fn get_func_body(self) -> Option<Vec<GNCAST>> {
        if let GNCAST::Block(statements_list) = self { Some(statements_list) } else { None }
    }
}


//>>>>>>>>>>>>>>>>>>>>>>>>
//      PARSER
//<<<<<<<<<<<<<<<<<<<<<<<<


// driver for the parser
pub fn parse(source_path: &str) -> Vec<GNCAST> {
    let mut source_file: File = File::open(source_path).expect("Unable to open source file!");
    let mut source_content: String = String::new();
    source_file.read_to_string(&mut source_content).expect("Unable to read the file!");

    let mut pairs = GNCParser::parse(Rule::gnc, &source_content).unwrap_or_else(|e| panic!("{}", e));
    let mut ast = vec![];
    let gnc_pair = pairs.next().unwrap();

    visit_gnc(gnc_pair, &mut ast);

    return ast;
}

//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// this is the start rule of GNC
//<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
fn visit_gnc(pair: Pair<'_, Rule>, ast: &mut Vec<GNCAST>) {
    if pair.as_rule() != Rule::gnc {
        panic!("[ERROR] cannot find start parsing rule: gnc");
    }

    for token in pair.into_inner() {
        println!("{}", token);

        match token.as_rule() {
            Rule::function => {
                visit_function(token, ast);
            }
            Rule::global_variable => {
                visit_global_variable(token, ast)
            }
            _ => {}
        }
    }
}


//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//  functions & global variables
//<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
fn visit_function(pair: Pair<'_, Rule>, ast: &mut Vec<GNCAST>) {
    let mut func_type: GNCType = GNCType::Int;
    let mut func_identifier: String = String::new();
    let mut func_parameter: Vec<GNCParameter> = vec![];
    let mut func_statements: Vec<GNCAST> = vec![];

    for token in pair.into_inner() {
        println!("{}", token);
        match token.as_rule() {
            Rule::data_type => { func_type = visit_data_type(token); }
            Rule::identifier => { func_identifier = token.as_str().to_string(); }
            Rule::function_parameter_list => { visit_function_parameter_list(token, &mut func_parameter); }
            Rule::statement => { visit_statement(token, &mut func_statements); }
            _ => { panic!("[ERROR] unexpected token while parsing the function"); }
        }
    }

    ast.push(GNCAST::Function(func_type, func_identifier, func_parameter, func_statements));
}


fn visit_function_parameter_list(pair: Pair<'_, Rule>, func_param_list: &mut Vec<GNCParameter>) {
    match pair.as_rule() {
        _ => {}
    }
}


// TODO global variable
fn visit_global_variable(pair: Pair<'_, Rule>, ast: &mut Vec<GNCAST>) {}


//>>>>>>>>>>>>>>>>>>>>>>>>>>
//      statements
//<<<<<<<<<<<<<<<<<<<<<<<<<<
fn visit_block(pair: Pair<'_, Rule>, func_statements: &mut Vec<GNCAST>) {
    for statement in pair.into_inner() {
        visit_statement(statement, func_statements);
    }
}


fn visit_statement(pair: Pair<'_, Rule>, func_statements: &mut Vec<GNCAST>) {
    for token in pair.into_inner() {
        println!("{}", token);
        match token.as_rule() {
            Rule::declaration_statement => {
                visit_declaration_statement(token, func_statements);
            }
            Rule::expression => {
                visit_expression(token);
            }
            Rule::if_statement => {
                // TODO
            }
            Rule::while_statement => {
                // TODO
            }
            Rule::do_while_statement => {
                // TODO
            }
            Rule::do_while_statement => {
                // TODO
            }
            Rule::for_statement => {
                // TODO
            }
            Rule::return_statement => { visit_return_statement(token, func_statements); }
            _ => { panic!("[ERROR] unexpected token while parsing statements"); }
        }
    }
}

//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
// The basic method for converting some statements to the AST
// is to traverse the corresponding function and operate on
// the parameter **func_statements: &mut Vec<GNCAST>**.
//
// Each traverse push the AST of the statements onto **func_statements**.
//<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<


fn visit_declaration_statement(pair: Pair<'_, Rule>, func_statements: &mut Vec<GNCAST>) {
    let mut data_type: GNCType = GNCType::Int;
    let mut variable_name: String = String::new();

    for token in pair.into_inner() {
        println!("{}", token);
        match token.as_rule() {
            Rule::data_type => {
                data_type = visit_data_type(token);
            }
            Rule::declaration => {
                for inner_token in token.into_inner() {
                    match inner_token.as_rule() {
                        Rule::identifier => {
                            variable_name = inner_token.as_str().to_string();
                            func_statements.push(GNCAST::Declaration(data_type, variable_name.clone()))
                        }
                        Rule::expression => {
                            func_statements.push(GNCAST::Assignment(AssignOperation::Simple,
                                                                    variable_name.clone(),
                                                                    Box::new(visit_expression(inner_token))));
                        }
                        _ => { panic!() }
                    }
                }
            }
            _ => { panic!("[ERROR] unexpected token while parsing declaration statement"); }
        }
    }
}


fn visit_return_statement(pair: Pair<'_, Rule>, func_statements: &mut Vec<GNCAST>) {
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::expression => {
                let return_expression = visit_expression(token);
                func_statements.push(GNCAST::ReturnStatement(Box::new(return_expression)));
            }
            _ => { panic!("[ERROR] unexpected token while parsing return statement"); }
        }
    }
}


//>>>>>>>>>>>>>>>>>>>>>>>>>>
//      expressions
//<<<<<<<<<<<<<<<<<<<<<<<<<<
fn visit_expression(pair: Pair<'_, Rule>) -> GNCAST {
    return if pair.as_rule() == Rule::assignment_expression {
        visit_assignment(pair)
    } else if pair.as_rule() == Rule::unary_expression {
        visit_unary(pair)
    } else {
        visit_binary(pair)
    };
}

fn visit_assignment(pair: Pair<'_, Rule>) -> GNCAST {
    let mut lhs: String = String::new();
    let mut assign_op = AssignOperation::Simple;

    for token in pair.into_inner() {
        if token.as_rule() == Rule::identifier {
            lhs = token.as_str().to_string();
        } else if token.as_rule() == Rule::expression {
            let rhs = visit_expression(token);
            let assign = GNCAST::Assignment(assign_op, lhs, Box::new(rhs));
            return assign;
        } else {
            assign_op = match token.as_rule() {
                Rule::assign_simple => AssignOperation::Simple,
                Rule::assign_div => AssignOperation::Division,
                Rule::assign_mul => AssignOperation::Multiplication,
                Rule::assign_mod => AssignOperation::Modulus,
                Rule::assign_add => AssignOperation::Addition,
                Rule::assign_sub => AssignOperation::Subtraction,
                Rule::assign_shift_left => AssignOperation::ShiftLeft,
                Rule::assign_shift_right => AssignOperation::ShiftRight,
                Rule::assign_bitwise_and => AssignOperation::BitwiseAnd,
                Rule::assign_exclusive_or => AssignOperation::ExclusiveOr,
                Rule::assign_inclusive_or => AssignOperation::InclusiveOr,
                _ => { panic!(); }
            }
        }
    }
    panic!();
}

fn visit_binary(pair: Pair<'_, Rule>) -> GNCAST {
    let mut pairs = pair.into_inner();

    let mut lhs = visit_expression(pairs.next().unwrap());
    let mut expr = pairs.next(); // now is the symbol

    while expr.is_some() {
        let op = match expr.unwrap().as_rule() {
            Rule::op_add => BinaryOperator::Add,
            Rule::op_sub => BinaryOperator::Subtract,
            Rule::op_mul => BinaryOperator::Multiply,
            Rule::op_div => BinaryOperator::Divide,
            Rule::op_mod => BinaryOperator::Modulus,
            Rule::op_shift_right => BinaryOperator::ShiftRight,
            Rule::op_shift_left => BinaryOperator::ShiftLeft,
            Rule::op_gt => BinaryOperator::GreaterThan,
            Rule::op_lt => BinaryOperator::LessThan,
            Rule::op_ge => BinaryOperator::GreaterEqual,
            Rule::op_le => BinaryOperator::LessEqual,
            Rule::op_ne => BinaryOperator::NotEqual,
            Rule::op_eq => BinaryOperator::Equal,
            Rule::op_bitwise_and => BinaryOperator::BitwiseAnd,
            Rule::op_exclusive_or => BinaryOperator::ExclusiveOr,
            Rule::op_inclusive_or => BinaryOperator::InclusiveOr,
            Rule::op_logical_and => BinaryOperator::LogicalAnd,
            Rule::op_logical_or => BinaryOperator::LogicalOr,
            _ => { panic!(); }
        };
        expr = pairs.next();
        let rhs = visit_expression(expr.unwrap());
        lhs = GNCAST::BinaryExpression(
            op,
            Box::new(lhs),
            Box::new(rhs),
        );
        expr = pairs.next();
    }
    return lhs;
}


fn visit_unary(pair: Pair<'_, Rule>) -> GNCAST {
    let mut pairs = pair.into_inner();
    let mut pair = pairs.next();

    if pair.is_some() {
        let expr = pair.unwrap();
        return if expr.as_str() == "(" {
            visit_expression(pairs.next().unwrap())
        } else if expr.as_rule() == Rule::int_literal {
            visit_int_literal(expr)
        } else if expr.as_rule() == Rule::identifier {
            GNCAST::Identifier(expr.as_str().to_string())
        } else {
            GNCAST::UnaryExpression(
                match expr.as_rule() {
                    Rule::op_arithmetic_not => UnaryOperator::UnaryMinus,
                    Rule::op_logical_not => UnaryOperator::LogicalNot,
                    Rule::op_bitwise_not => UnaryOperator::BitwiseComplement,
                    _ => { panic!() }
                },
                Box::new(visit_expression(expr)),
            )
        };
    }
    panic!("")
}


//>>>>>>>>>>>>>>>>>>>>>
//      tokens
//<<<<<<<<<<<<<<<<<<<<<
fn visit_data_type(pair: Pair<'_, Rule>) -> GNCType {
    match pair.as_str() {
        "int" => { GNCType::Int }
        "void" => { GNCType::Void }
        _ => { panic!("[ERROR] unexpected token while parsing the data type: {}", pair.as_str()); }
    }
}


fn visit_int_literal(pair: Pair<'_, Rule>) -> GNCAST {
    let literal = pair.into_inner().next().unwrap();
    match literal.as_rule() {
        Rule::dec_literal => GNCAST::IntLiteral(literal.as_str().to_string().parse::<i32>().unwrap()),
        _ => panic!("Unsupported int literal.")
    }
}