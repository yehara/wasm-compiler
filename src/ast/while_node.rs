use std::io::Write;
use crate::ast::{AstNode, node_id, WasmWriter, WatWriter};

pub struct WhileNode {
    id: u32,
    condition: Box<dyn AstNode>,
    body: Box<dyn AstNode>,
}

impl WatWriter for WhileNode {
    fn write_wat(&self, write: &mut dyn Write) -> std::io::Result<()> {
        writeln!(write, "(block $block{}", self.id)?;
        writeln!(write, "(loop $loop{}", self.id)?;
        self.condition.write_wat(write)?;
        writeln!(write, "i32.const 0")?;
        writeln!(write, "i32.eq")?;
        writeln!(write, "br_if $block{}", self.id)?;
        self.body.write_wat(write)?;
        writeln!(write, "drop")?;
        writeln!(write, "br $loop{}", self.id)?;
        writeln!(write, ")")?;
        writeln!(write, ")")?;
        writeln!(write, "i32.const 0")?;
        Ok(())
    }
}

impl WasmWriter for WhileNode {
    fn write_wasm(&self, _write: &mut dyn Write) -> std::io::Result<()> {
        todo!()
    }
}

impl AstNode for WhileNode {
    fn children(&self) -> Vec<&Box<dyn AstNode>> {
        vec![&self.condition, &self.body]
    }
}

impl WhileNode {
    pub fn new(condition: Box<dyn AstNode>,
               body: Box<dyn AstNode>) -> Self {
        Self {
            id: node_id(),
            condition, body
        }
    }
}
