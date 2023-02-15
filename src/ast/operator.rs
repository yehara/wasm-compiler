use std::io::Write;
use crate::ast::{AstNode, Function, Module, WasmWriter, WatWriter};

pub enum BiOpKind {
    Add,
    Sub,
    Mult,
    Div,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

pub struct BiOperator {
    kind: BiOpKind,
    lhs: Box<dyn AstNode>,
    rhs: Box<dyn AstNode>,
}

impl WatWriter for BiOperator {
    fn write_wat(&self, write: &mut dyn Write) -> std::io::Result<()> {
        self.lhs.write_wat(write)?;
        self.rhs.write_wat(write)?;
        let operator = match &self.kind {
            BiOpKind::Add => "add",
            BiOpKind::Sub => "sub",
            BiOpKind::Mult => "mul",
            BiOpKind::Div => "div_s",
            BiOpKind::Equal => "eq",
            BiOpKind::NotEqual => "ne",
            BiOpKind::GreaterThan => "gt_s",
            BiOpKind::GreaterThanOrEqual => "ge_s",
            BiOpKind::LessThan => "lt_s",
            BiOpKind::LessThanOrEqual => "le_s"
        };
        writeln!(write, "i32.{}", operator)?;
        Ok(())
    }
}

impl WasmWriter for BiOperator {
    fn write_wasm(&self, module: Option<&Module>, function: Option<&Function>, write: &mut dyn Write) -> std::io::Result<()> {
        self.lhs.write_wasm(module, function, write)?;
        self.rhs.write_wasm(module, function, write)?;
        let operator:u8 = match &self.kind {
            BiOpKind::Add => 0x6a,
            BiOpKind::Sub => 0x6b,
            BiOpKind::Mult => 0x6c,
            BiOpKind::Div => 0x6d,
            BiOpKind::Equal => 0x46,
            BiOpKind::NotEqual => 0x47,
            BiOpKind::GreaterThan => 0x4a,
            BiOpKind::GreaterThanOrEqual => 0x4e,
            BiOpKind::LessThan => 0x48,
            BiOpKind::LessThanOrEqual => 0x4c,
        };
        write.write(&[operator])?;
        Ok(())
    }
}

impl AstNode for BiOperator {}

impl BiOperator {
    pub fn new(kind: BiOpKind, lhs: Box<dyn AstNode>, rhs: Box<dyn AstNode>) -> Self {
        Self {
            kind,
            lhs,
            rhs
        }
    }
}
