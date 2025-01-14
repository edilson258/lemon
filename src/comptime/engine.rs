use std::{
	collections::HashMap,
	ops::{Div, Mul, Rem},
};

use super::{frame::Value, stack::Stack};
use crate::{diag::Diag, ir, report::throw_engine_error};

pub struct Engine<'eng> {
	stack: Stack,
	values: HashMap<ir::Register, Value>,
	fns: HashMap<ir::FnId, ir::Fn>,
	root: &'eng mut ir::Root,
	instrs: Vec<ir::Instr>,
}

type Result<T> = std::result::Result<T, Diag>;

impl<'eng> Engine<'eng> {
	pub fn new(root: &'eng mut ir::Root) -> Self {
		let fns = HashMap::new();
		let values = HashMap::new();
		let instrs = Vec::new();
		let stack = Stack::new(root.get_reg_size());
		Self { root, stack, values, fns, instrs }
	}
	pub fn execute(&mut self) -> Result<()> {
		// let mut globals = self.root.globals.clone();
		// for instr in globals.instrs.iter_mut() {
		// 	self.exe_instr(instr)?;
		// }
		// self.root.globals.instrs = mem::take(&mut self.instrs);
		Ok(())
	}

	fn exe_instr(&mut self, instr: &mut ir::Instr) -> Result<()> {
		match instr {
			ir::Instr::Add(binary) => self.exe_add_instr(binary),
			ir::Instr::Sub(binary) => self.exe_sub_instr(binary),
			ir::Instr::Div(binary) => self.exe_div_instr(binary),
			ir::Instr::Mul(binary) => self.exe_mul_instr(binary),
			ir::Instr::Mod(binary) => self.exe_mod_instr(binary),
			ir::Instr::CmpGt(binary) => self.exe_cmp_gt_instr(binary),
			ir::Instr::CmpEq(binary) => self.exe_cmp_eq_instr(binary),
			ir::Instr::CmpLt(binary) => self.exe_cmp_lt_instr(binary),
			ir::Instr::CmpLe(binary) => self.exe_cmp_le_instr(binary),
			ir::Instr::CmpGe(binary) => self.exe_cmp_ge_instr(binary),
			ir::Instr::Load(unary) => self.exe_load_instr(unary),
			// ir::Instr::Store(unary) => self.exe_store_instr(unary),
			ir::Instr::Free(_) => throw_engine_error("free ir is not implemented"),
			// ir::Instr::Own(own) => self.exe_own_instr(own),
			// ir::Instr::Call(call) => self.exe_call_instr(call),
			// ir::Instr::Goto(goto) => self.exe_goto_instr(goto),
			_ => throw_engine_error(format!("code {:?} not implemented", instr)),
		}
	}

	fn exe_add_instr(&mut self, binary: &ir::BinaryInstr) -> Result<()> {
		let (lhs, rhs) = self.exe_binary_instr(binary);
		let result = match (lhs, rhs) {
			(Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs + rhs),
			(Value::Float(lhs), Value::Float(rhs)) => Value::Float(lhs + rhs),
			_ => unreachable!(),
		};
		self.stack.current_frame().set_register(&binary.dest, result);
		Ok(())
	}

	fn exe_sub_instr(&mut self, binary: &ir::BinaryInstr) -> Result<()> {
		let (lhs, rhs) = self.exe_binary_instr(binary);
		let result = match (lhs, rhs) {
			(Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs.min(rhs)),
			(Value::Float(lhs), Value::Float(rhs)) => Value::Float(lhs.min(rhs)),
			_ => unreachable!(),
		};
		self.stack.current_frame().set_register(&binary.dest, result);
		Ok(())
	}

	fn exe_div_instr(&mut self, binary: &ir::BinaryInstr) -> Result<()> {
		let (lhs, rhs) = self.exe_binary_instr(binary);
		let result = match (lhs, rhs) {
			(Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs.div(rhs)),
			(Value::Float(lhs), Value::Float(rhs)) => Value::Float(lhs.div(rhs)),
			_ => unreachable!(),
		};
		self.stack.current_frame().set_register(&binary.dest, result);
		Ok(())
	}

