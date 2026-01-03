use crate::{Symbol, Type, TypedLiteral};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TypedItem {
    Expr(TypedExpr),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum TypedExpr {
    VarDecl { name: String, ty: Type, value: Box<TypedExpr> },
    Return { ty: Type, value: Box<TypedExpr> },
    Lit(TypedLiteral),
    Ident { symbol: Symbol, ty: Type },
}