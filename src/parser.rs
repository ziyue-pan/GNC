use std::io::Read;
use pest::Parser;
use pest::error::Error;
use std::fs::{File};


use self::AstNode::*;
use self::Type::*;


#[derive(Parser)]
#[grammar = "./gnalc.pest"]
struct GnalcParser;

pub enum Type {
    Void,
    Int,
}

pub enum AstNode {
    // confuse: Print的用法
    Print(Box<AstNode>),
    // Box是一个智能指针，用来在堆上分配内存

    ExternalDeclarationList(Vec<AstNode>),

    // Function: Type, Param Name, Function Name, Pointer to Body
    Function(Type, Vec<String>, String, Box<AstNode>),
    StatementList(Vec<AstNode>),
    ReturnStatement(Option<Box<AstNode>>),
    IntLiteral(i32),
}


//* parser driver
pub fn parse(source_path: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut source_file: File = File::open(source_path).expect("Unable to open source file!");
    let mut source_content: String = String::new();
    source_file.read_to_string(&mut source_content).expect("Unable to read the file!");

    let mut ast = vec![];

    let pairs = GnalcParser::parse(Rule::gnalc, &source_content).unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        match pair.as_rule() {
            Rule::gnalc => {     // 此处指定AST的入口规则（不改变）
                ast.push(Print(Box::new(build_ast(pair))));
            }
            _ => {}
        }
    }

    print!("DONE!");
    Ok(ast)
}


// 定义AST遍历路径
fn build_ast(pair: pest::iterators::Pair<Rule>) -> AstNode {
    println!("{:?}", pair.as_rule());

    match pair.as_rule() {
        Rule::gnalc => {
            let external_declaration_list: Vec<AstNode> = pair.into_inner().map(build_ast).collect();
            ExternalDeclarationList(external_declaration_list)
        }
        Rule::external_declaration => build_ast(pair.into_inner().next().unwrap()),
        Rule::function => {
            let mut pair = pair.into_inner();
            let data_type = build_type(pair.next().unwrap());
            let function_name = pair.next().unwrap().as_str().to_string();
            let function_parameter_list = pair.next().unwrap();
            let function_body: Vec<AstNode> = pair.next().unwrap().into_inner().map(build_ast).collect();

            // TODO parse function parameter list
            Function(data_type, Vec::new(), function_name, Box::new(StatementList(function_body)))
        }
        Rule::statement => build_ast(pair.into_inner().next().unwrap()),
        Rule::return_statement => ReturnStatement(Option::from(Box::new(build_ast(pair.into_inner().next().unwrap())))),
        Rule::expression => build_ast(pair.into_inner().next().unwrap()),
        Rule::int_literal => IntLiteral(pair.as_str().parse().unwrap()),
        _ => panic!("unsupported grammar: {:?}", pair.as_rule()),
    }
}


fn build_type(pair: pest::iterators::Pair<Rule>) -> Type {
    match pair.as_str() {
        "int" => Int,
        _ => panic!("unsupported data tyoe: {}", pair.as_str()),
    }
}