	fn exe_mul_instr(&mut self, binary: &ir::BinaryInstr) -> Result<()> {
		let (lhs, rhs) = self.exe_binary_instr(binary);
		let result = match (lhs, rhs) {
			(Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs.mul(rhs)),
			(Value::Float(lhs), Value::Float(rhs)) => Value::Float(lhs.mul(rhs)),
			_ => unreachable!(),
		};
		self.stack.current_frame().set_register(&binary.dest, result);
		Ok(())
	}

	fn exe_mod_instr(&mut self, binary: &ir::BinaryInstr) -> Result<()> {
		let (lhs, rhs) = self.exe_binary_instr(binary);
		let result = match (lhs, rhs) {
			(Value::Int(lhs), Value::Int(rhs)) => Value::Int(lhs.rem(rhs)),
			(Value::Float(lhs), Value::Float(rhs)) => Value::Float(lhs.rem(rhs)),
			_ => unreachable!(),
		};
		self.stack.current_frame().set_register(&binary.dest, result);
		Ok(())
	}

	fn exe_cmp_gt_instr(&mut self, binary: &ir::BinaryInstr) -> Result<()> {
		let (lhs, rhs) = self.exe_binary_instr(binary);
		let result = match (lhs, rhs) {
			(Value::Int(lhs), Value::Int(rhs)) => Value::Bool(lhs > rhs),
			(Value::Float(lhs), Value::Float(rhs)) => Value::Bool(lhs > rhs),
			_ => todo!("code {:?}", binary),
		};
		self.stack.current_frame().set_register(&binary.dest, result);
		Ok(())
	}

	fn exe_cmp_eq_instr(&mut self, binary: &ir::BinaryInstr) -> Result<()> {
		let (lhs, rhs) = self.exe_binary_instr(binary);
		let result = match (lhs, rhs) {
			(Value::Int(lhs), Value::Int(rhs)) => Value::Bool(lhs == rhs),
			(Value::Float(lhs), Value::Float(rhs)) => Value::Bool(lhs == rhs),
			_ => throw_engine_error("we don't expect... ocurrs in cmp_eq"),
		};
		self.stack.current_frame().set_register(&binary.dest, result);
		Ok(())
	}

	fn exe_cmp_lt_instr(&mut self, binary: &ir::BinaryInstr) -> Result<()> {
		let (lhs, rhs) = self.exe_binary_instr(binary);
		let result = match (lhs, rhs) {
			(Value::Int(lhs), Value::Int(rhs)) => Value::Bool(lhs < rhs),
			(Value::Float(lhs), Value::Float(rhs)) => Value::Bool(lhs < rhs),
			_ => throw_engine_error("we don't expect... ocurrs in cmp_lt"),
		};
		self.stack.current_frame().set_register(&binary.dest, result);
		Ok(())
	}

	fn exe_cmp_le_instr(&mut self, binary: &ir::BinaryInstr) -> Result<()> {
		let (lhs, rhs) = self.exe_binary_instr(binary);
		let result = match (lhs, rhs) {
			(Value::Int(lhs), Value::Int(rhs)) => Value::Bool(lhs <= rhs),
			(Value::Float(lhs), Value::Float(rhs)) => Value::Bool(lhs <= rhs),
			_ => throw_engine_error("we don't expect... ocurrs in cmp_le"),
		};
		self.stack.current_frame().set_register(&binary.dest, result);
		Ok(())
	}

	fn exe_cmp_ge_instr(&mut self, binary: &ir::BinaryInstr) -> Result<()> {
		let (lhs, rhs) = self.exe_binary_instr(binary);
		let result = match (lhs, rhs) {
			(Value::Int(lhs), Value::Int(rhs)) => Value::Bool(lhs >= rhs),
			(Value::Float(lhs), Value::Float(rhs)) => Value::Bool(lhs >= rhs),
			_ => throw_engine_error("we don't expect... ocurrs in cmp_ge"),
		};
		self.stack.current_frame().set_register(&binary.dest, result);
		Ok(())
	}

