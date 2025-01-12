use crate::ir::{self};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_instr(&mut self, instr: &ir::Instr) {
		match instr {
			ir::Instr::Add(binary) => self.llvm_add(binary),
			ir::Instr::Sub(binary) => self.llvm_sub(binary),
			ir::Instr::Mul(binary) => self.llvm_mul(binary),
			ir::Instr::CmpGt(binary) => self.llvm_cmp_gt(binary),
			ir::Instr::CmpEq(binary) => self.llvm_cmp_eq(binary),
			ir::Instr::CmpLt(binary) => self.llvm_cmp_lt(binary),
			ir::Instr::CmpLe(binary) => self.llvm_cmp_le(binary),
			ir::Instr::CmpGe(binary) => self.llvm_cmp_ge(binary),
			ir::Instr::Load(unary) => self.llvm_load(unary),
			ir::Instr::Store(store) => self.llvm_store(store),
			ir::Instr::Borrow(unary) => self.llvm_borrow(unary),
			ir::Instr::Free(free) => self.llvm_free(free),
			ir::Instr::Own(own) => self.llvm_own(own),
			ir::Instr::OwnHeap(own_heap) => self.llvm_own_heap(own_heap),
			ir::Instr::Call(call) => self.llvm_call(call),
			ir::Instr::Ret(ret) => self.llvm_ret(ret),
			_ => todo!("code {:?}", instr),
		}
	}
}
