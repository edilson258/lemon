use std::mem;

use crate::ir::ir;

pub struct Optimizer<'opt> {
	pub root: &'opt mut ir::Root,
}

impl<'opt> Optimizer<'opt> {
	pub fn new(root: &'opt mut ir::Root) -> Self {
		Self { root }
	}
	pub fn optimize(&mut self) -> ir::Root {
		// self.root.fns.iter_mut().for_each(|ir_fn| self.optimize_fn(ir_fn));
		mem::take(&mut self.root)
	}

	fn optimize_fn(&mut self, ir_fn: &mut ir::Fn) {
		for block in ir_fn.blocks.iter_mut() {
			self.optimize_block(block);
		}
	}

	fn optimize_block(&mut self, block: &mut ir::Block) {
		for instr in block.instrs.iter_mut() {
			self.optimize_instr(instr);
		}
	}

	fn optimize_instr(&mut self, instr: &mut ir::Instr) {
		match instr {
			ir::Instr::Add(binary) => self.optimize_add(binary),
			ir::Instr::Sub(binary) => self.optimize_sub(binary),
			ir::Instr::Div(binary) => self.optimize_div(binary),
			ir::Instr::Mul(binary) => self.optimize_mul(binary),
			ir::Instr::Mod(binary) => self.optimize_mod(binary),
			ir::Instr::CmpGt(binary) => self.optimize_cmp_gt(binary),
			ir::Instr::CmpEq(binary) => self.optimize_cmp_eq(binary),
			ir::Instr::CmpLt(binary) => self.optimize_cmp_lt(binary),
			ir::Instr::CmpLe(binary) => self.optimize_cmp_le(binary),
			_ => todo!("code {:?}", instr),
		}
	}

	fn optimize_add(&mut self, binary: &mut ir::BinaryInstr) {
		todo!()
	}

	fn optimize_sub(&mut self, binary: &mut ir::BinaryInstr) {
		todo!()
	}

	fn optimize_div(&mut self, binary: &mut ir::BinaryInstr) {
		todo!()
	}

	fn optimize_mul(&mut self, binary: &mut ir::BinaryInstr) {
		todo!()
	}

	fn optimize_mod(&mut self, binary: &mut ir::BinaryInstr) {
		todo!()
	}

	fn optimize_cmp_gt(&mut self, binary: &mut ir::BinaryInstr) {
		todo!()
	}

	fn optimize_cmp_eq(&mut self, binary: &mut ir::BinaryInstr) {
		todo!()
	}

	fn optimize_cmp_lt(&mut self, binary: &mut ir::BinaryInstr) {
		todo!()
	}

	fn optimize_cmp_le(&mut self, binary: &mut ir::BinaryInstr) {
		todo!()
	}
}
