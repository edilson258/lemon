use rustc_hash::FxHashMap;
use scope::Scope;

use crate::{
	checker::types::TypeId,
	ir::{BasicValue, IrBasicValue},
	report::throw_ir_build_error,
};
mod block;
mod label;
mod scope;

pub type FieldTable = FxHashMap<String, (TypeId, usize)>;

pub type StructTable = FxHashMap<String, FieldTable>;

pub struct Context {
	pub scopes: Vec<Scope>,
	pub block: block::Block,
	pub register_count: usize,
	pub struct_table: StructTable,
	// todo: improve it
	pub struct_table_size: FxHashMap<String, usize>,
}
impl Default for Context {
	fn default() -> Self {
		Self::new()
	}
}

impl Context {
	pub fn new() -> Context {
		let struct_table = StructTable::default();
		let block = block::Block::new();
		let scopes = Vec::new();
		let struct_table_size = FxHashMap::default();
		Context { scopes, block, register_count: 1, struct_table, struct_table_size }
	}

	pub fn get_struct_field(&self, struct_name: &str, field_name: &str) -> Option<(TypeId, usize)> {
		self.struct_table.get(struct_name).and_then(|fields| fields.get(field_name).copied())
	}

	pub fn set_struct_field(&mut self, struct_name: String, table: FieldTable) {
		self.struct_table.insert(struct_name, table);
	}

	#[allow(dead_code)]
	pub fn push_scope(&mut self) {
		self.scopes.push(Scope::new());
	}

	pub fn push_function_scope(&mut self, ret_type: TypeId) {
		self.scopes.push(Scope::new_function(ret_type));
	}

	pub fn push_impl_scope(&mut self, self_name: impl Into<String>, self_type: TypeId) {
		self.scopes.push(Scope::new_impl(self_name.into(), self_type));
	}

	pub fn push_member_scope(&mut self) {
		self.scopes.push(Scope::new_member());
	}

	#[allow(dead_code)]
	pub fn is_function_scope(&self) -> bool {
		self.scopes.iter().rev().any(|scope| scope.is_function_scope())
	}

	pub fn is_impl_scope(&self) -> bool {
		self.scopes.iter().rev().any(|scope| scope.is_impl_scope())
	}

	pub fn is_member_scope(&self) -> bool {
		// todo: we need to find all scops?
		self.get_current_scope().is_member_scope()
	}

	pub fn get_ret_type(&self) -> Option<TypeId> {
		for scope in self.scopes.iter().rev() {
			if scope.is_function_scope() {
				return scope.get_ret_type();
			}
		}
		None
	}

	pub fn get_self_info(&self) -> Option<(String, TypeId)> {
		for scope in self.scopes.iter().rev() {
			if scope.is_impl_scope() {
				return scope.get_self_info();
			}
		}
		None
	}

	pub fn new_register(&mut self, type_id: TypeId) -> IrBasicValue {
		let register = format!("r{}", self.register_count);
		self.register_count += 1;
		let register_value = BasicValue::Register(register);

		IrBasicValue::new(register_value, type_id)
	}

	pub fn pop_scope(&mut self) {
		self.scopes.pop();
	}

	pub fn get_current_scope(&self) -> &Scope {
		match self.scopes.last() {
			Some(scope) => scope,
			None => throw_ir_build_error("scope not found"),
		}
	}

	pub fn get_current_scope_mut(&mut self) -> &mut Scope {
		match self.scopes.last_mut() {
			Some(scope) => scope,
			None => throw_ir_build_error("scope not found"),
		}
	}

	pub fn add_local(&mut self, key: String, basic_value: IrBasicValue) {
		self.get_current_scope_mut().add_local(key, basic_value);
	}

	pub fn get_local(&self, name: &str) -> Option<&IrBasicValue> {
		self.scopes.iter().rev().find_map(|scope| scope.get_local(name))
	}

	pub fn add_dont_load(&mut self, key: impl Into<String>) {
		self.get_current_scope_mut().add_dont_load(key);
	}

	pub fn is_dont_load(&self, key: &str) -> bool {
		self.get_current_scope().is_dont_load(key)
	}

	pub fn add_free_value(&mut self, value: IrBasicValue) {
		self.get_current_scope_mut().add_free_value(value);
	}

	pub fn get_free_values(&self) -> Vec<IrBasicValue> {
		self.get_current_scope().get_free_values()
	}
}
