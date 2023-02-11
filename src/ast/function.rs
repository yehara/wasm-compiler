use std::io::{Write, Result};
use crate::ast::{WasmWriter, WatWriter};

pub struct Function {
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
