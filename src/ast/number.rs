use std::io::Write;
use crate::ast::{AstNode, WasmWriter, WatWriter};

pub struct Number {
    value: i32
}

impl WatWriter for Number {
    fn write_wat(&self, write: &mut dyn Write) -> std::io::Result<()> {
        writeln!(write, "i32.const {}", self.value)?;
        Ok(())
    }
}

impl WasmWriter for Number {
    fn write_wasm(&self, write: &mut dyn Write) -> std::io::Result<()> {
        write.write(&vec![0x41])?;             // i32.const
        write.write(&vec![self.value as u8])?; // i32 literal
        Ok(())
    }
}

impl AstNode for Number {}

impl Number {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}
