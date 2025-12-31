use nebula_ast::item::{BuiltinType, Type};
use crate::identification::{GlobalId, LocalId, TempId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IRType {
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    String,
}

#[derive(Debug, Clone)]
pub enum IRConst {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f64),
    F64(f64),
    String(String),
}

#[derive(Debug, Clone)]
pub struct IRTemp {
    pub id: TempId,
    pub ty: IRType,
}

#[derive(Debug, Clone)]
pub enum IRValue {
    Temp(IRTemp),
    Const(IRConst),
}

#[derive(Debug, Clone)]
pub enum IRPlace {
    Global(GlobalId),
    Local(LocalId),
}

impl From<&Type> for IRType {
    fn from(ty: &Type) -> Self {
        match ty {
            Type::Builtin(builtin) => match builtin {
                BuiltinType::Bool =>    IRType::Bool,
                BuiltinType::U8 =>      IRType::U8,
                BuiltinType::U16 =>     IRType::U16,
                BuiltinType::U32 =>     IRType::U32,
                BuiltinType::U64 =>     IRType::U64,
                BuiltinType::F32 =>     IRType::F32,
                BuiltinType::F64 =>     IRType::F64,
                BuiltinType::String =>  IRType::String,
            }
            Type::Named(..) => unimplemented!()
        }
    }
}