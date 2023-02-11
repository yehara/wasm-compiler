pub enum WasmType {
    I32
}

impl WasmType {

    pub fn code(&self) -> u8 {
        match &self {
            WasmType::I32 => 0x7f
        }
    }

}