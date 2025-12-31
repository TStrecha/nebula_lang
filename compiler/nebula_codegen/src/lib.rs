use inkwell::context::Context;
use inkwell::types::{BasicTypeEnum};
use inkwell::values::{BasicValueEnum};
use nebula_ir::instruction::{IRInstruction};
use nebula_ir::module::IRModule;
use nebula_ir::value::{IRConst, IRType};

pub fn generate_llvm_code(ir_module: IRModule) -> String {
    let context = Context::create();
    let builder = context.create_builder();
    let module = context.create_module(ir_module.name());

    {
        let void_type = context.void_type().fn_type(&[], false);
        let main_function = module.add_function("main", void_type, None);
        let entry_bb = context.append_basic_block(main_function, "entry");
        builder.position_at_end(entry_bb);
    }

    for global in ir_module.globals() {

        let (global_type, value_initializer): (BasicTypeEnum, Option<BasicValueEnum>) = match global.ty {

            // IRConst::Number(val) => {
            //     (context.i64_type().into(), context.i64_type().const_int(val, false).into())
            // }
            // IRConst::Decimal(val) => {
            //     (context.f64_type().into(), context.f64_type().const_float(val).into())
            // }
            // IRConst::String(val) => {
            //     let bytes = context.const_string(val.as_bytes(), true);
            //     (bytes.get_type().into(), bytes.into())
            // }
            IRType::U8 => { (context.i8_type().into(), test(&context, &global.initializer)) }
            IRType::U16 => { (context.i16_type().into(), test(&context, &global.initializer)) }
            IRType::U32 => { (context.i32_type().into(), test(&context, &global.initializer)) }
            IRType::U64 => { (context.i64_type().into(), test(&context, &global.initializer)) }
            IRType::F32 => { (context.f32_type().into(), test(&context, &global.initializer)) }
            IRType::F64 => { (context.f64_type().into(), test(&context, &global.initializer)) }
            IRType::Bool => { (context.bool_type().into(), test(&context, &global.initializer)) }
            IRType::String => unimplemented!()
        };

        let declared_global = module.add_global(global_type, None, &global.name);

        if let Some(initializer) = value_initializer {
            declared_global.set_initializer(&initializer);
        }
        declared_global.set_constant(true);
    }


    for instruction in ir_module.instructions() {
        match instruction {
            IRInstruction::Load { .. } => {}
            IRInstruction::Store { to, value } => {

                // let (global_type, value_initializer): (BasicTypeEnum, BasicValueEnum) = match source {
                //     IRConst::Number(val) => {
                //         (context.i64_type().into(), context.i64_type().const_int(val, false).into())
                //     }
                //     IRConst::Decimal(val) => {
                //         (context.f64_type().into(), context.f64_type().const_float(val).into())
                //     }
                //     IRConst::String(val) => {
                //         let bytes = context.const_string(val.as_bytes(), true);
                //         (bytes.get_type().into(), bytes.into())
                //     }
                // };
                //
                // let declared_global = module.add_global(global_type, None, &name);
                // declared_global.set_initializer(&value_initializer);
                // declared_global.set_constant(true);
            }
            _ => {}
        }
    }

    builder.build_return(None);
    module.print_to_string().to_string()
}

pub fn test<'ctx>(context: &'ctx Context, value: &Option<IRConst>) -> Option<BasicValueEnum<'ctx>> {
    match value {
        None => None,
        Some(value) => {
            let llvm_initializer: BasicValueEnum = match value {
                IRConst::U8(val) => context.i8_type().const_int(*val as u64, true).into(),
                IRConst::U16(val) => context.i16_type().const_int(*val as u64, true).into(),
                IRConst::U32(val) => context.i32_type().const_int(*val as u64, true).into(),
                IRConst::U64(val) => context.i64_type().const_int(*val, true).into(),
                IRConst::F32(val) => context.f32_type().const_float(*val).into(),
                IRConst::F64(val) => context.f64_type().const_float(*val).into(),
                IRConst::String(val) => context.const_string(val.as_bytes(), true).into(),
            };

            Some(llvm_initializer)
        }
    }
}