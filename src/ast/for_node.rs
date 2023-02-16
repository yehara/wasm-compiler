use std::io::Write;
use crate::ast::{AstNode, Function, Module, node_id, WasmWriter, WatWriter};

pub struct ForNode {
    id: u32,
    initialize: Option<Box<dyn AstNode>>,
    condition: Option<Box<dyn AstNode>>,
    increment: Option<Box<dyn AstNode>>,
    body: Box<dyn AstNode>,
}

impl WatWriter for ForNode {
    fn write_wat(&self, write: &mut dyn Write) -> std::io::Result<()> {
        if let Some(init) = &self.initialize {
            init.write_wat(write)?;
            writeln!(write, "drop")?;
        }
        writeln!(write, "(block $block{}", self.id)?;
        writeln!(write, "(loop $loop{}", self.id)?;
        if let Some(cond) = &self.condition {
            cond.write_wat(write)?;
            writeln!(write, "i32.const 0")?;
            writeln!(write, "i32.eq")?;
            writeln!(write, "br_if $block{}", self.id)?;
        }
        self.body.write_wat(write)?;
        if let Some(inc) = &self.increment {
            inc.write_wat(write)?;
            writeln!(write, "drop")?;
        }
        writeln!(write, "br $loop{}", self.id)?;
        writeln!(write, ")")?;
        writeln!(write, ")")?;
        writeln!(write, "i32.const 0")?;
        Ok(())
    }
}

impl WasmWriter for ForNode {
    fn write_wasm(&self, module: Option<&Module>, function: Option<&Function>, write: &mut dyn Write) -> std::io::Result<()> {
        if let Some(init) = &self.initialize {
            init.write_wasm(module, function, write)?;
            write.write(&vec![0x1a])?; // drop
        }
        write.write(&vec![0x02, 0x40])?; // block
        write.write(&vec![0x03, 0x40])?; // loop
        if let Some(cond) = &self.condition {
            cond.write_wasm(module, function, write)?;
            write.write(&vec![0x41, 0x00])?; // i32.const 0
            write.write(&vec![0x46])?; // i32.eq
            write.write(&vec![0x0d, 0x01])?; // br_if (block)
        }
        self.body.write_wasm(module, function, write)?;
        if let Some(inc) = &self.increment {
            inc.write_wasm(module, function, write)?;
            write.write(&vec![0x1a])?; // drop
        }
        write.write(&vec![0x0c, 0x00])?; // br (loop)
        write.write(&vec![0x0b])?; // end
        write.write(&vec![0x0b])?; // end
        write.write(&vec![0x41, 0x00])?; // i32.const 0
        Ok(())
    }
}

impl AstNode for ForNode {
    fn children(&self) -> Vec<&Box<dyn AstNode>> {
        let mut children = vec![&self.body];
        if let Some(init) = &self.initialize {
            children.push(init);
        }
        if let Some(inc) = &self.increment {
            children.push(inc);
        }
        if let Some(cond) = &self.condition {
            children.push(cond);
        }
        children
    }
}

impl ForNode {
    pub fn new(initialize: Option<Box<dyn AstNode>>,
               condition: Option<Box<dyn AstNode>>,
               increment: Option<Box<dyn AstNode>>,
               body: Box<dyn AstNode>
    ) -> Self {
        Self {
            id: node_id(), initialize, condition, increment, body
        }
    }
}
