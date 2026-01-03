use nebula_ir::identification::GlobalId;
use nebula_ir::instruction::IRInstruction;
use nebula_ir::module::IRGlobal;
use nebula_ir::value::{IRLiteral, IRPlace, IRTemp, IRType, IRValue};
use nebula_tst::{Place, Type, TypedLiteral};
use nebula_tst::item::TypedExpr;
use nebula_tst::ty::BuiltinType;
use crate::builder::{create_global_id, create_local_id, create_temp_id, IRModuleBuilder};

impl IRModuleBuilder {

    pub fn handle_expr(&mut self, expr: TypedExpr, place: Option<IRPlace>) {
        match expr {
            TypedExpr::VarDecl { name, ty, value } => {
                let global_id: GlobalId = (self.module.globals().len() as u32).into();

                let ir_type = type_to_ir_type(&ty);
                let place = IRPlace::Global(global_id.clone());

                self.handle_expr(*value, Some(place));

                let global_def = IRGlobal {
                    id: global_id,
                    name,
                    ty: ir_type,
                };

                self.module.push_global(global_def);
            }
            TypedExpr::Lit(lit) => {
                if let Some(place) = place {
                    let instr_value = literal_to_ir_literal(lit);

                    self.module.push_instr(IRInstruction::StoreLiteral {
                        target: place,
                        value: instr_value,
                    });
                }
            }
            TypedExpr::Ident { symbol, ty } => {
                let ir_type = type_to_ir_type(&ty);
                let ir_place = self.module.globals().iter().filter(|global| global.name == symbol.name)
                    .next()
                    .map(|global| IRPlace::Global(global.id))
                    .unwrap_or_else(|| place_to_ir_place(&symbol.place));

                let mut should_store = true;

                let target = if let Some(place) = place.clone() {
                    if let IRPlace::Temp(id) = place {
                        should_store = false;
                        IRTemp { id, ty: ir_type }
                    } else {
                        IRTemp { id: create_temp_id(), ty: ir_type }
                    }
                } else {
                    IRTemp { id: create_temp_id(), ty: ir_type }
                };

                let instr = IRInstruction::Load {
                    target: target.clone(),
                    from: ir_place,
                };

                self.module.push_instr(instr);

                if should_store {
                    if let Some(place) = place {
                        let instr = IRInstruction::Store {
                            to: place,
                            value: IRValue::Temp(target),
                        };

                        self.module.push_instr(instr);
                    }
                }
            },

            TypedExpr::Return { value, .. } => {
                let temp_id = create_temp_id();

                self.handle_expr(*value, Some(IRPlace::Temp(temp_id)));
                self.module.push_instr(IRInstruction::Return { from: IRPlace::Temp(temp_id) });
            }
        }
    }
}

pub fn type_to_ir_type(ty: &Type) -> IRType {
    match ty {
        Type::Builtin(builtin) => match builtin {
            BuiltinType::Bool =>    IRType::Bool,
            BuiltinType::U8 =>      IRType::U8,
            BuiltinType::U16 =>     IRType::U16,
            BuiltinType::U32 =>     IRType::U32,
            BuiltinType::U64 =>     IRType::U64,
            BuiltinType::F32 =>     IRType::F32,
            BuiltinType::F64 =>     IRType::F64,
            BuiltinType::String =>  IRType::String
        }
        Type::Named(..) => unimplemented!(),
        Type::Void => unimplemented!()
    }
}

pub fn place_to_ir_place(place: &Place) -> IRPlace {
    match place {
        Place::Global => IRPlace::Global(create_global_id()),
        Place::Local => IRPlace::Local(create_local_id()),
    }
}

pub fn literal_to_ir_literal(lit: TypedLiteral) -> IRLiteral {
    match lit {
        TypedLiteral::U8(val) => IRLiteral::U8(val),
        TypedLiteral::U16(val) => IRLiteral::U16(val),
        TypedLiteral::U32(val) => IRLiteral::U32(val),
        TypedLiteral::U64(val) => IRLiteral::U64(val),
        TypedLiteral::F32(val) => IRLiteral::F32(val),
        TypedLiteral::F64(val) =>  IRLiteral::F64(val),
        TypedLiteral::Bool(_) => unimplemented!(),
        TypedLiteral::String(val) => IRLiteral::String(val),
    }
}