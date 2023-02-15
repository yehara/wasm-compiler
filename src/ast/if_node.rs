use std::io::Write;
use crate::ast::{AstNode, Function, Module, WasmWriter, WatWriter};

pub struct IfNode {
    condition: Box<dyn AstNode>,
    then_block: Box<dyn AstNode>,
    else_block: Option<Box<dyn AstNode>>,
}

impl WatWriter for IfNode {
    fn write_wat(&self, write: &mut dyn Write) -> std::io::Result<()> {
        self.condition.write_wat(write)?;
        writeln!(write, "(if")?;
        writeln!(write, "(then")?;
        self.then_block.write_wat(write)?;
        writeln!(write, "drop")?;
        writeln!(write, ")")?;
        if let Some(els) = &self.else_block {
            writeln!(write, "(else")?;
            els.write_wat(write)?;
            writeln!(write, "drop")?;
            writeln!(write, ")")?;
        }
        writeln!(write, ")")?;
        writeln!(write, "i32.const 0")?;
        Ok(())
    }
}

impl WasmWriter for IfNode {
    fn write_wasm(&self, module: Option<&Module>, function: Option<&Function>, write: &mut dyn Write) -> std::io::Result<()> {
        self.condition.write_wasm(module, function, write)?;
        write.write(&vec![0x04])?; // if
        write.write(&vec![0x40])?; // block type
        self.then_block.write_wasm(module, function, write)?;
        write.write(&vec![0x1a])?; // drop
        if let Some(els) = &self.else_block {
            write.write(&vec![0x05])?; // else
            els.write_wasm(module, function, write)?;
            write.write(&vec![0x1a])?; // drop
        }
        write.write(&vec![0x0b])?; // end
        write.write(&vec![0x41, 0x00])?; // i32.const 0
        Ok(())
    }
}

impl AstNode for IfNode {
    fn children(&self) -> Vec<&Box<dyn AstNode>> {
        let mut children = vec![&self.condition, &self.then_block];
        if let Some(els) = &self.else_block {
            children.push(els);
        }
        children
    }
}

impl IfNode {
    pub fn new(condition: Box<dyn AstNode>,
               then_block: Box<dyn AstNode>,
               else_block: Option<Box<dyn AstNode>>) -> Self {
        Self {
            condition, then_block, else_block
        }
    }
}
