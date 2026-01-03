use std::sync::atomic::AtomicU32;
use nebula_ir::identification::{GlobalId, LocalId, TempId};
use nebula_ir::module::IRModule;
use nebula_tst::item::TypedItem;

//todo move after implementing scopes
const LOCAL_INDEX: AtomicU32 = AtomicU32::new(0);
const GLOBAL_INDEX: AtomicU32 = AtomicU32::new(0);
const TEMP_INDEX: AtomicU32 = AtomicU32::new(0);

#[derive(Debug)]
pub struct IRModuleBuilder {
    pub module: IRModule,
    ast: Vec<TypedItem>,
}

impl IRModuleBuilder {

    pub fn new(ast: Vec<TypedItem>) -> Self {
        Self {
            module: IRModule::new("main".to_string()),
            ast,
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
                TypedItem::Expr(expr) => self.handle_expr(expr, None)
            }
        }
    }

    fn consume_item(&mut self) -> Option<TypedItem> {
        if self.ast.is_empty() {
            return None;
        }

        Some(self.ast.remove(0))
    }

    fn peek_item(&mut self) -> Option<&TypedItem> {
        if self.ast.is_empty() {
            return None;
        }

        self.ast.get(0)
    }
}

pub(crate) fn create_global_id() -> GlobalId {
    GLOBAL_INDEX.fetch_add(1, std::sync::atomic::Ordering::SeqCst).into()
}

pub(crate) fn create_local_id() -> LocalId {
    LOCAL_INDEX.fetch_add(1, std::sync::atomic::Ordering::SeqCst).into()
}

pub(crate) fn create_temp_id() -> TempId {
    TEMP_INDEX.fetch_add(1, std::sync::atomic::Ordering::SeqCst).into()
}
