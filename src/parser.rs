use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest::error::Error;
use std::fs::File;
use std::io::Read;
use pest::prec_climber::PrecClimber;
use pest::prec_climber::Assoc;
use pest::prec_climber::Operator;

#[derive(Parser)]
#[grammar = "./gnc.pest"]
struct GNCParser;


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
    Divide,
}

pub enum GNCAST {
    // Function AST: return type, name, parameter list and code block
    Function(GNCType, String, Vec<GNCParameter>, Vec<GNCAST>),
    ReturnStatement(Box<GNCAST>),
    UnaryExpression(UnaryOperator, Box<GNCAST>),
    BinaryExpression(BinaryOperator, Box<GNCAST>, Box<GNCAST>),
    IntLiteral(i32),
    Identifier(String),
    Declaration(GNCType, String),
    Assignment(String, Box<GNCAST>),
}

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

fn visit_gnc(pair: Pair<'_, Rule>, ast: &mut Vec<GNCAST>) {
    if pair.as_rule() != Rule::gnc {
        panic!("[ERROR] cannot find start parsing rule: gnc");
    }

    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::external_declaration => {
                for external_declaration in token.into_inner() {
                    visit_external_declaration(external_declaration, ast);
                }
            }

            _ => {}
        }
    }
}


fn visit_external_declaration(pair: Pair<'_, Rule>, ast: &mut Vec<GNCAST>) {
    match pair.as_rule() {
        Rule::function => {
            visit_function(pair, ast);
        }
        _ => { panic!("[ERROR] unexpected token while parsing the external declaration"); }
    }
}

fn visit_function(pair: Pair<'_, Rule>, ast: &mut Vec<GNCAST>) {
    let mut func_type: GNCType = GNCType::Int;
    let mut func_identifier: String = String::new();
    let mut func_parameter: Vec<GNCParameter> = vec![];
    let mut func_statements: Vec<GNCAST> = vec![];

    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::data_type => { func_type = visit_data_type(token); }
            Rule::identifier => { func_identifier = token.as_str().to_string(); }
            Rule::function_parameter_list => { visit_function_parameter_list(token, &mut func_parameter); }
            Rule::statement => { visit_statement(token, &mut func_statements); }
            _ => { panic!("[ERROR] unexpected token while parsing the function"); }
        }
    }

    ast.push(GNCAST::Function(func_type, func_identifier, func_parameter, func_statements));
    println!("add a function! ");
}

fn visit_data_type(pair: Pair<'_, Rule>) -> GNCType {
    match pair.as_str() {
        "int" => { GNCType::Int }
        "void" => { GNCType::Void }
        _ => { panic!("[ERROR] unexpected token while parsing the data type: {}", pair.as_str()); }
    }
}

fn visit_function_parameter_list(pair: Pair<'_, Rule>, func_param_list: &mut Vec<GNCParameter>) {
    match pair.as_rule() {
        _ => {}
    }
}

fn visit_statement(pair: Pair<'_, Rule>, func_statements: &mut Vec<GNCAST>) {
    let mut data_type: GNCType = GNCType::Int;

    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::return_statement => { visit_return_statement(token, func_statements); }
            Rule::data_type => { data_type = visit_data_type(token) }
            Rule::declaration => {
                visit_declaration(token, func_statements, data_type.clone())
            }
//            Rule::assignment => {
//                visit_assignment(token, func_statements)
//            } // TODO Assignment Parse;
            _ => { panic!("[ERROR] unexpected token while parsing statements"); }
        }
    }
}

fn visit_declaration(pair: Pair<'_, Rule>, func_statements: &mut Vec<GNCAST>, ty: GNCType) {
    let mut variable_name: String = String::new();

    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::identifier => {
                variable_name = token.as_str().to_string();
                func_statements.push(GNCAST::Declaration(ty.clone(), variable_name.clone()));
            }
            Rule::expression => {
                func_statements.push(GNCAST::Assignment(variable_name.clone(), Box::new(visit_expression(token))));
            }
            _ => { panic!("[ERROR] unexpected token while parsing return statement"); }
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

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        PrecClimber::new(vec![
            Operator::new(Rule::add, Assoc::Left) | Operator::new(Rule::subtract, Assoc::Left),
            Operator::new(Rule::multiply, Assoc::Left) | Operator::new(Rule::divide, Assoc::Left)
        ])
    };
}

fn visit_int_literal(pair: Pair<'_, Rule>) -> GNCAST {
    println!("{}", pair);
    let literal = pair.into_inner().next().unwrap();
    println!("{}", literal);
    match literal.as_rule() {
        Rule::dec_literal => GNCAST::IntLiteral(literal.as_str().to_string().parse::<i32>().unwrap()),
        _ => panic!("Unsupported int literal.")
    }
}

fn visit_expression(pairs: Pair<'_, Rule>) -> GNCAST {
    println!("{}", pairs);
    PREC_CLIMBER.climb(
        pairs.into_inner(),
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::int_literal => visit_int_literal(pair),
            Rule::identifier => GNCAST::Identifier(pair.as_str().to_string()),
            Rule::expression => visit_expression(pair),
            Rule::unary_expression => visit_unary(pair),
            _ => {
                println!("{}", pair);
                unreachable!()
            },
        },
        |lhs: GNCAST, op: Pair<Rule>, rhs: GNCAST| match op.as_rule() {
            Rule::add => GNCAST::BinaryExpression(BinaryOperator::Add, Box::new(lhs), Box::new(rhs)),
            Rule::subtract => GNCAST::BinaryExpression(BinaryOperator::Subtract, Box::new(lhs), Box::new(rhs)),
            Rule::multiply => GNCAST::BinaryExpression(BinaryOperator::Multiply, Box::new(lhs), Box::new(rhs)),
            Rule::divide => GNCAST::BinaryExpression(BinaryOperator::Divide, Box::new(lhs), Box::new(rhs)),
            _ => {
                unreachable!()
            },
        },
    )
}

fn visit_term(pair: Pair<'_, Rule>) -> GNCAST {
    match pair.as_rule() {
        Rule::int_literal => visit_int_literal(pair),
        Rule::identifier => GNCAST::Identifier(pair.as_str().to_string()),
        Rule::unary_expression => visit_unary(pair),
        Rule::expression => visit_expression(pair),
        _ => unreachable!()
    }
}

fn visit_unary(pair: Pair<'_, Rule>) -> GNCAST {
    println!("unary parsed: {}", pair);
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::negative_unary => {
                let expression = visit_term(token.into_inner().next().unwrap());
                return GNCAST::UnaryExpression(UnaryOperator::UnaryMinus, Box::new(expression));
            }
            Rule::logical_not_unary => {
                let expression = visit_term(token.into_inner().next().unwrap());
                return GNCAST::UnaryExpression(UnaryOperator::LogicalNot, Box::new(expression));
            }
            Rule::bitwise_complement_unary => {
                let expression = visit_term(token.into_inner().next().unwrap());
                return GNCAST::UnaryExpression(UnaryOperator::BitwiseComplement, Box::new(expression));
            }
            _ => { panic!("[ERROR] unexpected token while parsing expressions {}", token); }
        }
    }
    panic!("[ERROR] missing unary while parsing expressions");
}