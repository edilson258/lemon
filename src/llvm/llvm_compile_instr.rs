use crate::ir::{self};

use super::Llvm;

impl Llvm<'_> {
	#[rustfmt::skip]
	pub fn llvm_compile_instr(&mut self, instr: &ir::Instr) {
		match instr {
		  // math
			ir::Instr::Add(binary)     => self.llvm_compile_add(binary),
			ir::Instr::Sub(binary)     => self.llvm_compile_sub(binary),
			ir::Instr::Mul(binary)     => self.llvm_compile_mul(binary),
			ir::Instr::Div(binary)     => self.llvm_compile_div(binary),
			ir::Instr::Mod(binary)     => self.llvm_compile_mod(binary),
			// logic
			// ir::Instr::Neg(binary)     => self.llvm_compile_neg(binary),
			// ir::Instr::Not(binary)     => self.llvm_compile_not(binary),
			ir::Instr::And(binary)     => self.llvm_compile_and(binary),
			ir::Instr::Or(binary)      => self.llvm_compile_or(binary),
			ir::Instr::Shl(binary)     => self.llvm_compile_shl(binary),
			ir::Instr::Shr(binary)     => self.llvm_compile_shr(binary),
			// cmp
			ir::Instr::CmpGt(binary)   => self.llvm_compile_cmp_gt(binary),
			ir::Instr::CmpEq(binary)   => self.llvm_compile_cmp_eq(binary),
			ir::Instr::CmpLt(binary)   => self.llvm_compile_cmp_lt(binary),
			ir::Instr::CmpLe(binary)   => self.llvm_compile_cmp_le(binary),
			ir::Instr::CmpGe(binary)   => self.llvm_compile_cmp_ge(binary),
			// mem
			ir::Instr::Load(unary)     => self.llvm_compile_load(unary),
			ir::Instr::Set(unary)      => self.llvm_compile_set(unary),
			ir::Instr::Salloc(instr)   => self.llvm_compile_salloc(instr),
			// control
			ir::Instr::JmpIf(jump_if)  => self.llvm_compile_jmp_if(jump_if),
			ir::Instr::Jmp(jump)       => self.llvm_compile_jmp(jump),
			// fn
			ir::Instr::Call(call)      => self.llvm_compile_call(call),
			ir::Instr::Ret(ret)        => self.llvm_compile_ret(ret),
			_ => todo!("code {:?}", instr),
		}
	}
}
