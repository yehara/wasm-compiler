use std::io::{Write, Result};
use crate::ast::{WasmWriter, WatWriter};
use crate::ast::function::Function;

struct Module {
    functions: Vec<Box<Function>>,
}

impl WatWriter for Module {
    fn write_wat(&self, write: &mut dyn Write) -> Result<()>{
        writeln!(write, "(module")?;

        for func in self.functions.iter() {
            let _ = &func.write_wat(write)?;
        }

        writeln!(write, ")")?;
        Ok(())
    }
}

impl WasmWriter for Module {
    fn write_wasm(&self, write: &mut dyn Write) -> Result<()> {
        write.write(&vec![0x00, 0x61, 0x73, 0x6d])?; // WASM_BINARY_MAGIC
        write.write(&vec![0x01, 0x00, 0x00, 0x00])?; // WASM_BINARY_VERSION
        Ok(())
    }
}

#[test]
fn test() {
    let module = Module {
        functions: vec![Box::new(Function {})]
    };
    let mut write = std::io::stdout();
    let _ = module.write_wat(&mut write);
}
