use nebula_ast::item::{Expr, Literal};
use nebula_tst::{Place, Symbol, TypedLiteral};
use nebula_tst::item::TypedExpr;
use crate::tst::parser::TypedAstParser;

impl TypedAstParser {

    pub fn handle_expr(&mut self, expr: Expr) -> TypedExpr {
        match expr {
            Expr::VarDecl { name, ty, value } => {
                let symbol = Symbol {
                    name: name.clone(),
                    place: Place::Global,
                    ty: ty.clone().into()
                };

                self.symbols.insert(name.clone(), symbol);

                TypedExpr::VarDecl {
                    name,
                    ty: ty.into(),
                    value: Box::from(self.handle_expr(*value)),
                }
            }

            Expr::Lit(lit) => {
                match lit {
                    Literal::Number(val) => {
                        let lit = match val {
                            x if x < u8::MAX as u128 => TypedLiteral::U8(val as u8),
                            x if x < u16::MAX as u128 => TypedLiteral::U16(val as u16),
                            x if x < u32::MAX as u128 => TypedLiteral::U32(val as u32),
                            x if x < u64::MAX as u128 => TypedLiteral::U64(val as u64),
                            _ => unimplemented!()
                        };
                        TypedExpr::Lit(lit)
                    },
                    Literal::Decimal(val) => {
                        let lit = match val {
                            x if x < f32::MAX as f64 => TypedLiteral::F32(val as f32),
                            x if x < f64::MAX => TypedLiteral::F64(val),
                            _ => unimplemented!()
                        };
                        TypedExpr::Lit(lit)
                    }
                    Literal::StringLit(val) => TypedExpr::Lit(TypedLiteral::String(val)),
                }
            }
            Expr::Ident(name) => {
                let symbol = self.symbols.get(&name);

                if let Some(symbol) = symbol {
                    TypedExpr::Ident {
                        symbol: symbol.clone(),
                        ty: symbol.ty.clone()
                    }
                } else {
                    panic!("Symbol not found {}", name)
                }
            }
        }
    }
}