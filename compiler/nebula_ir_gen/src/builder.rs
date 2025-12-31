use nebula_ast::item::Item;
use nebula_ir::module::IRModule;

#[derive(Debug)]
pub struct IRModuleBuilder {
    pub module: IRModule,
    ast: Vec<Item>,
}

impl IRModuleBuilder {

    pub fn new(ast: Vec<Item>) -> Self {
        Self {
            module: IRModule::new("main".to_string()),
            ast
        }
    }

    pub fn build(&mut self) {
        loop {
            self.next_ir_instruction_batch();

            if let None = self.peek_item() {
                break;
            }
        }
    }

    pub fn next_ir_instruction_batch(&mut self) {
        let next_item = self.consume_item();
        if let Some(item) = next_item {
            match item {
                Item::Expr(expr) => self.handle_expr(expr, None)
            }
        }
    }

    fn consume_item(&mut self) -> Option<Item> {
        if self.ast.is_empty() {
            return None;
        }

        Some(self.ast.remove(0))
    }

    fn peek_item(&mut self) -> Option<&Item> {
        if self.ast.is_empty() {
            return None;
        }

        self.ast.get(0)
    }
}