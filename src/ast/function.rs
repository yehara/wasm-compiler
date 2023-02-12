use std::io::{Write, Result};
use crate::ast::{AstNode, Param, WasmWriter, WatWriter};
use crate::ast::WasmType::I32;

pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Box<dyn AstNode>
}

impl Function {

    pub fn new(name: String, params: Vec<Param>, body: Box<dyn AstNode>) -> Self {
        Self { name, params,  body }
    }

    pub fn write_wasm_type(&self, write: &mut dyn Write) -> Result<()>{
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
        writeln!(write, "(result i32)")?;
        self.body.write_wat(write)?;
        writeln!(write, ")")?;
        Ok(())
    }
}

impl WasmWriter for Function {
    fn write_wasm(&self, write: &mut dyn Write) -> Result<()> {
        let mut buf : Vec<u8> = Vec::new();

        // todo: write function body

        write.write(&vec![buf.len() as u8])?; // section size
        write.write(&buf)?;
        Ok(())
    }
}
