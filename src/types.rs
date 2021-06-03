#[cfg(feature = "local")]
use inkwell::types::{BasicTypeEnum};
use serde::{Serialize};
#[cfg(feature = "local")]
use anyhow::Result;
#[cfg(feature = "local")]
use checker::GNCErr;
use std::fmt;


#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
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
//    Pointer(Box<GNCType>),
}

impl fmt::Display for GNCType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "local")]
#[derive(Clone, Copy)]
pub struct Type<'ctx> {
    pub ty: GNCType,
    pub llvm_ty: BasicTypeEnum<'ctx>,
}

#[cfg(feature = "local")]
impl<'ctx> Type<'ctx> {
    // get upcast priority
    fn priority(&self) -> i32 {
        match self.ty {
            GNCType::Void => -1,
//            GNCType::Pointer(_) => -1,
            GNCType::Bool => 1,
            GNCType::Char => 2,
            GNCType::UChar => 2,
            GNCType::Short => 3,
            GNCType::UShort => 3,
            GNCType::Int => 4,
            GNCType::UInt => 4,
            GNCType::Long => 5,
            GNCType::ULong => 5,
            GNCType::Float => 6,
            GNCType::Double => 7,
        }
    }

    // binary cast (used in binary expression)
    pub fn binary_cast(
        lhs_ty: &Type<'ctx>,
        rhs_ty: &Type<'ctx>,
    ) -> Result<Type<'ctx>> {
        // same type, return
        if lhs_ty.ty == rhs_ty.ty {
            return Ok(*lhs_ty);
        }

        // get priority
        let lhs_priority = lhs_ty.priority();
        let rhs_priority = rhs_ty.priority();

        // cannot do default cast between these types
        if lhs_priority < 0 || rhs_priority < 0 {
            return Err(GNCErr::InvalidDefaultCast(rhs_ty.ty, rhs_ty.ty).into());
        }

        // default upcast
        if lhs_priority < rhs_priority {
            return Ok(*rhs_ty);
        } else if lhs_priority > rhs_priority {
            return Ok(*lhs_ty);
        }

        // otherwise
        return Err(GNCErr::InvalidDefaultCast(rhs_ty.ty, rhs_ty.ty).into());
    }


    // default cast
    pub fn default_cast(
        cur_ty: &Type<'ctx>,
        cast_ty: &Type<'ctx>,
    ) -> Result<Type<'ctx>> {
        // same type, directly cast
        if cur_ty.ty == cast_ty.ty {
            return Ok(*cast_ty);
        }

        if cur_ty.priority() < cast_ty.priority() {
            return Ok(*cast_ty);
        }

        return Err(GNCErr::InvalidDefaultCast(cur_ty.ty, cast_ty.ty).into());
    }
}