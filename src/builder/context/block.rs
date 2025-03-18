use super::label::Label;
use crate::{
	ir::{Instr, IrBlock},
	report::throw_error,
};

pub struct Block {
	current_label: Label,
	next_label: Label,
	ir_blocks: Vec<IrBlock>,
	pub has_returned: bool,
}

impl Default for Block {
	fn default() -> Self {
		Self::new()
	}
}

impl Block {
	pub fn new() -> Self {
		let current_label = Label::default();
		let first_block = IrBlock::new(current_label.into());
		Self {
			current_label,
			next_label: current_label,
			ir_blocks: vec![first_block],
			has_returned: false,
		}
	}

	pub fn mark_as_returned(&mut self) {
		self.has_returned = true;
	}

	pub fn create_new_block(&mut self) -> Label {
		let label = self.next_label.next();
		self.next_label = label;
		self.ir_blocks.push(IrBlock::new(label.into()));
		label
	}

	pub fn extract_blocks(&mut self) -> Vec<IrBlock> {
		self.reset_state();
		std::mem::take(&mut self.ir_blocks)
	}

	pub fn append_instr(&mut self, instr: Instr) {
		self.current_block_mut().append_instr(instr);
	}

	pub fn switch_to_label(&mut self, label: Label) {
		self.has_returned = false;
		self.current_label = label;
		self.validate_label_index(label);
	}

	pub fn block_mut(&mut self, label: Label) -> &mut IrBlock {
		self.validate_label_index(label);
		&mut self.ir_blocks[label.id.wrapping_sub(1)]
	}

	fn current_block_mut(&mut self) -> &mut IrBlock {
		self.block_mut(self.current_label)
	}

	fn reset_state(&mut self) {
		self.has_returned = false;
		self.current_label = Label::default();
		self.next_label = self.current_label;
		self.ir_blocks = vec![IrBlock::new(self.current_label.into())];
	}

	fn validate_label_index(&self, label: Label) {
		let index = label.id.wrapping_sub(1);
		if index >= self.ir_blocks.len() {
			throw_error(format!(
				"Block index out of range: {}, blocks: {}",
				label.id,
				self.ir_blocks.len()
			));
		}
	}
}
