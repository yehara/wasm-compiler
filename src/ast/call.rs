use std::io::Write;
use crate::ast::{AstNode, Function, Module, WasmWriter, WatWriter};

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
    fn write_wasm(&self, module: Option<&Module>, function: Option<&Function>, write: &mut dyn Write) -> std::io::Result<()> {
        for arg in &self.arguments {
            arg.write_wasm(module, function, write)?;
        }
        let func_idx = module.unwrap().get_function_index(self.name.as_str());
        write.write(&[0x10, func_idx as u8])?; // call
        Ok(())
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
