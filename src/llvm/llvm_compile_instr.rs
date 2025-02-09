use crate::ir::{self};

use super::Llvm;

impl Llvm<'_> {
	#[rustfmt::skip]
	pub fn llvm_compile_instr(&mut self, instr: &ir::Instr) {
		match instr {
			ir::Instr::Add(binary)     => self.llvm_compile_add(binary),
			ir::Instr::Sub(binary)     => self.llvm_compile_sub(binary),
			ir::Instr::Mul(binary)     => self.llvm_compile_mul(binary),
			ir::Instr::CmpGt(binary)   => self.llvm_compile_cmp_gt(binary),
			ir::Instr::CmpEq(binary)   => self.llvm_compile_cmp_eq(binary),
			ir::Instr::CmpLt(binary)   => self.llvm_compile_cmp_lt(binary),
			ir::Instr::CmpLe(binary)   => self.llvm_compile_cmp_le(binary),
			ir::Instr::CmpGe(binary)   => self.llvm_compile_cmp_ge(binary),
			ir::Instr::Load(unary)     => self.llvm_compile_load(unary),
			ir::Instr::Call(call)      => self.llvm_compile_call(call),
			ir::Instr::Ret(ret)        => self.llvm_compile_ret(ret),
			ir::Instr::JmpIf(jump_if)  => self.llvm_compile_jmp_if(jump_if),
			ir::Instr::Jmp(jump)       => self.llvm_compile_jmp(jump),
			ir::Instr::Set(unary)      => self.llvm_compile_set(unary),
			ir::Instr::Salloc(instr)   => self.llvm_compile_salloc(instr),
			_ => todo!("code {:?}", instr),
		}
	}
}
