use std::io::Write;
use crate::ast::{AstNode, WasmWriter, WatWriter};

pub struct Variable {
    pub name: String
}

impl WatWriter for Variable {
    fn write_wat(&self, write: &mut dyn Write) -> std::io::Result<()> {
        writeln!(write, "local.get ${}", self.name)?;
        Ok(())
    }
}

impl WasmWriter for Variable {
    fn write_wasm(&self, _write: &mut dyn Write) -> std::io::Result<()> {
        todo!()
    }
}

impl AstNode for Variable {
    fn as_variable(&self) -> Option<&Variable> {
        Some(self)
    }
}

impl Variable {
    pub fn new(name: String) -> Self {
        Self {
            name
        }
    }
}
