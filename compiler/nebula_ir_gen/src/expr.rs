use nebula_ast::item::{BuiltinType, Expr, Literal, Type};
use nebula_ir::instruction::IRInstruction;
use nebula_ir::module::IRGlobal;
use nebula_ir::value::{IRConst, IRPlace, IRTemp, IRType, IRValue};
use crate::builder::IRModuleBuilder;

impl IRModuleBuilder {
    pub fn handle_expr(&mut self, expr: Expr, temp: Option<IRTemp>) {
        match expr {
            Expr::VarDecl { name, value_type, value } => {
                let ir_type = IRType::from(&value_type);

                let mut initializer: Option<IRConst> = None;

                if let Expr::Lit(literal) = *value {

                    let ir_const = match (&literal, &ir_type) {
                        (Literal::Number(val), IRType::U8) => IRConst::U8(*val as u8),
                        (Literal::Number(val), IRType::U16) => IRConst::U16(*val as u16),
                        (Literal::Number(val), IRType::U32) => IRConst::U32(*val as u32),
                        (Literal::Number(val), IRType::U64) => IRConst::U64(*val as u64),

                        (Literal::Decimal(val), IRType::F32) => IRConst::F32(*val),
                        (Literal::Decimal(val), IRType::F64) => IRConst::F64(*val),

                        (Literal::StringLit(val), IRType::String) => IRConst::String(val.clone()),

                        _ => panic!("Type mismatch: {:?} can't be {:?}", literal, value_type),
                    };

                    initializer = Some(ir_const)
                } else {
                    let temp = IRTemp {
                        id: 1.into(),
                        ty: IRType::U8,
                    };

                    self.handle_expr(*value, Some(temp.clone()));
                };

                let global_def = IRGlobal {
                    id: (self.module.globals().len() as u32).into(),
                    name,
                    ty: ir_type,
                    initializer,
                };

                self.module.push_global(global_def);
            }
            Expr::Lit(lit) => {
                if let Some(temp) = temp {
                    let instr_value = match lit {
                        Literal::Number(val) => IRConst::U32(val as u32),
                        Literal::Decimal(val) => IRConst::F64(val),
                        Literal::StringLit(val) => IRConst::String(val),
                    };

                    self.module.push_instr(IRInstruction::LoadConst {
                        target: temp,
                        value: instr_value,
                    });
                }
            }
            Expr::Ident(String) => {
                let target = IRTemp { id: 2.into(), ty: IRType::U8 };

                let instr = IRInstruction::Load {
                    target: target.clone(),
                    from: IRPlace::Local(1.into()),
                };

                self.module.push_instr(instr);

                let instr = IRInstruction::Store {
                    to: IRPlace::Local(1.into()),
                    value: IRValue::Temp(target),
                };

                self.module.push_instr(instr);
            }
        }
    }
}
