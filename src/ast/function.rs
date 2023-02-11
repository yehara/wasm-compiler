use std::io::{Write, Result};
use crate::ast::{Param, WasmWriter, WatWriter};
use crate::ast::WasmType::I32;

pub struct Function {
    pub params: Vec<Param>,
}

impl Function {
    pub fn write_wasm_type(&self, write: &mut dyn Write) -> Result<()>{
        write.write(&vec![0x60])?; // func
        write.write(&vec![0x00])?; // section size (guess)
        for param in self.params.iter() {
            write.write(&vec![param.wtype.code()])?;
        }
        // result タイプは i32 固定
        write.write(&vec![0x01])?; // num results
        write.write(&vec![I32.code()])?; // num results

        Ok(())
    }
}

impl WatWriter for Function {
    fn write_wat(&self, write: &mut dyn Write) -> Result<()>{
        writeln!(write, "(func")?;
        writeln!(write, ")")?;
        Ok(())
    }
}

impl WasmWriter for Function {
    fn write_wasm(&self, write: &mut dyn Write) -> Result<()> {
        write.write(&vec![0x03])?; // SECTION CODE
        write.write(&vec![0x00])?; // SECTION SIZE (GUESS)
        Ok(())
    }
}
