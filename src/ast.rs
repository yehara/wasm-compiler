mod module;
mod function;

use std::io::{Write, Result};

pub trait WatWriter {
    fn write_wat(&self, write: &mut dyn Write) -> Result<()>;
}
pub trait WasmWriter {
    fn write_wasm(&self, write: &mut dyn Write) -> Result<()>;
}

