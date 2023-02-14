use std::io::Write;
use crate::ast::{AstNode, WasmWriter, WatWriter};

pub struct Call {
    name: String,
    arguments: Vec<Box<dyn AstNode>>
}

impl WatWriter for Call {
    fn write_wat(&self, write: &mut dyn Write) -> std::io::Result<()> {
        for arg in &self.arguments {
            arg.write_wat(write)?;
        }
        writeln!(write, "call ${}", self.name)?;
        Ok(())
    }
}

impl WasmWriter for Call {
    fn write_wasm(&self, _write: &mut dyn Write) -> std::io::Result<()> {
        todo!()
    }
}

impl AstNode for Call {}

impl Call {
    pub fn new(name: String, arguments: Vec<Box<dyn AstNode>>) -> Self {
        Self {
            name, arguments
        }
    }
}
