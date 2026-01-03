#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Type {
    Void,
    Builtin(BuiltinType),
    Named(String),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum BuiltinType {
    U8, U16, U32, U64,
    F32, F64,
    Bool,
    String,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TypedLiteral {
    U8(u8), U16(u16), U32(u32), U64(u64),
    F32(f32), F64(f64),
    Bool(bool),
    String(String),
}

impl From<String> for Type {
    fn from(str: String) -> Self {
        match str.as_str() {
            "u8" => Type::Builtin(BuiltinType::U8),
            "u16" => Type::Builtin(BuiltinType::U16),
            "u32" => Type::Builtin(BuiltinType::U32),
            "u64" => Type::Builtin(BuiltinType::U64),
            "f32" => Type::Builtin(BuiltinType::F32),
            "f64" => Type::Builtin(BuiltinType::F64),
            "bool" => Type::Builtin(BuiltinType::Bool),
            "string" => Type::Builtin(BuiltinType::String),
            "void" => Type::Void,
            _ => Type::Named(str),
        }
    }
}
