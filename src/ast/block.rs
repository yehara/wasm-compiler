use std::io::Write;
use crate::ast::{AstNode, WasmWriter, WatWriter};

pub struct Block {
    statements: Vec<Box<dyn AstNode>>
}

impl WatWriter for Block {
    fn write_wat(&self, write: &mut dyn Write) -> std::io::Result<()> {
        for statement in &self.statements {
            statement.write_wat(write)?;
            writeln!(write, "drop")?;
        }
        writeln!(write, "i32.const 0")?;
        Ok(())
    }
}

impl WasmWriter for Block {
    fn write_wasm(&self, write: &mut dyn Write) -> std::io::Result<()> {
        for statement in &self.statements {
            statement.write_wasm(write)?;
            write.write(&vec![0x1a])?; // drop
        }
        write.write(&vec![0x41, 0x00])?; // i32.const 0
        Ok(())
    }
}

impl AstNode for Block {
    fn children(&self) -> Vec<&Box<dyn AstNode>> {
        self.statements.iter().map(|b| b).collect()
    }
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

