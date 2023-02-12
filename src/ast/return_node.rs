use std::io::Write;
use crate::ast::{AstNode, Expression, WasmWriter, WatWriter};

pub struct ReturnNode {
    expression: Expression
}

impl WatWriter for ReturnNode {
    fn write_wat(&self, write: &mut dyn Write) -> std::io::Result<()> {
        write.write("return".as_bytes())?;
        Ok(())
    }
}

impl WasmWriter for ReturnNode {
    fn write_wasm(&self, write: &mut dyn Write) -> std::io::Result<()> {
        todo!()
    }
}

impl AstNode for ReturnNode {}

impl ReturnNode {
    pub fn new(expression: Expression) -> Self {
        Self {
            expression
        }
    }
}
