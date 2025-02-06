use std::mem;

use inkwell::values::PointerValue;
use rustc_hash::{FxHashMap, FxHashSet};

use crate::{
	checker::types::TypeId,
	ir::{
		ir::{self, Block, BlockId},
		Register,
	},
	report::throw_ir_build_error,
};

type Scope = FxHashMap<String, ir::Register>;

struct StructInfoTable {
	pub fields: FxHashMap<String, Register>,
	// pub methods: FxHashMap<String, FnId>,
}
impl StructInfoTable {
	pub fn new() -> Self {
		let fields = FxHashMap::default();
		// let methods = FxHashMap::default();
		Self { fields }
	}

	pub fn add_field(&mut self, field: &str, register: Register) {
		self.fields.insert(field.to_string(), register);
	}

	pub fn get_field(&self, field: &str) -> Option<&Register> {
		self.fields.get(field)
	}
}

impl Default for StructInfoTable {
	fn default() -> Self {
		Self::new()
	}
}

pub struct IrContext {
	register: ir::Register,
	scopes: Vec<Scope>,
	ret_type: Option<TypeId>,
	types: FxHashMap<ir::Register, TypeId>,
	values: FxHashSet<String>,
	blocks: Vec<Block>,
	struct_table: FxHashMap<String, StructInfoTable>,
	struct_register: FxHashMap<Register, String>,
	block_id: BlockId,
	pub ret_owner: Option<Register>,
}
impl Default for IrContext {
	fn default() -> Self {
		Self::new()
	}
}

impl IrContext {
	pub fn new() -> Self {
		let register = ir::Register::new(0);
		let scopes = vec![FxHashMap::default()];
		let block_id = BlockId::new(0);
		let blocks = vec![];
		let types = FxHashMap::default();
		let values = FxHashSet::default();
		let struct_table = FxHashMap::default();
		let ret_type = None;
		let struct_register = FxHashMap::default();
		let ret_owner = None;
		Self {
			ret_owner,
			register,
			scopes,
			types,
			blocks,
			block_id,
			ret_type,
			values,
			struct_table,
			struct_register,
		}
	}

	// =================
	// structs
	//

	pub fn register_struct(&mut self, self_value: Register, atual: Register) {
		if let Some(struct_name) = self.get_struct_register(self_value) {
			self.struct_register.insert(atual, struct_name.to_owned());
			return;
		}
		println!("not found {}", self_value.as_string());
	}

	pub fn set_ret_owner(&mut self, register: Register) {
		self.ret_owner = Some(register);
	}

	pub fn get_ret_owner(&self) -> Option<&Register> {
		self.ret_owner.as_ref()
	}

	pub fn register_struct_name(&mut self, register: Register, name: &str) {
		self.struct_register.insert(register, name.into());
	}

	pub fn get_struct_register(&self, register: Register) -> Option<&str> {
		self.struct_register.get(&register).map(|s| s.as_str())
	}

	pub fn resolve_register_struct(&self, register: Register) -> String {
		if let Some(name) = self.get_struct_register(register) {
			return name.to_string();
		}
		throw_ir_build_error(format!("struct register '{}' not found", register.as_string()));
	}

	pub fn add_struct(&mut self, name: &str) {
		self.struct_table.insert(name.to_string(), StructInfoTable::new());
	}

	pub fn add_struct_field(&mut self, name: &str, field: &str, register: Register) {
		if !self.struct_table.contains_key(name) {
			self.add_struct(name);
		}
		self.struct_table.get_mut(name).unwrap().add_field(field, register);
	}

	pub fn get_struct_field_by_register(&self, reg: Register, filed: &str) -> Register {
		let self_name = self.resolve_register_struct(reg);
		self.get_struct_field_register(&self_name, filed)
	}

	pub fn get_struct_field_register(&self, name: &str, filed: &str) -> Register {
		if let Some(struct_table) = self.struct_table.get(name) {
			if let Some(register) = struct_table.get_field(filed) {
				return *register;
			}
			throw_ir_build_error(format!("field '{}' not found in struct '{}'", filed, name));
		}
		throw_ir_build_error(format!("struct '{}' not found", name));
	}

	// ===========
	//
	pub fn add_ir_value(&mut self, name: &str) {
		self.values.insert(name.to_string());
	}

	pub fn get_ir_value(&self, name: &str) -> Option<&String> {
		self.values.get(name)
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
		self.values.insert(name.to_string());
	}

	pub fn get_fn_id(&self, name: &str) -> Option<String> {
		self.values.get(name).map(|value| value.to_string())
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
		self.scopes.push(FxHashMap::default());
		let block_id = self.new_block();
		self.switch_to_block(block_id);
	}

	pub fn exit_scope(&mut self) {
		if self.scopes.pop().is_none() {
			throw_ir_build_error("scope not found");
		}
	}

	pub fn get_current_scope(&mut self) -> &mut FxHashMap<String, ir::Register> {
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
