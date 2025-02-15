use rustc_hash::FxHashMap;

use crate::ir::{self};
#[derive(Debug, Clone)]

pub struct Frame {
	pub locals: FxHashMap<String, ir::IrBasicValue>,
	pub blocks: Vec<ir::IrBlock>,
	pub fn_name: String,
	pub stack: Vec<ir::IrBasicValue>,
	pub index: usize,
}

impl Frame {
	pub fn new(fn_name: &str) -> Self {
		Self {
			locals: FxHashMap::default(),
			blocks: Vec::new(),
			stack: Vec::new(),
			index: 0,
			fn_name: fn_name.to_string(),
		}
	}

	pub fn push(&mut self, value: ir::IrBasicValue) {
		self.stack.push(value);
	}
	pub fn pop(&mut self) -> ir::IrBasicValue {
		self.stack.pop().unwrap()
	}
	pub fn get(&self, name: &str) -> Option<ir::IrBasicValue> {
		self.locals.get(name).cloned()
	}
	pub fn set(&mut self, name: &str, value: ir::IrBasicValue) {
		self.locals.insert(name.to_string(), value);
	}
}
