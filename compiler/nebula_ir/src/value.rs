use std::hash::Hash;
use crate::identification::{GlobalId, LocalId, PointerIdentifierKind, TempId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
pub enum IRLiteral {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
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
    Literal(IRLiteral),
}

#[derive(Debug, Clone)]
pub enum IRPlace {
    Global(GlobalId),
    Local(LocalId),
    Temp(TempId),
}

impl IRPlace {

    pub fn as_identifier(&self) -> PointerIdentifierKind<'_> {
        match self {
            IRPlace::Global(id) => id.as_identifier(),
            IRPlace::Local(id) => id.as_identifier(),
            IRPlace::Temp(id) => id.as_identifier(),
        }
    }
}

impl From<&IRLiteral> for IRType {
    fn from(literal: &IRLiteral) -> Self {
        match literal {
            IRLiteral::U8(_) => Self::U8,
            IRLiteral::U16(_) => Self::U16,
            IRLiteral::U32(_) => Self::U32,
            IRLiteral::U64(_) => Self::U64,
            IRLiteral::F32(_) => Self::F32,
            IRLiteral::F64(_) => Self::F64,
            IRLiteral::String(_) => Self::String,
        }
    }
}