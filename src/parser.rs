use pest::iterators::{Pair};
use types::GNCType;
use serde::{Serialize};


#[derive(Parser, Serialize)]
#[grammar = "./gnc.pest"]
pub struct GNCParser;


//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//      All the AST Enums
//<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

#[derive(Debug, Serialize)]
pub struct GNCParameter {
    pub param_type: GNCType,
    pub param_name: String,
}

#[derive(Debug, Serialize)]
pub enum UnaryOperator {
    UnaryMinus,
    LogicalNot,
    BitwiseComplement,
}

#[derive(Debug, Serialize)]
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
    FetchRHS,
}

impl BinaryOperator {
    pub fn is_compare(&self) -> bool {
        match self {
            BinaryOperator::Equal |
            BinaryOperator::LessThan |
            BinaryOperator::GreaterThan |
            BinaryOperator::LessEqual |
            BinaryOperator::GreaterEqual |
            BinaryOperator::NotEqual => true,
            _ => { false }
        }
    }
}


#[derive(Debug, Serialize)]
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


#[derive(Serialize, Debug)]
pub enum GNCAST {
    // Function AST: return type, name, parameter list and code block
    Function(GNCType, String, Vec<GNCParameter>, Vec<GNCAST>),

    // If Statement AST: expression, if-statement-list, else_statements
    IfStatement(Box<GNCAST>, Box<GNCAST>, Box<GNCAST>),

    // While Statement AST: is_do_while, condition, while_statements
    WhileStatement(bool, Box<GNCAST>, Box<GNCAST>),

    // For Statement AST: init_clause, condition, iteration, for_statements
    //      To support declaration in the init clause, we have
    // to set the first parameter as Vec.
    ForStatement(Vec<GNCAST>, Box<Option<GNCAST>>, Box<Option<GNCAST>>, Box<GNCAST>),

    // Statements block: (a new scope)
    BlockStatement(Vec<GNCAST>),

    // Function Call:
    FunctionCall(String, Vec<GNCAST>),

    // Global Variable
    GlobalDeclaration(GNCType, String, Box<GNCAST>),

    ContinueStatement,
    BreakStatement,
    ReturnStatement(Box<Option<GNCAST>>),
    UnaryExpression(UnaryOperator, Box<GNCAST>),
    BinaryExpression(BinaryOperator, Box<GNCAST>, Box<GNCAST>),

    // Literals
    BoolLiteral(bool),
    IntLiteral(i64),
    FloatLiteral(f64),

    Identifier(String),
    Declaration(GNCType, String),
    Assignment(AssignOperation, String, Box<GNCAST>),
}


//>>>>>>>>>>>>>>>>>>>>>>>>
//      PARSER
//<<<<<<<<<<<<<<<<<<<<<<<<


