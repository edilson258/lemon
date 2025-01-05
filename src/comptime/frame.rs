use lemon::report::throw_engine_error;

use crate::ir;
#[derive(Debug, Clone)]
pub enum Value {
	Int(i64),
	Float(f64),
	Bool(bool),
	String(String),
	Char(char),
	Register(ir::Register),
	Zero,
}
pub struct Frame {
	pub registers: Vec<Value>,
	pub return_value: Option<Value>,
}

impl Frame {
	pub fn new(size: usize) -> Self {
		let registers = Vec::from_iter(vec![Value::Zero; size]);
		Self { registers, return_value: None }
	}
	pub fn set_register(&mut self, reg: &ir::Register, value: Value) {
		self.registers[reg.as_usize()] = value;
	}

	pub fn get_register(&self, reg: &ir::Register) -> Value {
		if let Some(value) = self.registers.get(reg.as_usize()) {
			if let Value::Zero = value {
				throw_engine_error("register is zero");
			}
			value.clone()
		} else {
			throw_engine_error("register not found");
		}
	}

	pub fn set_return_value(&mut self, value: Value) {
		self.return_value = Some(value);
	}

	pub fn get_return_value(&self) -> Option<&Value> {
		self.return_value.as_ref()
	}
}
