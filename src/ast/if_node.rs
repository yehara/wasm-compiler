use std::io::Write;
use crate::ast::{AstNode, WasmWriter, WatWriter};

pub struct IfNode {
    condition: Box<dyn AstNode>,
    then_block: Box<dyn AstNode>,
    else_block: Option<Box<dyn AstNode>>,
}

/**
fn gen_if(&self) {
    self.cond.as_ref().unwrap().gen();
    println!("    (if");
    println!("      (then");
    self.then.as_ref().unwrap().gen();
    println!("      drop");
    println!("      )");
    if let Some(els) = &self.els {
        println!("      (else");
        els.gen();
        println!("      drop");
        println!("      )");
    }
    println!("    )");
    println!("    i32.const 0");
}
*/
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
    fn write_wasm(&self, _write: &mut dyn Write) -> std::io::Result<()> {
        todo!()
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
