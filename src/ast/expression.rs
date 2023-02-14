use std::io::Write;
use crate::ast::{AstNode, WasmWriter, WatWriter};

pub struct Expression {
}

impl WatWriter for Expression {
    fn write_wat(&self, _write: &mut dyn Write) -> std::io::Result<()> {
        todo!()
    }
}

impl WasmWriter for Expression {
    fn write_wasm(&self, _write: &mut dyn Write) -> std::io::Result<()> {
        todo!()
    }
}

impl AstNode for Expression {}

