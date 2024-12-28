use std::{collections::HashMap, mem};

use crate::ir::ir::{self, Block, BlockId};
type Scope = HashMap<String, ir::Register>;

pub struct Context {
	pub register: usize,
	pub values: Vec<Scope>,
	pub fns: HashMap<String, ir::FnId>,
	pub blocks: Vec<Block>,
	pub active_block: BlockId,
	pub comptime: bool,
}

impl Context {
	pub fn new() -> Self {
		let register = 0;
		let values = Vec::from_iter(vec![HashMap::new()]);
		let active_block = BlockId::new(0);
		let blocks = Vec::from_iter(vec![Block::new(active_block)]);
		let fns = HashMap::new();
		Self { register, values, blocks, fns, comptime: false, active_block }
	}

	pub fn is_comptime(&self) -> bool {
		self.comptime
	}

	pub fn enter_comptime(&mut self) {
		self.comptime = true;
	}

	pub fn exit_comptime(&mut self) {
		self.comptime = false;
	}

	pub fn enter_scope(&mut self) {
		self.values.push(HashMap::new());
	}

	pub fn exit_scope(&mut self) {
		self.values.pop();
	}

	pub fn exit_fn_scope(&mut self) -> Vec<Block> {
		self.values.pop();
		let blocks = mem::take(&mut self.blocks);
		let active_block_id = BlockId::new(0);
		self.active_block = active_block_id;
		self.blocks.push(Block::new(active_block_id));
		blocks
	}

	pub fn add_fn(&mut self, name: &str) {
		let fn_id = ir::FnId::new(name);
		self.fns.insert(name.to_owned(), fn_id);
	}

	pub fn get_fn_id(&self, name: &str) -> Option<ir::FnId> {
		self.fns.get(name).cloned()
	}

	pub fn get_register(&mut self) -> ir::Register {
		let register = self.register;
		self.register += 1;
		ir::Register::new(register)
	}

	pub fn get_current_scope(&mut self) -> &mut Scope {
		self.values.last_mut().expect("no irscope")
	}

	pub fn add_value(&mut self, name: &str, register: ir::Register) {
		self.get_current_scope().insert(name.to_owned(), register);
	}

	pub fn get_value(&self, name: &str) -> Option<&ir::Register> {
		self.values.iter().rev().find_map(|scope| scope.get(name))
	}

	pub fn get_mut_block(&mut self) -> Option<&mut Block> {
		let index = self.active_block.as_usize();
		self.blocks.get_mut(index)
	}
	pub fn add_instr(&mut self, instr: ir::Instr) {
		if let Some(block) = self.get_mut_block() {
			block.add_instr(instr)
		} else {
			panic!("no block to add instr");
		}
	}
	pub fn create_block(&mut self) -> BlockId {
		let block_id = BlockId::new(self.blocks.len());
		self.blocks.insert(block_id.as_usize(), Block::new(block_id));
		block_id
	}

	pub fn switch_to_block(&mut self, block_id: BlockId) {
		if block_id.as_usize() >= self.blocks.len() {
			panic!("invalid block id");
		}
		self.active_block = block_id;
	}
}

impl Default for Context {
	fn default() -> Self {
		Self::new()
	}
}
