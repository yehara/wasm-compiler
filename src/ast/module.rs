use std::io::{Write, Result};
use crate::ast::{Param, WasmWriter, WatWriter};
use crate::ast::Function;
use crate::ast::WasmType::I32;

pub struct Module {
    functions: Vec<Box<Function>>,
}

impl Module {

    pub fn write_wasm_type_section(&self, write: &mut dyn Write) -> Result<()>{
        write.write(&vec![0x01])?; // section code

        let mut buf : Vec<u8> = Vec::new();
        buf.write(&vec![self.functions.len() as u8])?; // num types
        for function in self.functions.iter() {
            function.write_wasm_type(&mut buf)?;
        }
        write.write(&vec![buf.len() as u8])?; // section size
        write.write(&buf)?;
        Ok(())
    }

}

impl WatWriter for Module {
    fn write_wat(&self, write: &mut dyn Write) -> Result<()>{
        writeln!(write, "(module")?;
        for func in self.functions.iter() {
            let _ = func.write_wat(write)?;
        }
        writeln!(write, ")")?;
        Ok(())
    }
}

impl WasmWriter for Module {
    fn write_wasm(&self, write: &mut dyn Write) -> Result<()> {
        write.write(&vec![0x00, 0x61, 0x73, 0x6d])?; // WASM_BINARY_MAGIC
        write.write(&vec![0x01, 0x00, 0x00, 0x00])?; // WASM_BINARY_VERSION
        self.write_wasm_type_section(write)?;
        Ok(())
    }
}

#[test]
fn test_wat() {
    let module = Module {
        functions: vec![Box::new(Function { params: vec![Param{wtype: I32, name: "123".to_string()}]})]
    };
    let mut write = std::io::stdout();
    let _ = module.write_wat(&mut write);
}

#[test]
fn test_wasm() {
    let module = Module {
        functions: vec![Box::new(Function { params: vec![Param{wtype: I32, name: "123".to_string()}]})]
    };
    let mut buf = vec![];
    let _ = module.write_wasm(&mut buf);
    println!("{:x?}", buf);

}
