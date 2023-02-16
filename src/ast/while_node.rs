use std::io::Write;
use crate::ast::{AstNode, Function, Module, node_id, WasmWriter, WatWriter};

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
    fn write_wasm(&self, module: Option<&Module>, function: Option<&Function>, write: &mut dyn Write) -> std::io::Result<()> {
        write.write(&vec![0x02, 0x40])?; // block
        write.write(&vec![0x03, 0x40])?; // loop
        self.condition.write_wasm(module, function, write)?;
        write.write(&vec![0x41, 0x00])?; // i32.const 0
        write.write(&vec![0x46])?; // i32.eq
        write.write(&vec![0x0d, 0x01])?; // br_if (block)
        self.body.write_wasm(module, function, write)?;
        write.write(&vec![0x1a])?; // drop
        write.write(&vec![0x0c, 0x00])?; // br (loop)
        write.write(&vec![0x0b])?; // end
        write.write(&vec![0x0b])?; // end
        write.write(&vec![0x41, 0x00])?; // i32.const 0
        Ok(())
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
