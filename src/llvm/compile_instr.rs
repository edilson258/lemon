use crate::ir::{self};
use inkwell::{
	basic_block::BasicBlock,
	values::{BasicValueEnum, FunctionValue},
};
use lemon::report::throw_llvm_error;

use super::Llvm;

impl<'ll> Llvm<'ll> {
	pub fn compile_instr(&mut self, instr: &ir::Instr) {
		match instr {
			ir::Instr::Add(binary) => self.compile_add(binary),
			ir::Instr::Sub(binary) => self.compile_sub(binary),
			ir::Instr::Mul(binary) => self.compile_mul(binary),
			ir::Instr::CmpGt(binary) => self.compile_cmp_gt(binary),
			ir::Instr::CmpEq(binary) => self.compile_cmp_eq(binary),
			ir::Instr::CmpLt(binary) => self.compile_cmp_lt(binary),
			ir::Instr::CmpLe(binary) => self.compile_cmp_le(binary),
			ir::Instr::CmpGe(binary) => self.compile_cmp_ge(binary),
			ir::Instr::Load(unary) => self.compile_load(unary),
			ir::Instr::Store(unary) => self.compile_store(unary),
			ir::Instr::Borrow(unary) => self.compile_borrow(unary),
			ir::Instr::BorrowMut(unary) => self.compile_borrow_mut(unary),
			ir::Instr::Free(register) => self.compile_free(register),
			ir::Instr::Own(own) => self.compile_own(own),
			ir::Instr::Call(call) => self.compile_call(call),
			ir::Instr::Goto(goto) => self.compile_goto(goto),
			ir::Instr::JmpIf(jmp) => self.compile_jmp_if(jmp),
			ir::Instr::Ret(ret) => self.compile_ret(ret),
			ir::Instr::Import(import) => self.compile_import(import),
			_ => todo!("don't impl instr: {:?}", instr),
		}
	}

	fn compile_borrow_mut(&mut self, borrow: &ir::UnaryInstr) {
		let value_ptr = self.load_value(borrow.value);
		let dest_str = borrow.dest.as_string();
		let value_type = value_ptr.get_type();
		let dest_ptr = match self.builder.build_alloca(value_type, &dest_str) {
			Ok(ptr) => ptr,
			Err(err) => {
				throw_llvm_error(format!("borrow register {} with type {:?}", dest_str, value_type));
			}
		};
		self.insert_value(borrow.dest, value_ptr);
	}

	fn compile_borrow(&mut self, borrow: &ir::UnaryInstr) {
		let value_ptr = self.load_value(borrow.value);
		self.insert_value(borrow.dest, value_ptr);
	}

	fn compile_own(&mut self, own: &ir::OwnInstr) {
		let value = self.compile_value(&own.value);
		let value_type = match self.compile_type_id(own.type_id) {
			Some(value_type) => value_type,
			None => value.get_type(),
		};
		self.insert_value(own.dest, value);
	}

	fn compile_value(&mut self, value: &ir::Value) -> BasicValueEnum<'ll> {
		let llvm_int_type = self.ctx.i32_type();
		let llvm_float_type = self.ctx.f64_type();
		match value {
			ir::Value::Int(i) => llvm_int_type.const_int(*i as u64, false).into(),
			ir::Value::Float(f) => llvm_float_type.const_float(*f).into(),
			ir::Value::Bool(b) => llvm_int_type.const_int(if *b { 1 } else { 0 }, false).into(),
			ir::Value::Char(c) => llvm_int_type.const_int(*c as u64, false).into(),
			ir::Value::String(s) => todo!("String not implemented"),
			ir::Value::Register(reg) => self.load_value(*reg),
			_ => todo!("value {:?}", value),
		}
	}

	fn compile_goto(&mut self, goto: &ir::GotoInstr) {
		let block = *self.get_block(&goto.block_id);
		self.builder.build_unconditional_branch(block).expect("error building goto");
	}

	fn compile_free(&mut self, register: &ir::Register) {
		// let value = self.load_value(*register);
		// match self.builder.build_free(value) {
		// 	Ok(_) => {}
		// 	Err(err) => throw_llvm_error(format!("free {}", register.as_string())),
		// }
	}

	fn compile_load(&mut self, load: &ir::UnaryInstr) {
		let value = self.load_value(load.value);
		let llvm_type = self.compile_type_id(load.type_id).unwrap();
		// let result = match self.builder.build_load(llvm_type, value, &load.dest.as_string()) {
		// 	Ok(result) => result,
		// 	Err(err) => {
		// 		throw_llvm_error(format!("load register {:?} with type {:?}", load.dest, llvm_type))
		// 	}
		// };
		self.insert_value(load.dest, value);
		// let dest = self.allocate_register(llvm_type, load.dest);
		// self.store_value_to_ptr(result, dest);
	}

	fn compile_store(&mut self, store: &ir::UnaryInstr) {
		let value = self.load_value(store.value);
		self.insert_value(store.dest, value);
	}
	fn compile_jmp_if(&mut self, jmp: &ir::JmpIfInstr) {
		let cond_value = self.load_value(jmp.cond).into_int_value();
		let block_left = self.get_or_create_block(&jmp.l0);
		let block_right = self.get_or_create_block(&jmp.l1);
		match self.builder.build_conditional_branch(cond_value, block_left, block_right) {
			Ok(_) => {}
			Err(err) => {
				throw_llvm_error(format!(
					"failed to build conditional branch: cond_register={:?}, error={:?}",
					jmp.cond, err
				));
			}
		}
	}

	fn get_or_create_block(&mut self, block_id: &ir::BlockId) -> BasicBlock<'ll> {
		if let Some(existing_block) = self.block_store.get(block_id) {
			return *existing_block;
		}
		let block_name = self.compile_block_id(block_id);
		let parent_fn = self.get_parent_block();
		let new_block = self.ctx.append_basic_block(parent_fn, &block_name);
		self.set_block(block_id, new_block);
		new_block
	}

	fn get_parent_block(&self) -> FunctionValue<'ll> {
		match self.builder.get_insert_block() {
			Some(block) => match block.get_parent() {
				Some(parent) => parent,
				None => throw_llvm_error("no parent fn found for the current block."),
			},
			None => throw_llvm_error("builder is not positioned in a block."),
		}
	}

	fn compile_ret(&mut self, ret: &ir::RetInstr) {
		if let Some(value) = ret.value {
			let ret_value = self.load_value(value);
			self.builder.build_return(Some(&ret_value)).expect("error building ret");
		} else {
			self.builder.build_return(None).expect("error building ret");
		}
	}
}
