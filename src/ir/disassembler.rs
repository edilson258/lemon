use super::ir::{self};
use crate::checker::types::{TypeFormatter, TypeId, TypeStore};

pub struct Disassembler<'ir> {
	pub formatter_type: TypeFormatter<'ir>,
}

impl<'ir> Disassembler<'ir> {
	pub fn new(type_store: &'ir TypeStore) -> Self {
		Self { formatter_type: TypeFormatter::new(type_store) }
	}

	pub fn disassemble(&self, ir: &ir::Root) -> String {
		let mut result = String::new();
		self.disassemble_root(ir, &mut result);
		result
	}

	fn disassemble_root(&self, root: &ir::Root, result: &mut String) {
		self.disassemble_global(&root.globals, result);
		result.push('\n');
		for fn_ir in root.fns.iter() {
			self.disassemble_fn(fn_ir, result);
			result.push('\n');
		}
	}

	fn disassemble_global(&self, global: &ir::Global, result: &mut String) {
		result.push_str("globals:\n");
		for instr in global.instrs.iter() {
			self.disassemble_instr(instr, result);
			result.push('\n');
		}
	}

	fn disassemble_fn(&self, fn_ir: &ir::Fn, result: &mut String) {
		match fn_ir {
			ir::Fn::Native(fn_ir) => self.disassemble_native_fn(fn_ir, result),
			ir::Fn::Comptime(fn_ir) => self.disassemble_comptime_fn(fn_ir, result),
			ir::Fn::Extern(fn_ir) => self.disassemble_extern_fn(fn_ir, result),
		}
	}
	fn disassemble_native_fn(&self, fn_ir: &ir::FnNative, result: &mut String) {
		result.push_str("fn ");
		result.push_str(fn_ir.fn_id.as_string());
		if !fn_ir.params.is_empty() {
			result.push(' ');
		}
		self.disassemble_binds(&fn_ir.params, result);
		result.push_str(" -> ");
		self.resolve_type_id(fn_ir.ret, result);
		result.push('\n');
		for block in fn_ir.blocks.iter() {
			self.disassemble_block(block, result);
		}
	}
	fn disassemble_comptime_fn(&self, fn_ir: &ir::FnComptime, result: &mut String) {
		result.push_str("fn! ");
		result.push_str(fn_ir.fn_id.as_string());
		if !fn_ir.params.is_empty() {
			result.push(' ');
		}
		self.disassemble_binds(&fn_ir.params, result);
		result.push_str(" -> ");
		self.resolve_type_id(fn_ir.ret, result);
		result.push('\n');

		for block in fn_ir.blocks.iter() {
			self.disassemble_block(block, result);
		}
	}
	fn disassemble_extern_fn(&self, fn_ir: &ir::FnExtern, result: &mut String) {
		result.push_str("extern ");
		result.push_str(fn_ir.fn_id.as_string());
		if !fn_ir.params.is_empty() {
			result.push(' ');
		}
		self.disassemble_binds(&fn_ir.params, result);
		result.push_str(" -> ");
		self.resolve_type_id(fn_ir.ret, result);
		result.push('\n');

		for block in fn_ir.blocks.iter() {
			self.disassemble_block(block, result);
		}
	}

	fn disassemble_block(&self, block: &ir::Block, result: &mut String) {
		result.push_str(&block.block_id.as_string());
		result.push_str(": ");
		if block.instrs.is_empty() {
			result.push('\n');
		}
		for (index, instr) in block.instrs.iter().enumerate() {
			if index == 0 {
				self.disassemble_instr(instr, result);
				result.push('\n');
			} else {
				// add padding
				result.push_str("    ");
				self.disassemble_instr(instr, result);
				result.push('\n');
			}
		}
	}

	fn disassemble_binds(&self, binds: &[ir::Bind], result: &mut String) {
		for (i, bind) in binds.iter().enumerate() {
			if i > 0 {
				result.push_str(", ");
			}
			self.disassemble_bind(bind, result);
		}
	}

	fn disassemble_bind(&self, bind: &ir::Bind, result: &mut String) {
		result.push_str(&bind.register.as_string());
		result.push_str(": ");
		self.resolve_type_id(bind.type_id, result);
	}

	fn disassemble_instr(&self, instr: &ir::Instr, result: &mut String) {
		match instr {
			// binary
			ir::Instr::Add(binary) => self.disassemble_binary_instr("add", binary, result),
			ir::Instr::Sub(binary) => self.disassemble_binary_instr("sub", binary, result),
			ir::Instr::Div(binary) => self.disassemble_binary_instr("div", binary, result),
			ir::Instr::Mul(binary) => self.disassemble_binary_instr("mul", binary, result),
			ir::Instr::Mod(binary) => self.disassemble_binary_instr("mod", binary, result),
			ir::Instr::CmpGt(binary) => self.disassemble_binary_instr("cmp_gt", binary, result),
			ir::Instr::CmpEq(binary) => self.disassemble_binary_instr("cmp_eq", binary, result),
			ir::Instr::CmpLt(binary) => self.disassemble_binary_instr("cmp_lt", binary, result),
			ir::Instr::CmpLe(binary) => self.disassemble_binary_instr("cmp_le", binary, result),

			// unary
			ir::Instr::Borrow(borrow) => self.disassemble_unary_instr("borrow", borrow, result),
			ir::Instr::BorrowMut(borrow) => self.disassemble_unary_instr("borrow_mut", borrow, result),

			// others
			ir::Instr::JmpIf(jmp_if) => self.disassemble_jmpif(jmp_if, result),
			ir::Instr::Own(own) => self.disassemble_own(own, result),

			ir::Instr::Free(free) => self.disassemble_unary_instr("free", free, result),
			ir::Instr::Ret(ret) => self.disassemble_ret(ret, result),
			ir::Instr::Call(call) => self.disassemble_call(call, result),
			ir::Instr::Load(unary) => self.disassemble_unary_instr("load", unary, result),
			ir::Instr::Goto(goto) => self.disassemble_goto(goto, result),
			_ => todo!("code {:?}", instr),
		}
	}

