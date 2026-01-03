use nebula_ir::module::IRModule;
use nebula_tst::TypedSyntaxTree;
use crate::builder::IRModuleBuilder;

pub mod builder;
pub mod expr;

pub fn generate_ir_module(tst: TypedSyntaxTree) -> IRModule {
    let mut ir_module_builder = IRModuleBuilder::new(tst.items_owned());
    ir_module_builder.build();

    ir_module_builder.module
}