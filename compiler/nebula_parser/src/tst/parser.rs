use std::collections::HashMap;
use nebula_ast::item::{Item};
use nebula_tst::{Symbol, TypedSyntaxTree};
use nebula_tst::item::TypedItem;

pub struct TypedAstParser {
    pub ast: Vec<Item>,

    pub typed_items: Vec<TypedItem>,
    pub symbols: HashMap<String, Symbol>,
}

impl TypedAstParser {
    pub fn new(ast: Vec<Item>) -> Self {
        Self {
            ast,
            typed_items: vec![],
            symbols: HashMap::new(),
        }
    }

    pub fn next_typed_item(&mut self) -> Option<TypedItem> {
        if self.ast.len() == 0 {
            return None;
        }

        let typed_item = match self.ast.remove(0) {
            Item::Expr(expr) => TypedItem::Expr(self.handle_expr(expr))
        };

        Some(typed_item)
    }

    pub fn build_typed_tree(mut self) -> TypedSyntaxTree {
        while let Some(typed_items) = self.next_typed_item() {
            self.typed_items.push(typed_items);
        }

        TypedSyntaxTree::new(self.typed_items, self.symbols)
    }
}