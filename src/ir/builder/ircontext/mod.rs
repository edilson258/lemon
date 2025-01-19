use std::{collections::HashMap, mem};

use crate::{
	checker::types::TypeId,
	ir::ir::{self, Block, BlockId},
	report::throw_ir_build_error,
};

type Scope = HashMap<String, ir::Register>;

pub struct IrContext {
	register: ir::Register,
	scopes: Vec<Scope>,
	ret_type: Option<TypeId>,
	types: HashMap<ir::Register, TypeId>,
	fns: HashMap<String, ir::FnId>,
	blocks: Vec<Block>,
	block_id: BlockId,
}
impl Default for IrContext {
	fn default() -> Self {
		Self::new()
	}
}

impl IrContext {
	pub fn new() -> Self {
		let register = ir::Register::new(0);
		let scopes = vec![HashMap::new()];
		let fns = HashMap::new();
		let block_id = BlockId::new(0);
		let blocks = vec![];
		let types = HashMap::new();
		Self { register, scopes, types, fns, blocks, block_id, ret_type: None }
	}

	pub fn set_ret_type(&mut self, ret: Option<TypeId>) {
		self.ret_type = ret;
	}

	pub fn get_ret_type(&self) -> Option<&TypeId> {
		self.ret_type.as_ref()
	}

	pub fn add_type(&mut self, register: ir::Register, type_id: TypeId) {
		self.types.insert(register, type_id);
	}

	pub fn get_type(&self, register: ir::Register) -> Option<&TypeId> {
		self.types.get(&register)
	}

	pub fn get_register_size(&self) -> usize {
		self.register.as_usize()
	}

	pub fn is_global_scope(&self) -> bool {
		self.scopes.len() == 1
	}

	pub fn add_fn(&mut self, name: &str) {
		self.fns.insert(name.to_owned(), ir::FnId::new(name));
	}

	pub fn get_fn_id(&self, name: &str) -> Option<&ir::FnId> {
		self.fns.get(name)
	}

	pub fn get_current_register(&mut self) -> ir::Register {
		self.register
	}

	pub fn new_register(&mut self) -> ir::Register {
		let register = self.register.next_register();
		self.register = register;
		register
	}

	pub fn enter_scope(&mut self) {
		self.scopes.push(HashMap::new());
		let block_id = self.new_block();
		self.switch_to_block(block_id);
	}

	pub fn exit_scope(&mut self) {
		if self.scopes.pop().is_none() {
			throw_ir_build_error("scope not found");
		}
	}

	pub fn get_current_scope(&mut self) -> &mut HashMap<String, ir::Register> {
		if let Some(scope) = self.scopes.last_mut() {
			return scope;
		}
		throw_ir_build_error("scope not found");
	}

	pub fn add_value(&mut self, name: &str, register: ir::Register) {
		self.get_current_scope().insert(name.to_string(), register);
	}

	pub fn get_value(&self, name: &str) -> Option<&ir::Register> {
		self.scopes.iter().rev().find_map(|scope| scope.get(name))
	}

	pub fn get_mut_block(&mut self) -> &mut Block {
		if let Some(block) = self.blocks.get_mut(self.block_id.as_usize()) {
			return block;
		}
		throw_ir_build_error(format!("block '{}' not found", self.block_id.as_usize()));
	}

	pub fn add_instr(&mut self, instr: ir::Instr) {
		self.get_mut_block().add_instr(instr);
	}

	pub fn new_block(&mut self) -> BlockId {
		let id = BlockId::new(self.blocks.len());
		self.blocks.push(Block::new(id));
		id
	}

	pub fn get_current_block_id(&self) -> BlockId {
		self.block_id
	}

	pub fn get_next_block_id(&mut self) -> BlockId {
		self.block_id.next_block()
	}

	pub fn switch_to_block(&mut self, block_id: BlockId) {
		if block_id.as_usize() >= self.blocks.len() {
			let error = format!(
				"out of bounds blocks, max '{}' found '{}'",
				self.blocks.len(),
				block_id.as_usize()
			);
			throw_ir_build_error(error);
		}
		self.block_id = block_id;
	}

	pub fn reset_block(&mut self) {
		self.blocks.clear();
		self.block_id = BlockId::new(0);
	}

	pub fn reset_fn_scope(&mut self) -> Vec<Block> {
		let blocks = mem::take(&mut self.blocks);
		self.reset_block();
		blocks
	}
}
