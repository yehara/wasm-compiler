mod module;
mod function;
mod param;
mod wasm_type;

pub use module::Module;
pub use function::Function;
pub use wasm_type::WasmType;
pub use param::Param;

use std::io::{Write, Result};

pub trait WatWriter {
    fn write_wat(&self, write: &mut dyn Write) -> Result<()>;
}
pub trait WasmWriter {
    fn write_wasm(&self, write: &mut dyn Write) -> Result<()>;
}

