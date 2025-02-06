use crate::{
	ir::{Instr, IrBlock},
	report::throw_error,
};

pub struct Block {
	pub label_count: usize,
	pub blocks: Vec<IrBlock>,
}

impl Default for Block {
	fn default() -> Self {
		Self::new()
	}
}

impl Block {
	pub fn new() -> Self {
		let block = IrBlock::new(1);
		Self { label_count: 1, blocks: vec![block] }
	}

	pub fn create_new_block(&mut self) -> usize {
		let label = self.label_count + 1;
		self.blocks.push(IrBlock::new(label));
		label
	}

	pub fn take_blocks(&mut self) -> Vec<IrBlock> {
		self.label_count = 1;
		let blocks = std::mem::take(&mut self.blocks);
		self.blocks = vec![IrBlock::new(1)];
		blocks
	}

	pub fn get_current_block(&mut self) -> &mut IrBlock {
		self.get_block_mut(self.label_count)
	}

	pub fn add_instr(&mut self, instr: Instr) {
		self.get_current_block().add_instr(instr);
	}

	pub fn set_current_block(&mut self, label: usize) {
		let label = label - 1;
		if self.blocks.len() <= label {
			throw_error(format!("block index out of range: {}, blocks: {}", label, self.blocks.len()));
		}
		self.label_count = label;
	}

	pub fn get_block(&self, label: usize) -> &IrBlock {
		let label = label - 1;
		if self.blocks.len() <= label {
			throw_error(format!("block index out of range: {}, blocks: {}", label, self.blocks.len()));
		}
		match self.blocks.get(label) {
			Some(block) => block,
			None => throw_error(format!("block index out of range: {}", label)),
		}
	}

	pub fn get_block_mut(&mut self, label: usize) -> &mut IrBlock {
		let label = label - 1;
		if self.blocks.len() <= label {
			throw_error(format!("block index out of range: {}, blocks: {}", label, self.blocks.len()));
		}
		match self.blocks.get_mut(label) {
			Some(block) => block,
			None => throw_error(format!("block index out of range: {}", label)),
		}
	}
}
