use super::ir::{self};
use crate::checker::types::{TypeFormatter, TypeId, TypeStore};

pub struct Disassembler<'ir> {
	pub formatter_type: TypeFormatter<'ir>,
}

impl<'ir> Disassembler<'ir> {
	pub fn new(type_store: &'ir TypeStore) -> Self {
		Self { formatter_type: TypeFormatter::new(type_store) }
	}

	pub fn disassemble(&self, ir: &ir::Ir) -> String {
		let mut result = String::new();
		for fn_ir in ir.fns.iter() {
			self.disassemble_fn_ir(fn_ir, &mut result);
			result.push('\n');
		}
		result
	}

	fn disassemble_fn_ir(&self, fn_ir: &ir::Fn, result: &mut String) {
		result.push_str("fn ");
		self.disassemble_fn_id(&fn_ir.fn_id, result);
		if !fn_ir.params.is_empty() {
			result.push(' ');
		}
		self.disassemble_binds(&fn_ir.params, result);
		result.push_str(" -> ");
		self.resolve_type_id(fn_ir.ret_id, result);
		result.push('\n');

		for block in fn_ir.body.iter() {
			self.disassemble_block(block, result);
		}
	}

	fn disassemble_block(&self, block: &ir::Block, result: &mut String) {
		self.disassemble_label(&block.label, result);
		if block.instrs.is_empty() {
			result.push('\n');
		}
		for (index, instr) in block.instrs.iter().enumerate() {
			if index == 0 {
				self.disassemble_instr(instr, result);
			} else {
				// add padding
				result.push_str("    ");
				self.disassemble_instr(instr, result);
			}
		}
	}

	fn disassemble_instr(&self, instr: &ir::Instr, result: &mut String) {
		// self.disassemble_meta(&instr.meta, result);

		self.disassemble_code(&instr.code, result);
		result.push('\n');
	}

	fn disassemble_code(&self, code: &ir::Code, result: &mut String) {
		match code {
			ir::Code::ADD { lhs, rhs, dest } => {
				result.push_str("add ");
				self.disassemble_three_register(lhs, rhs, dest, result);
			}
			ir::Code::SUB { lhs, rhs, dest } => {
				result.push_str("sub ");
				self.disassemble_three_register(lhs, rhs, dest, result);
			}
			ir::Code::DIV { lhs, rhs, dest } => {
				result.push_str("div ");
				self.disassemble_three_register(lhs, rhs, dest, result);
			}
			ir::Code::MUL { lhs, rhs, dest } => {
				result.push_str("mul ");
				self.disassemble_three_register(lhs, rhs, dest, result);
			}
			ir::Code::MOD { lhs, rhs, dest } => {
				result.push_str("mod ");
				self.disassemble_three_register(lhs, rhs, dest, result);
			}
			ir::Code::CMPGT { lhs, rhs, dest } => {
				result.push_str("cmp_gt ");
				self.disassemble_three_register(lhs, rhs, dest, result);
			}
			ir::Code::CMPEQ { lhs, rhs, dest } => {
				result.push_str("cmp_eq ");
				self.disassemble_three_register(lhs, rhs, dest, result);
			}
			ir::Code::CMPLT { lhs, rhs, dest } => {
				result.push_str("cmp_lt ");
				self.disassemble_three_register(lhs, rhs, dest, result);
			}
			ir::Code::CMPLE { lhs, rhs, dest } => {
				result.push_str("cmp_le ");
				self.disassemble_three_register(lhs, rhs, dest, result);
			}
			ir::Code::JMPIF { cond, l0, l1 } => {
				result.push_str("jmp_if ");
				self.disassemble_register(cond, result);
				result.push_str(", ");
				self.disassemble_label(l0, result);
				result.push_str(", ");
				self.disassemble_label(l1, result);
			}
			ir::Code::GOTO { to } => {
				result.push_str("goto ");
				self.disassemble_label(to, result);
			}
			ir::Code::OWN { value, dest } => {
				result.push_str("own ");
				self.disassemble_register(value, result);
				result.push_str(" -> ");
				self.disassemble_register(dest, result);
			}
			ir::Code::BORROW { value, dest } => {
				result.push_str("borrow ");
				self.disassemble_register(value, result);
				result.push_str(" -> ");
				self.disassemble_register(dest, result);
			}
			ir::Code::BORROW_MUT { value, dest } => {
				result.push_str("borrow_mut ");
				self.disassemble_register(value, result);
				result.push_str(" -> ");
				self.disassemble_register(dest, result);
			}
			ir::Code::FREE { value } => {
				result.push_str("free ");
				self.disassemble_register(value, result);
			}
			ir::Code::RET { value } => {
				result.push_str("ret ");
				self.disassemble_register(value, result);
			}
			ir::Code::CALL { fn_id, args, dest } => {
				result.push_str("call ");
				self.disassemble_fn_id(fn_id, result);
				result.push_str(", ");
				self.disassemble_registers(args, result);
				result.push_str(" -> ");
				self.disassemble_register(dest, result);
			}
			ir::Code::HEAP { value, dest } => {
				result.push_str("heap ");
				self.disassemble_head_value(value, result);
				result.push_str(" -> ");
				self.disassemble_register(dest, result);
			}
			ir::Code::LOAD { value, dest } => {
				result.push_str("load ");
				self.disassemble_register(value, result);
				result.push_str(" -> ");
				self.disassemble_register(dest, result);
			}
			_ => todo!("code {:?}", code),
		}
	}

