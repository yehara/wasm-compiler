use std::io::Write;
use crate::ast::{AstNode, Function, Module, WasmWriter, WatWriter};

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
    fn write_wasm(&self, _module: Option<&Module>, function: Option<&Function>, write: &mut dyn Write) -> std::io::Result<()> {
        let local_idx = function.unwrap().local_index.get(self.name.as_str());
        if let Some(&index) = local_idx {
            write.write(&vec![0x20, index as u8])?;
        } else {
            panic!("variable {} is not defined", self.name);
        }
        Ok(())
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
