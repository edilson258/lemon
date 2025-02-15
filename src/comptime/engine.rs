use crate::ir;

use super::stack::Stack;

pub struct Engine {
	stack: Stack,
	ir: ir::IR,
}

impl Engine {
	pub fn new(ir: ir::IR) -> Self {
		Self { stack: Stack::new(), ir }
	}
}
