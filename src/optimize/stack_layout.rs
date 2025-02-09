use std::mem;

// todo: is really necessary? NB: this code is only draft...
use rustc_hash::FxHashMap;

use crate::{
	checker::types::TypeId,
	ir::{self, SallocInstr},
};

pub struct StackLayout {
	stack_grup: FxHashMap<String, TypeId>, // name -> type_id
	layout: FxHashMap<String, usize>,
}

impl StackLayout {
	pub fn new() -> Self {
		Self { stack_grup: FxHashMap::default(), layout: FxHashMap::default() }
	}

	pub fn align_to(&self, offset: usize, align: usize) -> usize {
		if offset % align == 0 {
			return offset;
		}
		offset + (align - (offset % align))
	}

	pub fn create_stack_layout(&mut self) {
		let mut offset = 0;
		let mut layout = FxHashMap::default();
		for (name, type_id) in &self.stack_grup {
			let align = type_id.get_align();
			offset = self.align_to(offset, align);
			layout.insert(name.to_owned(), offset);
			offset += type_id.get_size();
		}
		self.layout = layout;
	}

	fn optimize_function(&mut self, function: &mut ir::Function) {
		// todo: is good?
		function.blocks.iter_mut().for_each(|block| self.optimize_block(block));
		self.create_stack_layout();
		if let Some(block) = function.blocks.first_mut() {
			let layout_fmt = format!("layout [{} x i32]", block.instrs.len());
			let layout_value = ir::IrValue::new(layout_fmt, TypeId::I32);
			let alloc_layout = SallocInstr::new(layout_value, TypeId::I32);
			block.instrs.insert(0, alloc_layout.into());
		}

		function.blocks.iter_mut().for_each(|block| self.optimize_gen_block(block));
	}

	fn optimize_block(&mut self, block: &mut ir::IrBlock) {
		let taked = mem::take(block);
		let instrs = taked.instrs.into_iter().filter(|instr| {
			if let ir::Instr::Salloc(instr) = instr {
				self.stack_grup.insert(instr.dest.value.clone(), instr.dest.kind);
				return false;
			}
			true
		});
		let instrs = instrs.collect::<Vec<_>>();
		block.instrs.extend(instrs);
	}

	fn optimize_gen_block(&mut self, block: &mut ir::IrBlock) {
		block.instrs.iter_mut().for_each(|instr| self.optimize_gen_instr(instr));
	}

	fn optimize_gen_instr(&mut self, instr: &mut ir::Instr) {
		if let ir::Instr::Set(instr) = instr {
			if let Some(position) = self.layout.get(&instr.dest.value) {
				let dest = &instr.dest;
				let dest_in_layout = format!("layout [{}]", position);
				let dest = ir::IrValue::new(dest_in_layout, dest.kind);
				instr.dest = dest;
			}
		}
	}
	pub fn optimize(&mut self, ir: &mut ir::IR) {
		ir.functions.iter_mut().for_each(|function| self.optimize_function(function));
	}
}
impl Default for StackLayout {
	fn default() -> Self {
		Self::new()
	}
}
