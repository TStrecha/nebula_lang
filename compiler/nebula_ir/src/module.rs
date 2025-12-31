use crate::identification::GlobalId;
use crate::instruction::{IRInstruction};
use crate::value::{IRConst, IRType};

#[derive(Debug)]
pub struct IRGlobal {
    pub id: GlobalId,
    pub name: String,
    pub ty: IRType,
    pub initializer: Option<IRConst>,
}

#[derive(Debug)]
pub struct IRModule {
    name: String,
    globals: Vec<IRGlobal>,
    instructions: Vec<IRInstruction>,
}

impl IRModule {
    pub fn new(name: String) -> Self {
        Self {
            name,
            globals: vec![],
            instructions: vec![],
        }
    }

    pub fn push_instr(&mut self, instr: IRInstruction) {
        self.instructions.push(instr);
    }

    pub fn push_global(&mut self, global: IRGlobal) {
        self.globals.push(global);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn globals(&self) -> &Vec<IRGlobal> {
        &self.globals
    }

    pub fn instructions(&self) -> &Vec<IRInstruction> {
        &self.instructions
    }
}