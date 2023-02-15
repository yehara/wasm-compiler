use std::io::Write;
use crate::ast::{AstNode, WasmWriter, WatWriter};

pub struct ReturnNode {
    child: Box<dyn AstNode>
}

impl WatWriter for ReturnNode {
    fn write_wat(&self, write: &mut dyn Write) -> std::io::Result<()> {
        self.child.write_wat(write)?;
        writeln!(write, "return")?;
        Ok(())
    }
}

impl WasmWriter for ReturnNode {
    fn write_wasm(&self, write: &mut dyn Write) -> std::io::Result<()> {
        self.child.write_wasm(write)?;
        write.write(&vec![0x0f])?; // return
        Ok(())
    }
}

impl AstNode for ReturnNode {}

impl ReturnNode {
    pub fn new(child: Box<dyn AstNode>) -> Self {
        Self {
            child
        }
    }
}
