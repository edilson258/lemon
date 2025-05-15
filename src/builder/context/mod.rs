use crate::{
	checker::types::TypeId,
	ir::{BasicValue, IrBasicValue},
	loader::ModId,
	throw_error,
};
use rustc_hash::FxHashMap;
use scope::Scope;

mod block;
mod label;
mod scope;

pub type StructFieldMap = FxHashMap<String, (TypeId, usize)>;
pub type StructDefinitions = FxHashMap<String, StructFieldMap>;

pub struct Context {
	scope_stack: Vec<Scope>,
	next_register_id: usize,
	struct_definitions: StructDefinitions,
	pub struct_sizes: FxHashMap<String, usize>,
	pub current_block: block::Block,
	#[allow(dead_code)]
	pub mod_id: ModId,
}

impl Default for Context {
	fn default() -> Self {
		Self::new()
	}
}

impl Context {
	pub fn new() -> Self {
		Self::new_with_defaults(ModId::default())
	}

	fn new_with_defaults(mod_id: ModId) -> Self {
		Self {
			scope_stack: Vec::new(),
			current_block: block::Block::new(),
			next_register_id: 1,
			struct_definitions: FxHashMap::default(),
			struct_sizes: FxHashMap::default(),
			mod_id,
		}
	}

	pub fn lookup_struct_field(
		&self,
		struct_name: &str,
		field_name: &str,
	) -> Option<(TypeId, usize)> {
		self.struct_definitions.get(struct_name)?.get(field_name).copied()
	}

	pub fn define_struct_fields(&mut self, name: String, fields: StructFieldMap) {
		self.struct_definitions.insert(name, fields);
	}

	pub fn push_scope(&mut self, scope: Scope) {
		self.scope_stack.push(scope);
	}

	pub fn push_function_scope(&mut self, return_type: TypeId) {
		self.push_scope(Scope::new_function_scope(return_type));
	}

	pub fn push_implementation_scope(&mut self, name: impl Into<String>, type_id: TypeId) {
		let receiver_name = name.into();
		let scope = Scope::new_implementation_scope(receiver_name, type_id);
		self.push_scope(scope);
	}

	pub fn push_struct_member_scope(&mut self) {
		self.push_scope(Scope::new_struct_member_scope());
	}

	#[allow(dead_code)]
	pub fn in_function_scope(&self) -> bool {
		self.scope_stack.iter().rev().any(Scope::is_function)
	}

	pub fn in_implementation_scope(&self) -> bool {
		self.scope_stack.iter().rev().any(Scope::is_implementation)
	}

	#[allow(dead_code)]
	pub fn in_struct_member_scope(&self) -> bool {
		self.current_scope().is_struct_member()
	}

	pub fn function_return_type(&self) -> Option<TypeId> {
		self.scope_stack.iter().rev().find_map(Scope::return_type)
	}

	pub fn receiver_info(&self) -> Option<(String, TypeId)> {
		self.scope_stack.iter().rev().find_map(Scope::receiver_info)
	}

	pub fn create_register(&mut self, type_id: TypeId) -> IrBasicValue {
		let register = format!("R{}", self.next_register_id);
		self.next_register_id += 1;
		IrBasicValue::new(BasicValue::Register(register), type_id)
	}

	pub fn create_reference(&mut self, type_id: TypeId, base: Option<TypeId>) -> IrBasicValue {
		let register = format!("R{}", self.next_register_id);
		self.next_register_id += 1;
		let value = BasicValue::Register(register);
		IrBasicValue::new_register(value, type_id, base)
	}

	pub fn pop_scope(&mut self) {
		self.scope_stack.pop();
	}

	fn current_scope(&self) -> &Scope {
		self.scope_stack.last().unwrap_or_else(|| throw_error!("scope not found"))
	}

	fn current_scope_mut(&mut self) -> &mut Scope {
		self.scope_stack.last_mut().unwrap_or_else(|| throw_error!("scope not found"))
	}

	pub fn define_local_variable(&mut self, name: String, value: IrBasicValue) {
		self.current_scope_mut().define_local_variable(name, value);
	}

	pub fn lookup_local_variable(&self, name: &str) -> Option<&IrBasicValue> {
		self.scope_stack.iter().rev().find_map(|scope| scope.lookup_local_variable(name))
	}

	pub fn register_unbound_value(&mut self, value: IrBasicValue) {
		self.current_scope_mut().register_unbound_value(value);
	}

	pub fn collect_unbound_values(&self) -> Vec<IrBasicValue> {
		self.current_scope().collect_unbound_values()
	}
}