	fn disassemble_three_register(
		&self,
		lhs: &ir::Register,
		rhs: &ir::Register,
		dest: &ir::Register,
		result: &mut String,
	) {
		self.disassemble_register(lhs, result);
		result.push_str(", ");
		self.disassemble_register(rhs, result);
		result.push_str(", ");
		self.disassemble_register(dest, result);
	}

	fn disassemble_bind(&self, bind: &ir::Bind, result: &mut String) {
		self.disassemble_register(&bind.register, result);
		result.push_str(": ");
		self.resolve_type_id(bind.type_id, result);
	}

	fn disassemble_binds(&self, binds: &[ir::Bind], result: &mut String) {
		for (i, bind) in binds.iter().enumerate() {
			if i > 0 {
				result.push_str(", ");
			}
			self.disassemble_bind(bind, result);
		}
	}

	fn disassemble_register(&self, register: &ir::Register, result: &mut String) {
		result.push_str(&format!("r{}", register.0));
	}

	fn disassemble_fn_id(&self, fn_id: &ir::FnId, result: &mut String) {
		result.push_str(&fn_id.0.to_string());
	}

	fn disassemble_label(&self, label: &ir::Label, result: &mut String) {
		result.push_str(&format!("l{}: ", label.0));
	}

	fn disassemble_head_value(&self, head_value: &ir::HeapValue, result: &mut String) {
		match head_value {
			ir::HeapValue::Char(c) => result.push_str(&format!("'{}'", c)),
			ir::HeapValue::String(s) => result.push_str(&format!("\"{}\"", s)),
			ir::HeapValue::Int(i) => result.push_str(&format!("{}", i)),
			ir::HeapValue::Float(f) => result.push_str(&f.to_string()),
			ir::HeapValue::Bool(b) => result.push_str(&format!("{}", b)),
		}
	}

	fn disassemble_registers(&self, registers: &[ir::Register], result: &mut String) {
		for (i, register) in registers.iter().enumerate() {
			if i > 0 {
				result.push_str(", ");
			}
			self.disassemble_register(register, result);
		}
	}

	fn resolve_type_id(&self, type_id: TypeId, result: &mut String) {
		result.push_str(&self.formatter_type.format(type_id));
	}
	// fn disassemble_meta(&self, meta: &ir::Meta, result: &mut String) {
	//
}
