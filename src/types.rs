#[cfg(feature = "local")]
use inkwell::types::{BasicTypeEnum};
use serde::{Serialize};
#[cfg(feature = "local")]
use anyhow::Result;
#[cfg(feature = "local")]
use checker::GNCErr;
use std::fmt;
#[cfg(feature = "local")]
use inkwell::context::Context;
#[cfg(feature = "local")]
use inkwell::types::{BasicType};
#[cfg(feature = "local")]
use inkwell::AddressSpace;


#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum GNCType {
    Void,
    Bool,
    Char,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
    Long,
    ULong,
    Float,
    Double,
    Pointer(Box<GNCType>),
}

#[cfg(feature = "local")]
impl GNCType {
    // check if is pointer type
    pub fn is_ptr_ty(&self) -> bool {
        match self {
            GNCType::Pointer(_) => true,
            _ => false
        }
    }

    // check if is int type
    pub fn is_int_ty(&self) -> bool {
        match self {
            GNCType::Bool | GNCType::Char | GNCType::UChar | GNCType::Short |
            GNCType::UShort | GNCType::Int | GNCType::UInt | GNCType::Long |
            GNCType::ULong => true,
            _ => false,
        }
    }

pub fn deref_ptr(&self) -> Result<GNCType> {
    return match self {
        GNCType::Pointer(ref ty) => Ok(*ty.clone()),
        _ => { Err(GNCErr::DereferenceNonPointer(self.clone()).into()) }
    };
}
}

impl fmt::Display for GNCType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[cfg(feature = "local")]
impl<'ctx> GNCType {
// convert from internal type to llvm type
pub fn to_basic_llvm_type(&self, ctx: &'ctx Context) -> BasicTypeEnum<'ctx> {
    match self {
        &GNCType::Bool => ctx.bool_type().as_basic_type_enum(),
        &GNCType::Char => ctx.i8_type().as_basic_type_enum(),
        &GNCType::UChar => ctx.i8_type().as_basic_type_enum(),
        &GNCType::Short => ctx.i16_type().as_basic_type_enum(),
        &GNCType::UShort => ctx.i16_type().as_basic_type_enum(),
        &GNCType::Int => ctx.i32_type().as_basic_type_enum(),
        &GNCType::UInt => ctx.i32_type().as_basic_type_enum(),
        &GNCType::Long => ctx.i64_type().as_basic_type_enum(),
        &GNCType::ULong => ctx.i64_type().as_basic_type_enum(),
        &GNCType::Float => ctx.f32_type().as_basic_type_enum(),
        &GNCType::Double => ctx.f64_type().as_basic_type_enum(),
        &GNCType::Pointer(ref ty) => ty.to_basic_llvm_type(ctx)
            .ptr_type(AddressSpace::Generic).as_basic_type_enum(),
        _ => { panic!() }
    }
}


// get upcast priority
fn priority(&self) -> i32 {
    match self {
        &GNCType::Void => -1,
        &GNCType::Pointer(_) => -1,
        &GNCType::Bool => 1,
        &GNCType::Char => 2,
        &GNCType::UChar => 2,
        &GNCType::Short => 3,
        &GNCType::UShort => 3,
        &GNCType::Int => 4,
        &GNCType::UInt => 4,
        &GNCType::Long => 5,
        &GNCType::ULong => 5,
        &GNCType::Float => 6,
        &GNCType::Double => 7,
    }
}

// binary cast (used in binary expression)
pub fn binary_cast(
    lhs_ty: &GNCType,
    rhs_ty: &GNCType,
) -> Result<GNCType> {
    // same type, return
    if lhs_ty == rhs_ty {
        return Ok(lhs_ty.clone());
    }

    // cast to pointer type
    if lhs_ty.is_ptr_ty() {
        return Ok(lhs_ty.clone());
    } else if rhs_ty.is_ptr_ty() {
        return Ok(rhs_ty.clone());
    }

    // get priority
    let lhs_priority = lhs_ty.priority();
    let rhs_priority = rhs_ty.priority();

    // cannot do default cast between these types
    if lhs_priority < 0 || rhs_priority < 0 {
        return Err(GNCErr::InvalidDefaultCast(rhs_ty.clone(), rhs_ty.clone()).into());
    }

    // default upcast
    if lhs_priority < rhs_priority {
        return Ok(rhs_ty.clone());
    } else if lhs_priority > rhs_priority {
        return Ok(lhs_ty.clone());
    }

    // otherwise
    return Err(GNCErr::InvalidDefaultCast(rhs_ty.clone(), rhs_ty.clone()).into());
}


// default cast
pub fn default_cast(&self, cast_ty: &GNCType) -> Result<GNCType> {
    // same type, directly cast
    if self == cast_ty {
        return Ok(cast_ty.clone());
    }

    if self.priority() < cast_ty.priority() {
        return Ok(cast_ty.clone());
    }

    return Err(GNCErr::InvalidDefaultCast(self.clone(), cast_ty.clone()).into());
}
}