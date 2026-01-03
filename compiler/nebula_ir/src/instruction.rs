use crate::value::{IRLiteral, IRPlace, IRTemp, IRValue};

#[derive(Debug, Clone)]
pub enum IRInstruction {
    LoadLiteral {
        target: IRPlace,
        value: IRLiteral,
    },

    Load {
        target: IRTemp,
        from: IRPlace,
    },

    Store {
        to: IRPlace,
        value: IRValue,
    },
}