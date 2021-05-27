use colored::{Colorize};
use std::process;
use pest::Span;

//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//          GNCError
//    Defines all the recoverable
// errors when GNC executing.
//<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

pub enum GNCError {
    InvalidSuffix,
    MissingFunction(String),
    DuplicateFunction(String),
    ParameterCountMismatch(String, usize, usize),
    ParameterMismatch(),
    InvalidFunctionCall(),
}

impl GNCError {
    pub fn msg(&self) -> String {
        match self {
            GNCError::InvalidSuffix => "the source file extension must be `.c`!".to_string(),
            GNCError::MissingFunction(ref function_name) => {
                format!("cannot find function: {}", function_name.as_str().yellow())
            }
            GNCError::DuplicateFunction(ref function_name) => {
                format!("duplicate function: {}", function_name.as_str().yellow())
            }
            GNCError::ParameterCountMismatch(ref function_name,
                                             ref required_size,
                                             ref given_size) => {
                format!("parameter counts mismatch when calling `{}`, requires {}, found {}",
                        function_name.as_str().yellow(),
                        required_size.to_string().as_str().yellow(),
                        required_size.to_string().as_str().yellow())
            }
            GNCError::ParameterMismatch() => {
                "".to_string()
            }
            GNCError::InvalidFunctionCall() => {
                "".to_string()
            }
        }
    }

    pub fn handle(err: &GNCError, span: Option<&Span<'_>>) {
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