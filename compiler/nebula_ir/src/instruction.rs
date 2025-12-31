use crate::value::{IRConst, IRPlace, IRTemp  , IRValue};

#[derive(Debug, Clone)]
pub enum IRInstruction {
    LoadConst {
        target: IRTemp,
        value: IRConst,
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