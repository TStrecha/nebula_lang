use crate::value::{IRLiteral, IRPlace, IRTemp};

#[derive(Debug, Clone)]
pub enum IRInstruction {
    StoreLiteral {
        target: IRPlace,
        value: IRLiteral,
    },

    Load {
        target: IRTemp,
        from: IRPlace,
    },

    Return {
        from: IRTemp,
    }
}