mod module;
mod function;
mod param;
mod wasm_type;
mod builder;
mod block;
mod statement;
mod return_node;
mod expression;
mod assign;
mod number;
mod operator;
mod variable;
mod if_node;

use std::any::Any;
use std::collections::HashSet;
pub use module::Module;
pub use function::Function;
pub use wasm_type::WasmType;
pub use param::Param;
pub use block::Block;
pub use statement::Statement;
pub use return_node::ReturnNode;
pub use expression::Expression;
pub use assign::Assign;
pub use number::Number;
pub use operator::*;
pub use variable::Variable;
pub use if_node::IfNode;

use std::io::{Write, Result};

pub trait WatWriter {
    fn write_wat(&self, write: &mut dyn Write) -> Result<()>;
}
pub trait WasmWriter {
    fn write_wasm(&self, write: &mut dyn Write) -> Result<()>;
}
pub trait AstNode: WatWriter + WasmWriter + Any {
    fn as_variable(&self) -> Option<&Variable> {
        None
    }
    fn children(&self) -> Vec<&Box<dyn AstNode>> {
        vec![]
    }

    fn collect_locals(&self, params: &mut HashSet<String>, vars: &mut HashSet<String>) {
        for child in self.children().iter() {
            child.collect_locals(params, vars);
        }
    }

}
