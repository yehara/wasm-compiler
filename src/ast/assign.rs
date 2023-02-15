use std::collections::HashSet;
use std::io::Write;
use crate::ast::{AstNode, Function, Module, Variable, WasmWriter, WatWriter};

pub struct Assign {
    lhs: Box<Variable>,
    rhs: Box<dyn AstNode>,
}

impl WatWriter for Assign {
    fn write_wat(&self, write: &mut dyn Write) -> std::io::Result<()> {
        self.rhs.write_wat(write)?;
        writeln!(write, "local.tee ${}", self.lhs.name)?;
        Ok(())
    }
}

impl WasmWriter for Assign {
    fn write_wasm(&self, module: Option<&Module>, function: Option<&Function>, write: &mut dyn Write) -> std::io::Result<()> {
        self.rhs.write_wasm(module, function, write)?;
        let &local_idx = function.unwrap().local_index.get(self.lhs.name.as_str()).unwrap();
        write.write(&vec![0x22, local_idx as u8])?; // local.tee local_idx
        Ok(())
    }
}

impl AstNode for Assign {
    fn collect_locals(&self, _params: &mut HashSet<String>, locals: &mut Vec<String>) {
        if !locals.contains(&self.lhs.name) {
            locals.push(self.lhs.name.to_string());
        }
    }
}

impl Assign {
    pub fn new(lhs: Box<dyn AstNode>, rhs: Box<dyn AstNode>) -> Self {
        match &lhs.as_variable() {
            Some(variable) => {
                Self {
                    lhs: Box::new(Variable::new(variable.name.to_string())),
                    rhs,
                }
            },
            None => {
                panic!("左辺が変数ではありませんｎ")
            }
        }
    }
}
