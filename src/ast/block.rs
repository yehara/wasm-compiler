use std::io::Write;
use crate::ast::{AstNode, WasmWriter, WatWriter};

pub struct Block {
    statements: Vec<Box<dyn AstNode>>
}

impl WatWriter for Block {
    fn write_wat(&self, write: &mut dyn Write) -> std::io::Result<()> {
        for statement in &self.statements {
            statement.write_wat(write)?;
        }
        Ok(())
    }
}

impl WasmWriter for Block {
    fn write_wasm(&self, write: &mut dyn Write) -> std::io::Result<()> {
        todo!()
    }
}

impl AstNode for Block {

}

impl Block {
    pub fn new() -> Self {
        Self {
            statements: vec![]
        }
    }

    pub fn add_statement(&mut self, statement: Box<dyn AstNode>) {
        self.statements.push(statement);
    }
}

