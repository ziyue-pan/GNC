use inkwell::types::{BasicTypeEnum};
use serde::{Serialize};
use anyhow::Result;
use checker::GNCErr;


#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub enum GNCType {
    Void,
    Bool,
    Byte,
    UnsignedByte,
    Short,
    UnsignedShort,
    Int,
    UnsignedInt,
    Long,
    UnsignedLong,
    Float,
    Double,
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
            GNCType::UnsignedByte => 2,
            GNCType::Short => 3,
            GNCType::UnsignedShort => 3,
            GNCType::Int => 4,
            GNCType::UnsignedInt => 4,
            GNCType::Long => 5,
            GNCType::UnsignedLong => 5,
            GNCType::Float => 6,
            GNCType::Double => 7,
        }
    }

    // binary cast
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
        return Err(GNCErr::InvalidCast().into());
    }
}