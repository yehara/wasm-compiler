use std::io::Write;
use crate::ast::{AstNode, WasmWriter, WatWriter};

pub struct Assign {
    lhs: Box<dyn AstNode>,
    rhs: Box<dyn AstNode>,
}

impl WatWriter for Assign {
    fn write_wat(&self, write: &mut dyn Write) -> std::io::Result<()> {
        todo!()
    }
}

impl WasmWriter for Assign {
    fn write_wasm(&self, write: &mut dyn Write) -> std::io::Result<()> {
        todo!()
    }
}

impl AstNode for Assign {}

impl Assign {
    pub fn new(lhs: Box<dyn AstNode>, rhs: Box<dyn AstNode>) -> Self {
        Self {
            lhs,
            rhs,
        }
    }
}
