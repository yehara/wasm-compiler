use std::collections::{HashMap, HashSet};
use std::io::{Write, Result};
use crate::ast::{AstNode, Module, Param, WasmWriter, WatWriter};
use crate::ast::WasmType::I32;

pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Box<dyn AstNode>,
    locals: Vec<String>,
    pub local_index: HashMap<String, usize>,
}

impl Function {

    pub fn new(name: String, params: Vec<Param>, body: Box<dyn AstNode>) -> Self {
        let mut function = Self { name, params,  body, locals: vec![], local_index: HashMap::new() };
        let mut param_set : HashSet<String> = HashSet::new();
        for param in function.params.iter() {
            param_set.insert(param.name.to_string());
        }
        let mut vars = vec![];
        function.collect_locals(&mut param_set, &mut vars);
        function.locals = vars;
        for i in 0..function.locals.len() {
            function.local_index.insert(function.locals[i].to_string(), i);
        }
        function
    }

    pub fn write_wasm_type(&self, write: &mut dyn Write) -> Result<()>{
        //self.collect_locals(&mut self);
        write.write(&vec![0x60])?; // func
        write.write(&vec![self.params.len() as u8])?; // num params
        for param in self.params.iter() {
            write.write(&vec![param.wtype.code()])?; // param type
        }
        // result タイプは i32 固定
        write.write(&vec![0x01])?; // num results
        write.write(&vec![I32.code()])?; // result type
        Ok(())
    }

}

impl WatWriter for Function {
    fn write_wat(&self, write: &mut dyn Write) -> Result<()>{
        writeln!(write, "(func ${}", &self.name)?;
        for param in self.params.iter() {
            writeln!(write, "    (param ${} i32)", param.name)?;
        }
        writeln!(write, "(result i32)")?;
        for local in &self.locals {
            writeln!(write, "    (local ${} i32)", local)?;
        }
        self.body.write_wat(write)?;
        writeln!(write, ")")?;

        Ok(())
    }
}

impl WasmWriter for Function {
    fn write_wasm(&self, module: Option<&Module>, _function: Option<&Function>, write: &mut dyn Write) -> Result<()> {
        let mut buf : Vec<u8> = Vec::new();
        buf.write(&vec![0x01])?; // local decl count
        buf.write(&vec![self.locals.len() as u8])?; // local type count
        buf.write(&vec![0x7f])?; // i32
        self.body.write_wasm(module, Some(self), &mut buf)?; // function body
        buf.write(&vec![0x0b])?; //end
        write.write(&vec![buf.len() as u8])?; // function body size
        write.write(&buf)?;
        Ok(())
    }
}

impl AstNode for Function {
    fn children(&self) -> Vec<&Box<dyn AstNode>> {
        vec![&self.body]
    }
}