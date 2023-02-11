use std::io::{Write, Result};

pub trait WatWriter {
    fn write_wat(&self, write: &mut dyn Write) -> Result<()>;
}
pub trait WasmWriter {
    fn write_wasm(&self, write: &mut dyn Write) -> Result<()>;
}

struct Module {
}

impl WatWriter for Module {
    fn write_wat(&self, write: &mut dyn Write) -> Result<()>{
        writeln!(write, "(module")?;
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
    let module = Module{};
    let mut write = std::io::stdout();
    let _ = module.write_wat(&mut write);
}
