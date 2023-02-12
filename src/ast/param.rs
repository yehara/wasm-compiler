use crate::ast::WasmType;
use crate::ast::WasmType::I32;

pub struct Param {
    pub wtype: WasmType,
    pub name: String,
}

impl Param {
    pub fn new(name: String) -> Self {
        Self {
            name,
            wtype: I32
        }
    }
}