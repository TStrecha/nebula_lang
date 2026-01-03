use std::collections::HashMap;
use inkwell::AddressSpace;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module as LLVMModule;
use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, GlobalValue, PointerValue};
use nebula_ir::identification::{GlobalId, PointerIdentifierKind, LocalId, TempId};
use nebula_ir::module::IRGlobal;
use nebula_ir::value::{IRLiteral, IRTemp, IRType, IRValue};
use crate::STRING_LIT_INDEX;

pub struct CodegenContext<'ctx> {
    modules: HashMap<&'ctx str, Module<'ctx>>
}

impl<'ctx> CodegenContext<'ctx> {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub(crate) fn generate_to_string(&self) -> String {
        self.modules.values()
            .map(|module| module.generate_to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn add_module(&mut self, module: Module<'ctx>) {
        self.modules.insert(module.name, module);
    }
}

pub struct Module<'ctx> {
    llvm: &'ctx Context,
    builder: &'ctx Builder<'ctx>,
    name: &'ctx str,
    module: LLVMModule<'ctx>,

    globals: HashMap<GlobalId, GlobalValue<'ctx>>,
    locals: HashMap<LocalId, PointerValue<'ctx>>,
    temps: HashMap<TempId, PointerValue<'ctx>>,
}

impl<'ctx> Module<'ctx> {
    pub fn new(name: &'ctx str, llvm: &'ctx Context, builder: &'ctx Builder) -> Self {
        Self {
            llvm,
            builder,
            name,
            module: llvm.create_module(name),

            globals: HashMap::new(),
            locals: HashMap::new(),
            temps: HashMap::new(),
        }
    }

    // todo better way after adding functions
    pub fn generate_entry_point(&self) {
        let void_return_type = self.llvm.void_type().fn_type(&[], false);
        let _int_return_type = self.llvm.i32_type().fn_type(&[], false);
        let main_function = self.module.add_function("main", void_return_type, None);
        let entry_bb = self.llvm.append_basic_block(main_function, "entry");
        self.builder.position_at_end(entry_bb);
    }
    pub fn add_void_return(&self) {
        self.builder.build_return(None).unwrap();
    }

    pub fn add_global(&mut self, ir_global: &IRGlobal) {
        let value_initializer = ir_type_to_llvm_value(&self.llvm, &ir_global.ty);

        let global_type = ir_type_to_llm_type(&self.llvm, &ir_global.ty);
        let declared_global = self.module.add_global(global_type, None, &ir_global.name);

        declared_global.set_initializer(&value_initializer);

        self.globals.insert(ir_global.id, declared_global);
    }

    pub(crate) fn load_lit(&self, id: PointerIdentifierKind, value: &IRLiteral) {
        let llvm_value = self.ir_literal_to_llvm_value(value);
        let ptr = self.get_pointer_from_identifier(id);

        self.builder.build_store(ptr, llvm_value).unwrap();
    }

    pub(crate) fn load(&mut self, target: &IRTemp, id: PointerIdentifierKind) {
        let ty = ir_type_to_llm_type(&self.llvm, &target.ty);
        let ptr = self.get_pointer_from_identifier(id);

        self.builder.build_load(ty, ptr, &format!("tmp${}", target.id.0)).unwrap();
        self.temps.insert(target.id, ptr);
    }


    pub(crate) fn store(&self, id: PointerIdentifierKind, value: &IRValue) {
        match value {
            IRValue::Temp(temp) => {
                let ptr_to = self.get_pointer_from_identifier(id);
                let ptr_from = self.temps[&temp.id];

                self.builder.build_store(ptr_to, ptr_from).unwrap();

            }
            IRValue::Literal(lit) => self.load_lit(id, lit),
        }
    }

    pub(crate) fn generate_to_string(&self) -> String {
        self.module.print_to_string().to_string()
    }

    pub(self) fn get_pointer_from_identifier(&self, id: PointerIdentifierKind) -> PointerValue<'ctx> {
        match id {
            PointerIdentifierKind::Global(id) => self.globals.get(id).unwrap().as_pointer_value(),
            PointerIdentifierKind::Local(id) => *self.locals.get(id).unwrap(),
        }
    }

    pub fn ir_literal_to_llvm_value(&self, literal: &IRLiteral) -> BasicValueEnum {
        match literal {
            IRLiteral::U8(val) => self.llvm.i8_type().const_int(*val as u64, true).into(),
            IRLiteral::U16(val) => self.llvm.i16_type().const_int(*val as u64, true).into(),
            IRLiteral::U32(val) => self.llvm.i32_type().const_int(*val as u64, true).into(),
            IRLiteral::U64(val) => self.llvm.i64_type().const_int(*val, true).into(),
            IRLiteral::F32(val) => self.llvm.f32_type().const_float(*val as f64).into(),
            IRLiteral::F64(val) => self.llvm.f64_type().const_float(*val).into(),
            IRLiteral::String(val) => {
                let string_id = STRING_LIT_INDEX.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

                let str_const = self.llvm.const_string(val.as_bytes(), true);
                let str_global = self.module.add_global(
                    str_const.get_type(),
                    None,
                    &format!(".str.{}", string_id)
                );

                str_global.set_initializer(&str_const);
                str_global.set_constant(true);
                str_global.set_linkage(inkwell::module::Linkage::Private);

                str_global.as_pointer_value().into()
            },
        }
    }
}

pub fn _build_type_map(context: &'_ Context) -> HashMap<IRType, BasicTypeEnum<'_>> {
    let mut type_map = HashMap::new();
    type_map.insert(IRType::U8, ir_type_to_llm_type(context, &IRType::U8));
    type_map.insert(IRType::U16, ir_type_to_llm_type(context, &IRType::U16));
    type_map.insert(IRType::U32, ir_type_to_llm_type(context, &IRType::U32));
    type_map.insert(IRType::U64, ir_type_to_llm_type(context, &IRType::U64));
    type_map.insert(IRType::F32, ir_type_to_llm_type(context, &IRType::F32));
    type_map.insert(IRType::F64, ir_type_to_llm_type(context, &IRType::F64));
    type_map.insert(IRType::String, ir_type_to_llm_type(context, &IRType::String));

    type_map
}

pub fn ir_type_to_llm_type<'ctx>(context: &'ctx Context, ty: &IRType) -> BasicTypeEnum<'ctx> {
    match ty {
        IRType::U8 => context.i8_type().into(),
        IRType::U16 => context.i16_type().into(),
        IRType::U32 => context.i32_type().into(),
        IRType::U64 => context.i64_type().into(),
        IRType::F32 => context.f32_type().into(),
        IRType::F64 => context.f64_type().into(),
        IRType::Bool => context.bool_type().into(),
        IRType::String => context.ptr_type(AddressSpace::default()).into(),
    }
}

pub fn ir_type_to_llvm_value<'ctx>(context: &'ctx Context, ty: &IRType) -> BasicValueEnum<'ctx> {
    match ty {
        IRType::U8 => context.i8_type().const_int(0, true).into(),
        IRType::U16 => context.i16_type().const_int(0, true).into(),
        IRType::U32 => context.i32_type().const_int(0, true).into(),
        IRType::U64 => context.i64_type().const_int(0, true).into(),
        IRType::F32 => context.f32_type().const_float(0.0).into(),
        IRType::F64 => context.f64_type().const_float(0.0).into(),
        IRType::String => context.ptr_type(AddressSpace::default()).const_null().into(),
        IRType::Bool => unimplemented!(),
    }
}
