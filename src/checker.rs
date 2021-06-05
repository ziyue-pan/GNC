use colored::{Colorize};
//use pest::Span;
use thiserror::Error;
use types::GNCType;
use parser::{BinaryOperator};


//>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
//          GNCError
//    Defines all the recoverable
// errors when GNC executing.
//<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

#[derive(Error, Debug)]
pub enum GNCErr {
    /* --- MISC --- */
    #[error("the source file extension must be `.c`!")]
    InvalidSuffix,

    #[error("unknown expression: {}", .0.as_str().yellow())]
    UnknownExpression(String),

    #[error("redefinition of symbol: {}", .0.as_str().yellow())]
    Redefinition(String),

    /* --- functions --- */
    #[error("cannot find function: {}", .0.as_str().yellow())]
    MissingFunction(String),

    #[error("there are duplicate functions: {}", .0.as_str().yellow())]
    DuplicateFunction(String),

    /* --- variables --- */
    #[error("there are duplicate global variables: {}", .0.as_str().yellow())]
    DuplicateGlobalVar(String),

    #[error("there are duplicate local variables: {}", .0.as_str().yellow())]
    DuplicateVar(String),

    #[error("missing variable: {}", .0.as_str().yellow())]
    MissingVariable(String),

    #[error("void type variable is not allowed: {}", .0.as_str().yellow())]
    VoidVariable(String),

    /* --- types --- */
    #[error("invalid left value")]
    InvalidLeftValue(),

    #[error("cannot cast from {} to {}",
    .0.to_string().as_str().yellow(),
    .1.to_string().as_str().yellow())]
    InvalidCast(GNCType, GNCType),

    #[error("cannot cast automatically between {} and {}", .0.to_string().as_str().yellow(),
    .1.to_string().as_str().yellow())]
    InvalidDefaultCast(GNCType, GNCType),

    #[error("cannot dereference non-pointer type: {}", .0.to_string().as_str().yellow())]
    DereferenceNonPointer(GNCType),

    #[error("cannot reference non-variable")]
    ReferenceNonVariable(),

    /* unary expression */
    #[error("")]
    InvalidUnary(),

    #[error("parameter count mismatch")]
    ParameterCountMismatch(String, usize, usize),



    #[error("invalid function call")]
    InvalidFunctionCall(),


    /* binary expression */
    #[error("this type: {} does not support the operation: {}",
    .0.to_string().as_str().yellow(), .1.to_string().as_str().yellow())]
    InvalidOperation(GNCType, BinaryOperator),

    #[error("")]
    ReturnTypeMismatch(),

    #[error(transparent)]
    Other(#[from] anyhow::Error),  // source and Display delegate to anyhow::Error
}

