use crate::item::{Expr, Type};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Item {
    Expr(Typed<Expr>),
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Typed<T> {
    value: T,
    type_: Type,
}