// driver for the parser
pub fn parse(gnc_pair: Pair<'_, Rule>) -> Vec<GNCAST> {
    let mut ast = vec![];
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
        match token.as_rule() {
            Rule::function => { visit_function(token, ast); }
            Rule::global_variable => { visit_global_variable(token, ast) }
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
    let mut pairs = pair.into_inner();
    let mut token_pair = pairs.next();

    while token_pair.is_some() {
        let mut token = token_pair.unwrap().into_inner();

        let param_type = token.next().unwrap();
        let param_name = token.next().unwrap();

        func_param_list.push(GNCParameter {
            param_type: visit_data_type(param_type),
            param_name: param_name.as_str().to_string(),
        });

        token_pair = pairs.next();
    }
}

// parse global variable
fn visit_global_variable(pair: Pair<'_, Rule>, ast: &mut Vec<GNCAST>) {
    let mut data_type: GNCType = GNCType::Int;
    let mut variable_name: String = String::new();

    let mut pairs = pair.into_inner();
    let mut token_pair = pairs.next();
    let mut uncommitted = false;

    while token_pair.is_some() {
        let token = token_pair.unwrap();
        match token.as_rule() {
            Rule::data_type => {
                data_type = visit_data_type(token);
            }
            Rule::identifier => {
                if uncommitted {
                    ast.push(GNCAST::GlobalDeclaration(data_type,
                                                       variable_name.clone(),
                                                       Box::new(GNCAST::IntLiteral(0))));
                }
                variable_name = token.as_str().to_string();
                uncommitted = true;
            }
            Rule::expression => {
                if uncommitted {
                    ast.push(GNCAST::GlobalDeclaration(data_type,
                                                       variable_name.clone(),
                                                       Box::new(visit_expression(token))));
                    uncommitted = false;
                }
            }
            _ => { panic!("[ERROR] unexpected token while parsing declaration statement"); }
        }
        token_pair = pairs.next();
    }

    if uncommitted {
        ast.push(GNCAST::GlobalDeclaration(data_type,
                                           variable_name.clone(),
                                           Box::new(GNCAST::IntLiteral(0))));
    }
}


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
        match token.as_rule() {
            Rule::declaration_statement => {
                visit_declaration_statement(token, func_statements);
            }
            Rule::expression => {
                let expr = visit_expression(token);
                func_statements.push(expr);
            }
            Rule::if_statement => {
                visit_if_statement(token, func_statements);
            }
            Rule::while_statement => {
                visit_while_statement(token, func_statements, false);
            }
            Rule::do_while_statement => {
                visit_while_statement(token, func_statements, true);
            }
            Rule::for_statement => {
                visit_for_statement(token, func_statements);
            }
            Rule::block_statement => {
                visit_block(token, func_statements);
            }
            Rule::continue_statement => {
                func_statements.push(GNCAST::ContinueStatement);
            }
            Rule::break_statement => {
                func_statements.push(GNCAST::BreakStatement);
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


fn visit_if_statement(pair: Pair<'_, Rule>, func_statements: &mut Vec<GNCAST>) {
    let mut condition: Option<GNCAST> = None;
    let mut if_statements: Vec<GNCAST> = vec![];
    let mut else_statements: Vec<GNCAST> = vec![];

    let mut pairs = pair.into_inner();
    let mut token_pair = pairs.next();
    let mut first_statement = true;

    while token_pair.is_some() {
        let token = token_pair.unwrap();
        if token.as_rule() == Rule::expression {
            condition = Option::from(visit_expression(token));
        } else if token.as_rule() == Rule::statement {
            if first_statement {
                first_statement = false;
                visit_statement(token, &mut if_statements);
            } else {
                visit_statement(token, &mut else_statements);
            }
        } else {
            break;
        }
        token_pair = pairs.next();
    }

    if condition.is_some() {
        let if_block = GNCAST::BlockStatement(if_statements);
        let else_block = GNCAST::BlockStatement(else_statements);
        func_statements.push(GNCAST::IfStatement(Box::new(condition.unwrap()),
                                                 Box::new(if_block),
                                                 Box::new(else_block)));
    } else {
        panic!()
    }
}


fn visit_declaration_statement(pair: Pair<'_, Rule>, func_statements: &mut Vec<GNCAST>) {
    let mut data_type: GNCType = GNCType::Int;
    let mut variable_name: String = String::new();

    let mut pairs = pair.into_inner();
    let mut token_pair = pairs.next();

    while token_pair.is_some() {
        let token = token_pair.unwrap();
        match token.as_rule() {
            Rule::data_type => {
                data_type = visit_data_type(token);
            }
            Rule::identifier => {
                variable_name = token.as_str().to_string();
                func_statements.push(GNCAST::Declaration(data_type, variable_name.clone()))
            }
            Rule::expression => {
                func_statements.push(GNCAST::Assignment(AssignOperation::Simple,
                                                        variable_name.clone(),
                                                        Box::new(visit_expression(token))));
            }
            _ => { panic!("[ERROR] unexpected token while parsing declaration statement"); }
        }
        token_pair = pairs.next();
    }
}


fn visit_return_statement(pair: Pair<'_, Rule>, func_statements: &mut Vec<GNCAST>) {
    let token = pair.into_inner().next();

    if token.is_some() {
        let expr = token.unwrap();

        let return_expression = visit_expression(expr);
        func_statements.push(
            GNCAST::ReturnStatement(Box::new(Some(return_expression)))
        );
    } else {
        func_statements.push(
            GNCAST::ReturnStatement(Box::new(None))
        );
    }
}


fn visit_while_statement(pair: Pair<'_, Rule>, func_statements: &mut Vec<GNCAST>, is_do_while: bool) {
    let mut condition: Option<GNCAST> = None;
    let mut while_statements: Vec<GNCAST> = vec![];

    for token in pair.into_inner() {
        match token.as_rule() {
            Rule::expression => {
                condition = Option::from(visit_expression(token));
            }
            Rule::statement => {
                visit_statement(token, &mut while_statements);
            }
            _ => { panic!(); }
        }
    }

    if condition.is_some() {
        let while_block = GNCAST::BlockStatement(while_statements);
        func_statements.push(GNCAST::WhileStatement(is_do_while,
                                                    Box::new(condition.unwrap()),
                                                    Box::new(while_block)));
    } else {
        panic!();
    }
}


fn visit_for_statement(pair: Pair<'_, Rule>, func_statements: &mut Vec<GNCAST>) {
    let mut init_clause: Vec<GNCAST> = vec![];
    let mut condition: Option<GNCAST> = None;
    let mut iteration: Option<GNCAST> = None;

    let mut for_statements: Vec<GNCAST> = vec![];

    let mut count = 0;
    let mut pairs = pair.into_inner();
    let mut token_pair = pairs.next();


    while token_pair.is_some() {
        let token = token_pair.unwrap();
        match token.as_rule() {
            Rule::declaration_statement => {
                visit_declaration_statement(token, &mut init_clause);
            }
            Rule::expression => {
                let expr: GNCAST = visit_expression(token);

                match count {
                    0 => { init_clause.push(expr); }
                    1 => { condition = Option::from(expr); }
                    2 => { iteration = Option::from(expr); }
                    _ => { panic!(); }
                }
            }
            Rule::statement => {
                visit_statement(token, &mut for_statements);
            }
            Rule::none_for_condition => {}
            _ => { panic!(); }
        }
        token_pair = pairs.next();
        count += 1;
    }

    let for_block = GNCAST::BlockStatement(for_statements);
    func_statements.push(GNCAST::ForStatement(init_clause,
                                              Box::new(condition),
                                              Box::new(iteration),
                                              Box::new(for_block)));
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
    let pair = pairs.next();

    if pair.is_some() {
        let expr = pair.unwrap();

        return match expr.as_rule() {
            Rule::bool_literal => visit_bool_literal(expr),
            Rule::float_literal => visit_float_literal(expr),
            Rule::int_literal => visit_int_literal(expr),
            Rule::bracket_expression => visit_expression(expr.into_inner().next().unwrap()),
            Rule::function_call => visit_function_call(expr),
            Rule::identifier => GNCAST::Identifier(expr.as_str().to_string()),
            _ =>
                GNCAST::UnaryExpression(
                    match expr.as_rule() {
                        Rule::op_arithmetic_not => UnaryOperator::UnaryMinus,
                        Rule::op_logical_not => UnaryOperator::LogicalNot,
                        Rule::op_bitwise_not => UnaryOperator::BitwiseComplement,
                        _ => { panic!() }
                    },
                    Box::new(visit_expression(pairs.next().unwrap())),
                )
        };
    }
    panic!("")
}


// parse function call
fn visit_function_call(pair: Pair<'_, Rule>) -> GNCAST {
    let mut pairs = pair.into_inner();

    let function_name = pairs.next().unwrap().as_str().to_string();
    let mut call_parameters: Vec<GNCAST> = vec![];

    // iterate parameters
    let mut expr_pair = pairs.next();
    while expr_pair.is_some() {
        let expr = expr_pair.unwrap();
        call_parameters.push(visit_expression(expr));
        expr_pair = pairs.next();
    }

    return GNCAST::FunctionCall(function_name, call_parameters);
}


//>>>>>>>>>>>>>>>>>>>>>
//      tokens
//<<<<<<<<<<<<<<<<<<<<<
fn visit_data_type(pair: Pair<'_, Rule>) -> GNCType {
    let token = pair.into_inner().next().unwrap();

    match token.as_rule() {
        Rule::bool => GNCType::Bool,
        Rule::byte => GNCType::Byte,
        Rule::unsigned_byte => GNCType::UByte,
        Rule::short => GNCType::Short,
        Rule::unsigned_short => GNCType::UShort,
        Rule::int => GNCType::Int,
        Rule::unsigned_int => GNCType::UInt,
        Rule::long => GNCType::Long,
        Rule::unsigned_long => GNCType::ULong,
        Rule::float => GNCType::Float,
        Rule::double => GNCType::Double,
        Rule::void => GNCType::Void,
        _ => { panic!("[ERROR] unexpected data type: {}", token.as_str()); }
    }
}


//>>>>>>>>>>>>>>>>>>
//      literals
//<<<<<<<<<<<<<<<<<<

// int literal
fn visit_int_literal(pair: Pair<'_, Rule>) -> GNCAST {
    let literal_rule = pair.into_inner().next().unwrap();


    match literal_rule.as_rule() {
        Rule::bin_literal => {
            let literal_str = literal_rule.as_str();
            let literal = i64::from_str_radix(&literal_str[2..literal_str.len()],
                                              2).unwrap();
            return GNCAST::IntLiteral(literal);
        }
        Rule::oct_literal => {
            let literal_str = literal_rule.as_str();
            let literal = i64::from_str_radix(&literal_str[2..literal_str.len()],
                                              8).unwrap();
            return GNCAST::IntLiteral(literal);
        }
        Rule::hex_literal => {
            let literal_str = literal_rule.as_str();
            let literal = i64::from_str_radix(&literal_str[2..literal_str.len()],
                                              16).unwrap();
            return GNCAST::IntLiteral(literal);
        }
        Rule::dec_literal => {
            let literal = literal_rule.as_str().to_string().parse::<i64>().unwrap();
            return GNCAST::IntLiteral(literal);
        }
        _ => { panic!("cannot parse int literal") }
    }
}

// bool literal
fn visit_bool_literal(pair: Pair<'_, Rule>) -> GNCAST {
    match pair.as_str() {
        "true" => GNCAST::BoolLiteral(true),
        _ => GNCAST::BoolLiteral(false),
    }
}

// float literal
fn visit_float_literal(pair: Pair<'_, Rule>) -> GNCAST {
    let literal_rst = pair.as_str().to_string().parse::<f64>();

    match literal_rst {
        Ok(literal) => GNCAST::FloatLiteral(literal),
        Err(_) => { panic!("cannot parse float literal") }
    }
}



