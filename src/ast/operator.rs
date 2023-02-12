use std::io::Write;
use crate::ast::{AstNode, WasmWriter, WatWriter};

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
        match &self.kind {
            BiOpKind::Add => {
                writeln!(write, "i32.add")?;
            }
            BiOpKind::Sub => {
                writeln!(write, "i32.sub")?;
            }
            BiOpKind::Mult => {
                writeln!(write, "i32.mul")?;
            }
            BiOpKind::Div => {
                writeln!(write, "i32.div_s")?;
            }
            BiOpKind::Equal => {
                writeln!(write, "i32.eq")?;
            }
            BiOpKind::NotEqual => {
                writeln!(write, "i32.ne")?;
            }
            BiOpKind::GreaterThan => {
                writeln!(write, "i32.gt_s")?;
            }
            BiOpKind::GreaterThanOrEqual => {
                writeln!(write, "i32.ge_s")?;
            }
            BiOpKind::LessThan => {
                writeln!(write, "i32.lt_s")?;
            }
            BiOpKind::LessThanOrEqual => {
                writeln!(write, "i32.le_s")?;
            }
        }
        Ok(())
    }
}

impl WasmWriter for BiOperator {
    fn write_wasm(&self, write: &mut dyn Write) -> std::io::Result<()> {
        todo!()
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
