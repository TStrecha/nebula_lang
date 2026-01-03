mod context;

use std::sync::atomic::AtomicU32;
use inkwell::builder::Builder;
use inkwell::context::Context;
use nebula_ir::instruction::IRInstruction;
use nebula_ir::module::IRModule;
use crate::context::{CodegenContext, Module};

const STRING_LIT_INDEX: AtomicU32 = AtomicU32::new(0);

pub fn generate_llvm_ir(ir_modules: Vec<IRModule>) -> String {
    let context = Context::create();
    let builder = context.create_builder();

    let mut codegen_context = CodegenContext::new();

    for ir_module in ir_modules {
        let module = create_llvm_module(ir_module, &context, &builder);
        codegen_context.add_module(module);
    }

    codegen_context.generate_to_string()
}

pub fn create_llvm_module<'ctx>(ir_module: IRModule, context: &'ctx Context, builder: &'ctx Builder) -> Module<'ctx> {
    let mut module = Module::new("main", context, builder);
    module.generate_entry_point();

    for global in ir_module.globals() {
        module.add_global(global)
    }

    for instruction in ir_module.instructions() {
        match instruction {
            IRInstruction::LoadLiteral { target, value } =>
                module.load_lit(target.as_identifier(), value),
            IRInstruction::Load { target, from } =>
                module.load(target, from.as_identifier()),
            IRInstruction::Store { to, value } =>
                module.store(to.as_identifier(), value),
        }
    }

    module.add_void_return();

    module
}