	fn exe_cmp_ne_instr(&mut self, binary: &ir::BinaryInstr) -> Result<()> {
		let (lhs, rhs) = self.exe_binary_instr(binary);
		let result = match (lhs, rhs) {
			(Value::Int(lhs), Value::Int(rhs)) => Value::Bool(lhs != rhs),
			(Value::Float(lhs), Value::Float(rhs)) => Value::Bool(lhs != rhs),
			_ => throw_engine_error("we don't expect... ocurrs in cmp_ne"),
		};
		self.stack.current_frame().set_register(&binary.dest, result);
		Ok(())
	}

	fn exe_load_instr(&mut self, unary: &ir::UnaryInstr) -> Result<()> {
		let value = self.stack.current_frame().get_register(&unary.value);
		self.stack.current_frame().set_register(&unary.dest, value);
		Ok(())
	}

	fn exe_store_instr(&mut self, unary: &ir::UnaryInstr) -> Result<()> {
		let value = self.stack.current_frame().get_register(&unary.value);
		self.stack.current_frame().set_register(&unary.dest, value);
		Ok(())
	}

	fn exe_free_instr(&mut self, unary: &ir::UnaryInstr) -> Result<()> {
		let value = self.stack.current_frame().get_register(&unary.value);
		self.stack.current_frame().set_register(&unary.dest, value);
		Ok(())
	}

	fn exe_binary_instr(&mut self, binary: &ir::BinaryInstr) -> (Value, Value) {
		let lhs = self.stack.current_frame().get_register(&binary.lhs);
		let rhs = self.stack.current_frame().get_register(&binary.rhs);
		(lhs, rhs)
	}

	// fn exe_own_instr(&mut self, own: &ir::OwnInstr) -> Result<()> {
	// 	if let ir::Value::Register(register) = &own.value {
	// 		let value = self.stack.current_frame().get_register(register);
	// 		let ir_value = self.engine_value_to_ir_value(&value, register.as_usize())?;
	// 		let own = ir::OwnInstr { type_id: own.type_id, value: ir_value, dest: *register };
	// 		self.instrs.push(ir::Instr::Own(own));
	// 		// save instruction
	// 		self.values.insert(*register, value);
	// 		return Ok(());
	// 	}
	// 	let value = self.exe_value(&own.value);
	// 	self.stack.current_frame().set_register(&own.dest, value);
	// 	Ok(())
	// }

	// fn engine_value_to_ir_value(&self, value: &Value, register: usize) -> Result<ir::Value> {
	// 	let value = match value {
	// 		Value::Int(int) => ir::Value::Int(*int),
	// 		Value::Float(float) => ir::Value::Float(*float),
	// 		Value::Bool(bool) => ir::Value::Bool(*bool),
	// 		Value::String(string) => ir::Value::String(string.clone()),
	// 		Value::Char(char) => ir::Value::Char(*char),
	// 		Value::Register(register) => ir::Value::Register(*register),
	// 		Value::Zero => return Err(EngineError::uninitialized_register(register)),
	// 	};
	// 	Ok(value)
	// }

	// fn exe_value(&self, value: &ir::Value) -> Value {
	// 	match value {
	// 		ir::Value::Register(register) => {
	// 			if let Some(value) = self.values.get(register) {
	// 				return value.clone();
	// 			}
	// 			Value::Register(*register)
	// 		}
	// 		ir::Value::Int(int) => Value::Int(*int),
	// 		ir::Value::Float(float) => Value::Float(*float),
	// 		ir::Value::Bool(bool) => Value::Bool(*bool),
	// 		ir::Value::String(string) => Value::String(string.clone()),
	// 		ir::Value::Char(char) => Value::Char(*char),
	// 		ir::Value::Bind(_) => throw_engine_error("bind ir is not implemented"),
	// 		_ => throw_engine_error(format!("value {:?} not implemented", value)),
	// 	}
	// }
}