	fn disassemble_ret(&self, ret: &ir::RetInstr, result: &mut String) {
		result.push_str("ret");

		if ret.type_id != TypeId::NOTHING {
			result.push(' ');
			self.resolve_type_id(ret.type_id, result);
		}
		if let Some(value) = &ret.value {
			result.push(' ');
			self.disassemble_register(value, result);
		}
	}

	fn disassemble_goto(&self, goto: &ir::GotoInstr, result: &mut String) {
		result.push_str("goto ");
		self.disassemble_block_id(&goto.block_id, result);
	}

	fn disassemble_binary_instr(&self, op: &str, binary: &ir::BinaryInstr, result: &mut String) {
		result.push_str(op);
		result.push(' ');
		self.disassemble_binary(binary, result);
	}
	fn disassemble_unary_instr(&self, op: &str, unary: &ir::UnaryInstr, result: &mut String) {
		result.push_str(op);
		result.push(' ');
		self.disassemble_unary(unary, result);
	}
	fn disassemble_jmpif(&self, jmp_if: &ir::JmpIfInstr, result: &mut String) {
		result.push_str("jmp_if ");
		self.disassemble_register(&jmp_if.cond, result);
		result.push_str(", ");
		self.disassemble_block_id(&jmp_if.l0, result);
		result.push_str(", ");
		self.disassemble_block_id(&jmp_if.l1, result);
	}
	fn disassemble_own(&self, own: &ir::OwnInstr, result: &mut String) {
		result.push_str("own ");
		if own.type_id != TypeId::NOTHING {
			self.resolve_type_id(own.type_id, result);
			result.push(' ');
		}
		self.disassemble_value(&own.value, result);
		result.push_str(" -> ");
		self.disassemble_register(&own.dest, result);
	}
	fn disassemble_call(&self, call: &ir::CallInstr, result: &mut String) {
		result.push_str("call ");
		self.disassemble_fn_id(&call.fn_id, result);
		result.push_str(", ");
		self.disassemble_registers(&call.args, result);
		result.push_str(" -> ");
		self.disassemble_register(&call.dest, result);
	}

	fn disassemble_unary(&self, unary: &ir::UnaryInstr, result: &mut String) {
		self.resolve_type_id(unary.type_id, result);
		result.push(' ');
		self.disassemble_register(&unary.value, result);
		result.push_str(" -> ");
		self.disassemble_register(&unary.dest, result);
	}

	fn disassemble_binary(&self, binary: &ir::BinaryInstr, result: &mut String) {
		self.resolve_type_id(binary.type_id, result);
		result.push(' ');
		self.disassemble_register(&binary.lhs, result);
		result.push_str(", ");
		self.disassemble_register(&binary.rhs, result);
		result.push_str(" -> ");
		self.disassemble_register(&binary.dest, result);
	}
	fn disassemble_register(&self, register: &ir::Register, result: &mut String) {
		result.push_str(&register.as_string());
	}

	fn disassemble_registers(&self, registers: &[ir::Register], result: &mut String) {
		for (i, register) in registers.iter().enumerate() {
			if i > 0 {
				result.push_str(", ");
			}
			self.disassemble_register(register, result);
		}
	}

	fn disassemble_value(&self, value: &ir::Value, result: &mut String) {
		match value {
			ir::Value::Register(register) => self.disassemble_register(register, result),
			ir::Value::Bool(bool) => result.push_str(format!("{}", bool).as_str()),
			ir::Value::String(string) => result.push_str(string.to_string().as_str()),
			ir::Value::Int(int) => result.push_str(format!("{}", int).as_str()),
			ir::Value::Float(float) => result.push_str(format!("{}", float).as_str()),
			ir::Value::Bind(bind) => self.disassemble_bind(bind, result),
			_ => todo!("value {:?}", value),
		}
	}

	fn disassemble_fn_id(&self, fn_id: &ir::FnId, result: &mut String) {
		result.push_str(fn_id.as_string());
	}

	fn disassemble_block_id(&self, block_id: &ir::BlockId, result: &mut String) {
		result.push_str(&block_id.as_string());
	}

	fn resolve_type_id(&self, type_id: TypeId, result: &mut String) {
		result.push_str(&self.formatter_type.format(type_id));
	}
}
