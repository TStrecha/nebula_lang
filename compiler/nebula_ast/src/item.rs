#[derive(Debug, PartialEq, PartialOrd)]
pub enum Item {
    Expr(Expr),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Expr {
    VarDecl { name: String, value_type: Type, value: Box<Expr> },
    Lit(Literal),
    Ident(String),
}

// Not type specific, currently
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Literal {
    Number(u128),
    Decimal(f64),
    StringLit(String),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Type {
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
            _ => Type::Named(str),
        }
    }
}