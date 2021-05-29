use colored::{Colorize};
use std::process;
use pest::Span;
use parser::GNCType;

//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//          GNCError
//    Defines all the recoverable
// errors when GNC executing.
//<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

pub enum GNCErr {
    InvalidSuffix,
    MissingFunction(String),
    MissingVariable(String),
    DuplicateFunction(String),
    DuplicateGlobalVar(String),
    InvalidType(GNCType),
    ParameterCountMismatch(String, usize, usize),

    // TODO add more info in parameter mismatch
    ParameterMismatch(),
    InvalidFunctionCall(),
}

impl GNCErr {
    pub fn msg(&self) -> String {
        match self {
            GNCErr::InvalidSuffix => "the source file extension must be `.c`!".to_string(),
            GNCErr::MissingFunction(ref function_name) => {
                format!("cannot find function: {}", function_name.as_str().yellow())
            }
            GNCErr::DuplicateFunction(ref function_name) => {
                format!("duplicate function: {}", function_name.as_str().yellow())
            }
            GNCErr::ParameterCountMismatch(ref function_name,
                                           ref required_size,
                                           ref given_size) => {
                format!("parameter counts mismatch when calling `{}`, requires {}, found {}",
                        function_name.as_str().yellow(),
                        required_size.to_string().as_str().yellow(),
                        given_size.to_string().as_str().yellow())
            }
            GNCErr::ParameterMismatch() => {
                "Parameter Mismatch".to_string()
            }
            GNCErr::InvalidFunctionCall() => {
                "Invalid Function Call".to_string()
            }
            GNCErr::DuplicateGlobalVar(_) => {
                "Duplicate Global Variable".to_string()
            }
            GNCErr::MissingVariable(_) => {
                "Missing Variable".to_string()
            }
            GNCErr::InvalidType(ty) => {
                ty.to_string()
            }
        }
    }

    pub fn handle(err: &GNCErr, span: Option<&Span<'_>>) {
        println!("{} {}", "[ERROR]".red().bold(), err.msg());

        if span.is_some() {
            let span_info = span.unwrap();
            let start_line = span_info.start_pos().line_col().0;
            let start_col = span_info.start_pos().line_col().1;

            let end_line = span_info.end_pos().line_col().0;
            let end_col = span_info.end_pos().line_col().1;
            let token = span_info.as_str();

            if start_line == end_line {
                println!("\n{} at {}:{}, from {}:{} to {}:{}, token: {}",
                         ">>>".red().bold(),
                         "line".yellow().bold(),
                         start_line.to_string().as_str().yellow().bold(),
                         "col".yellow().bold(),
                         start_col.to_string().as_str().yellow().bold(),
                         "col".yellow().bold(),
                         end_col.to_string().as_str().yellow().bold(),
                         token.cyan().bold());
            } else {
                println!("\n{} from [{}:{},{}:{}] to [{}:{},{}:{}], token: {}",
                         ">>>".red().bold(),
                         "line".yellow().bold(),
                         start_line.to_string().as_str().yellow().bold(),
                         "col".yellow().bold(),
                         start_col.to_string().as_str().yellow().bold(),
                         "line".yellow().bold(),
                         end_line.to_string().as_str().yellow().bold(),
                         "col".yellow().bold(),
                         end_col.to_string().as_str().yellow().bold(),
                         token.cyan().bold());
            }
        }

        process::exit(1);
    }
}