use scope::{Local, Scope};

use crate::checker::types::TypeId;
mod block;
mod scope;

pub struct Context {
	pub scopes: Vec<Scope>,
	pub block: block::Block,
}
impl Default for Context {
	fn default() -> Self {
		Self::new()
	}
}

impl Context {
	pub fn new() -> Context {
		Context { scopes: Vec::new(), block: block::Block::new() }
	}

	pub fn push_scope(&mut self) {
		self.scopes.push(Scope::new());
	}

	pub fn pop_scope(&mut self) {
		self.scopes.pop();
	}

	pub fn get_current_scope(&self) -> &Scope {
		self.scopes.last().unwrap()
	}

	pub fn get_current_scope_mut(&mut self) -> &mut Scope {
		self.scopes.last_mut().unwrap()
	}

	pub fn add_local(&mut self, name: String, type_id: TypeId) {
		self.get_current_scope_mut().add_local(name, type_id);
	}

	pub fn get_local(&self, name: &str) -> Option<&Local> {
		self.get_current_scope().get_local(name)
	}

	pub fn get_local_mut(&mut self, name: &str) -> Option<&mut Local> {
		self.get_current_scope_mut().get_local_mut(name)
	}
}
