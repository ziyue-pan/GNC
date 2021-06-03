use inkwell::types::{BasicTypeEnum};
use serde::{Serialize};
use anyhow::Result;
use checker::GNCErr;
use std::fmt;


#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum GNCType {
    Void,
    Bool,
    Byte,
    UByte,
    Short,
    UShort,
    Int,
    UInt,
    Long,
    ULong,
    Float,
    Double,
}

impl fmt::Display for GNCType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Clone, Copy)]
pub struct Type<'ctx> {
    pub ty: GNCType,
    pub llvm_ty: BasicTypeEnum<'ctx>,
}


impl<'ctx> Type<'ctx> {
    // get upcast priority
    fn priority(&self) -> i32 {
        match self.ty {
            GNCType::Void => 0,
            GNCType::Bool => 1,
            GNCType::Byte => 2,
            GNCType::UByte => 2,
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

        // default upcast
        if lhs_ty.priority() < rhs_ty.priority() {
            return Ok(*rhs_ty);
        } else if lhs_ty.priority() > rhs_ty.priority() {
            return Ok(*lhs_ty);
        }

        // otherwise
        return Err(GNCErr::InvalidDefaultCast(rhs_ty.ty, rhs_ty.ty).into());
    }


    // cast
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