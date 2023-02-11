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

    pub fn write_wasm_function_section(&self, write: &mut dyn Write) -> Result<()>{
        write.write(&vec![0x03])?; // section code

        let mut buf : Vec<u8> = Vec::new();
        buf.write(&vec![self.functions.len() as u8])?; // num functions
        for i in 0.. self.functions.len() {
            buf.write(&vec![i as u8])?; // function signature index
        }
        write.write(&vec![buf.len() as u8])?; // section size
        write.write(&buf)?;
        Ok(())
    }

    pub fn write_wasm_export_section(&self, write: &mut dyn Write) -> Result<()> {
        write.write(&vec![0x07])?; // section code

        let mut buf : Vec<u8> = Vec::new();
        buf.write(&vec![0x01])?; // num exports (1固定)
        let main_name = "main";
        buf.write(&vec![main_name.len() as u8])?; // string length
        buf.write(main_name.as_bytes())?; // export name
        buf.write(&vec![0x00])?; // export kind

        let main_func = self.functions.iter().enumerate().find(|(_, function)| {
            function.name == main_name
        });

        match main_func {
            Some((i, _)) => {
                buf.write(&vec![i as u8])?; // export func index
            },
            None => {
                panic!("function `main` not found");
            }
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
        self.write_wasm_function_section(write)?;
        self.write_wasm_export_section(write)?;
        Ok(())
    }
}

#[test]
fn test_wat() {
    let module = Module {
        functions: vec![Box::new(Function {
            name: "main".to_string(),
            params: vec![Param{wtype: I32, name: "123".to_string()}]}
        )]
    };
    let mut write = std::io::stdout();
    let _ = module.write_wat(&mut write);
}

#[test]
fn test_wasm() {
    let module = Module {
        functions: vec![Box::new(Function {
            name: "main".to_string(),
            params: vec![Param{wtype: I32, name: "123".to_string()}]}
        )]
    };
    let mut buf = vec![];
    let _ = module.write_wasm(&mut buf);
    println!("{:x?}", buf);

}
