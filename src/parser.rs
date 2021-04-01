use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Parser)]
#[grammar = "./gnalc.pest"]
struct GnalcParser;


pub enum GnalcType {
    Void,
    Int,
}

pub struct GnalcParameter {
    param_type: GnalcType,
    param_name: String,
}

pub enum GnalcStatement {
    Return(String),
}

pub enum UnaryOperator {
    UnaryMinus,
    LogicalNot,
    BitwiseComplement,
}

pub enum GnalcAST {
    // Function AST: return type, name, parameter list and code block
    Function(GnalcType, String, Vec<GnalcParameter>, Vec<GnalcAST>),
    ReturnStatement(Box<GnalcAST>),
    UnaryExpression(UnaryOperator, Box<GnalcAST>),
    IntLiteral(i32),
    LocalDeclaration(GnalcType, String),
    //TODO GlobalDeclaration(GnalcType, String)
}

pub fn parse(source_path: &str) -> Vec<GnalcAST> {
    let mut source_file: File = File::open(source_path).expect("Unable to open source file!");
    let mut source_content: String = String::new();
    source_file.read_to_string(&mut source_content).expect("Unable to read the file!");

    let mut pairs = GnalcParser::parse(Rule::gnalc, &source_content).unwrap_or_else(|e| panic!("{}", e));
    let mut ast = vec![];
    let gnalc_pair = pairs.next().unwrap();

    visit_gnalc(gnalc_pair, &mut ast);

    return ast;
}

fn visit_gnalc(pair: Pair<'_, Rule>, ast: &mut Vec<GnalcAST>) {
    if pair.as_rule() != Rule::gnalc {
        panic!("[ERROR] cannot find start parsing rule: gnalc");
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


fn visit_external_declaration(pair: Pair<'_, Rule>, ast: &mut Vec<GnalcAST>) {
    match pair.as_rule() {
        Rule::function => {
            visit_function(pair, ast);
        }
        _ => { panic!("[ERROR] unexpected token while parsing the external declaration"); }
    }
}

fn visit_function(pair: Pair<'_, Rule>, ast: &mut Vec<GnalcAST>) {
    let mut func_type: GnalcType = GnalcType::Int;
    let mut func_identifier: String = String::new();
    let mut func_parameter: Vec<GnalcParameter> = vec![];
    let mut func_statements: Vec<GnalcAST> = vec![];

    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::data_type => { func_type = visit_data_type(token); }
            Rule::identifier => { func_identifier = token.as_str().to_string(); }
            Rule::function_parameter_list => { visit_function_parameter_list(token, &mut func_parameter); }
            Rule::statement => { visit_statement(token, &mut func_statements); }
            _ => { panic!("[ERROR] unexpected token while parsing the function"); }
        }
    }

    ast.push(GnalcAST::Function(func_type, func_identifier, func_parameter, func_statements));
    println!("add a function! ");
}

fn visit_data_type(pair: Pair<'_, Rule>) -> GnalcType {
    match pair.as_str() {
        "int" => { GnalcType::Int }
        "void" => { GnalcType::Void }
        _ => { panic!("[ERROR] unexpected token while parsing the data type: {}", pair.as_str()); }
    }
}

fn visit_function_parameter_list(pair: Pair<'_, Rule>, func_param_list: &mut Vec<GnalcParameter>) {
    match pair.as_rule() {
        _ => {}
    }
}

fn visit_statement(pair: Pair<'_, Rule>, func_statements: &mut Vec<GnalcAST>) {
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::return_statement => { visit_return_statement(token, func_statements); }
            Rule::variable_decalaration => {visit_variable_declaration(token, func_statements)}
            _ => { panic!("[ERROR] unexpected token while parsing statements"); }
        }
    }
}

fn visit_variable_declaration(pair: Pair<'_, Rule>, func_statements: &mut Vec<GnalcAST>) {
    let mut declare_type: GnalcType = GnalcType::Int;
    let mut variable_name : String = String::new();

    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::data_type  => { declare_type  = visit_data_type(token); }
            Rule::identifier => { variable_name = token.as_str().to_string(); }
            _ => { panic!("[ERROR] unexpected token while parsing return statement"); }
        }
    }
    func_statements.push(GnalcAST::LocalDeclaration(declare_type, variable_name));
}


fn visit_return_statement(pair: Pair<'_, Rule>, func_statements: &mut Vec<GnalcAST>) {
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::expression => {
                let return_expression = visit_expression(token);
                func_statements.push(GnalcAST::ReturnStatement(Box::new(return_expression)));
            }
            _ => { panic!("[ERROR] unexpected token while parsing return statement"); }
        }
    }
}

fn visit_expression(pair: Pair<'_, Rule>) -> GnalcAST {
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::unary_expression => {
                return visit_unary(token);

                // match token.as_str().to_string().parse::<i32>() {
                //     Ok(int_literal) => GnalcAST::IntLiteral(int_literal),
                //     Err(E) => panic!("[ERROR] unexpected token while parsing int literal"),
                // }
            }
            _ => { panic!("[ERROR] unexpected token while parsing expressions {}", token); }
        }
    }
    panic!("[ERROR] missing token while parsing expressions");
}

fn visit_unary(pair: Pair<'_, Rule>) -> GnalcAST {
    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::int_literal => {
                let int_literal = token.as_str().to_string().parse::<i32>().unwrap();
                return GnalcAST::IntLiteral(int_literal);
            }
            Rule::negative_unary => {
                let unary_expression = visit_expression(token);
                return GnalcAST::UnaryExpression(UnaryOperator::UnaryMinus, Box::new(unary_expression))
            }
            Rule::logical_not_unary => {
                let unary_expression = visit_expression(token);
                return GnalcAST::UnaryExpression(UnaryOperator::LogicalNot, Box::new(unary_expression))
            }
            Rule::bitwise_complement_unary => {
                let unary_expression = visit_expression(token);
                return GnalcAST::UnaryExpression(UnaryOperator::BitwiseComplement, Box::new(unary_expression))
            }
            _ => { panic!("[ERROR] unexpected token while parsing expressions {}", token); }
        }
    }
    panic!("[ERROR] missing unary while parsing expressions");
}





