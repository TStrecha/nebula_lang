pub mod item;
pub mod ty;

use std::collections::HashMap;
use crate::item::TypedItem;
pub use crate::ty::{Type, TypedLiteral};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Place {
    Global,
    Local,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Symbol {
    pub name: String,
    pub place: Place,
    pub ty: Type,
}

#[derive(Debug, PartialEq, Default)]
pub struct TypedSyntaxTree {
    items: Vec<TypedItem>,
    symbols: HashMap<String, Symbol>,
}

impl TypedSyntaxTree {

    pub fn new(items: Vec<TypedItem>, symbols: HashMap<String, Symbol>) -> Self {
        Self { items, symbols }
    }

    pub fn items(&self) -> &Vec<TypedItem> {
        &self.items
    }

    pub fn items_owned(self) -> Vec<TypedItem> {
        self.items
    }

    pub fn symbols(&self) -> &HashMap<String, Symbol> {
        &self.symbols
    }
}
