use super::label::Label;
use crate::{
	ir::{Instr, IrBlock},
	report::throw_error,
};

pub struct Block {
	pub label: Label,
	pub next: Label,
	pub blocks: Vec<IrBlock>,
}

impl Default for Block {
	fn default() -> Self {
		Self::new()
	}
}

impl Block {
	pub fn new() -> Self {
		let label = Label::default();

		let block = IrBlock::new(label.into());
		Self { label, blocks: vec![block], next: label }
	}

	pub fn new_block(&mut self) -> Label {
		let label = self.next.increment();
		self.next = label;
		// start from 0, but label starts from 1
		self.blocks.push(IrBlock::new(label.into()));
		label
	}

	pub fn take_blocks(&mut self) -> Vec<IrBlock> {
		self.label = Label::default();
		let blocks = std::mem::take(&mut self.blocks);
		self.blocks = vec![IrBlock::new(self.label.into())];
		blocks
	}

	pub fn get_current_block(&mut self) -> &mut IrBlock {
		self.get_block_mut(self.label)
	}

	pub fn add_instr(&mut self, instr: Instr) {
		self.get_current_block().add_instr(instr);
	}

	pub fn switch_to_block(&mut self, label: Label) {
		let value = label.value.wrapping_sub(1);
		if self.blocks.len() <= value {
			throw_error(format!(
				"block index out of range: {}, blocks: {}",
				label.value,
				self.blocks.len()
			));
		}
		self.label = label;
	}

	pub fn get_block_mut(&mut self, label: Label) -> &mut IrBlock {
		let value = label.value.wrapping_sub(1);
		if self.blocks.len() <= value {
			throw_error(format!(
				"block index out of range: {}, blocks: {}",
				label.value,
				self.blocks.len()
			));
		}
		match self.blocks.get_mut(value) {
			Some(block) => block,
			None => throw_error(format!("block index out of range: {}", label.value)),
		}
	}
}
