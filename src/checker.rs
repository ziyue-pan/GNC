use colored::{Colorize};
//use pest::Span;
use thiserror::Error;
use types::GNCType;


//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//          GNCError
//    Defines all the recoverable
// errors when GNC executing.
//<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

// TODO add more info in parameter mismatch
#[derive(Error, Debug)]
pub enum GNCErr {
    /* --- MISC --- */
    #[error("the source file extension must be `.c`!")]
    InvalidSuffix,

    /* --- functions --- */
    #[error("cannot find function: {:?}", .0.as_str().yellow())]
    MissingFunction(String),

    #[error("there are duplicate functions: {:?}", 0.as_str().yellow())]
    DuplicateFunction(String),

    /* --- variables --- */
    #[error("there are duplicate global variables: {:?}", 0.as_str().yellow())]
    DuplicateGlobalVar(String),


    #[error("")]
    MissingVariable(String),

    /* --- types --- */
    #[error("")]
    InvalidType(GNCType),

    #[error("")]
    InvalidCast(),

    /* unary expression */
    #[error("")]
    InvalidUnary(),

    #[error("")]
    ParameterCountMismatch(String, usize, usize),

    #[error("")]
    ParameterMismatch(),

    #[error("")]
    InvalidFunctionCall(),

    #[error("")]
    TypeMismatch(),

    #[error("")]
    InvalidFloatingPointOperation(),

    #[error("")]
    ReturnTypeMismatch(),


    #[error(transparent)]
    Other(#[from] anyhow::Error),  // source and Display delegate to anyhow::Error
}


//            GNCErr::MissingFunction(ref function_name) => {
//                format!("cannot find function: {}", function_name.as_str().yellow())
//            }
//            GNCErr::DuplicateFunction(ref function_name) => {
//                format!("duplicate function: {}", function_name.as_str().yellow())
//            }
//            GNCErr::ParameterCountMismatch(ref function_name,
//                                           ref required_size,
//                                           ref given_size) => {
//                format!("parameter counts mismatch when calling `{}`, requires {}, found {}",
//                        function_name.as_str().yellow(),
//                        required_size.to_string().as_str().yellow(),
//                        given_size.to_string().as_str().yellow())
//            }
//            GNCErr::ParameterMismatch() => {
//                "Parameter Mismatch".to_string()
//            }
//            GNCErr::InvalidFunctionCall() => {
//                "Invalid Function Call".to_string()
//            }
//            GNCErr::DuplicateGlobalVar(_) => {
//                "Duplicate Global Variable".to_string()
//            }
//            GNCErr::MissingVariable(_) => {
//                "Missing Variable".to_string()
//            }
//            GNCErr::InvalidType(ty) => {
//                ty.to_string()
//            }
//            GNCErr::ImplicitTypeCast() => {
//                "".to_string()
//            }
//            GNCErr::InvalidFloatingPointOperation() => {
//                "".to_string()
//            }
//            GNCErr::ReturnTypeMismatch() => {
//                "".to_string()
//            }
//        }
//    }
//
//    fn handle(&self, span: Option<&Span<'_>>) {
//        println!("{} {}", "[ERROR]".red().bold(), self.msg());
//
//        if span.is_some() {
//            let span_info = span.unwrap();
//            let start_line = span_info.start_pos().line_col().0;
//            let start_col = span_info.start_pos().line_col().1;
//
//            let end_line = span_info.end_pos().line_col().0;
//            let end_col = span_info.end_pos().line_col().1;
//            let token = span_info.as_str();
//
//            if start_line == end_line {
//                println!("\n{} at {}:{}, from {}:{} to {}:{}, token: {}",
//                         ">>>".red().bold(),
//                         "line".yellow().bold(),
//                         start_line.to_string().as_str().yellow().bold(),
//                         "col".yellow().bold(),
//                         start_col.to_string().as_str().yellow().bold(),
//                         "col".yellow().bold(),
//                         end_col.to_string().as_str().yellow().bold(),
//                         token.cyan().bold());
//            } else {
//                println!("\n{} from [{}:{},{}:{}] to [{}:{},{}:{}], token: {}",
//                         ">>>".red().bold(),
//                         "line".yellow().bold(),
//                         start_line.to_string().as_str().yellow().bold(),
//                         "col".yellow().bold(),
//                         start_col.to_string().as_str().yellow().bold(),
//                         "line".yellow().bold(),
//                         end_line.to_string().as_str().yellow().bold(),
//                         "col".yellow().bold(),
//                         end_col.to_string().as_str().yellow().bold(),
//                         token.cyan().bold());
//            }
//        }
//
//        process::exit(1);
//    }
//
//}
//
//impl fmt::Display for GNCErrorType {
//    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//        write!(f, "{} {}", "[ERROR]".red().bold(), self.msg())
//    }
//}