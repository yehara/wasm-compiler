use std::collections::HashSet;
use std::io::Write;
use crate::ast::{AstNode, WasmWriter, WatWriter};

pub struct Assign {
    lhs: Box<dyn AstNode>,
    rhs: Box<dyn AstNode>,
}

impl WatWriter for Assign {
    fn write_wat(&self, write: &mut dyn Write) -> std::io::Result<()> {

        match &self.lhs.as_variable() {
            Some(variable) => {
                self.rhs.write_wat(write)?;
                writeln!(write, "local.tee ${}", variable.name)?;
            },
            None => {
                panic!("左辺が変数ではありませんｎ")
            }
        }
        Ok(())
    }
}

impl WasmWriter for Assign {
    fn write_wasm(&self, _write: &mut dyn Write) -> std::io::Result<()> {
        todo!()
    }
}

impl AstNode for Assign {
    fn collect_locals(&self, _params: &mut HashSet<String>, vars: &mut HashSet<String>) {
        match &self.lhs.as_variable() {
            Some(variable) => {
                vars.insert(variable.name.to_string());
            },
            None => {}

        }
    }
}

impl Assign {
    pub fn new(lhs: Box<dyn AstNode>, rhs: Box<dyn AstNode>) -> Self {
        Self {
            lhs,
            rhs,
        }
    }
}
