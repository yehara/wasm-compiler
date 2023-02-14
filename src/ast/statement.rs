use std::io::Write;
use crate::ast::{AstNode, WasmWriter, WatWriter};

pub struct Statement {
    child: Box<dyn AstNode>
}

impl WatWriter for Statement {
    fn write_wat(&self, write: &mut dyn Write) -> std::io::Result<()> {
        self.child.write_wat(write)?;
        Ok(())
    }
}

impl WasmWriter for Statement {
    fn write_wasm(&self, _write: &mut dyn Write) -> std::io::Result<()> {
        todo!()
    }
}

impl AstNode for Statement {}
