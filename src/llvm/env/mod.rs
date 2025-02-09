use inkwell::{
	basic_block::BasicBlock,
	values::{BasicValueEnum, PointerValue},
};
use scope::Scope;
mod scope;

pub struct Env<'ll> {
	pub function_scope: Vec<Scope<'ll>>,
}

impl<'ll> Env<'ll> {
	pub fn new() -> Self {
		Self { function_scope: Vec::new() }
	}

	pub fn enter_function_scope(&mut self) {
		self.function_scope.push(Scope::new());
	}

	pub fn exit_function_scope(&mut self) {
		self.function_scope.pop();
	}

	pub fn get_current_scope(&mut self) -> &mut Scope<'ll> {
		self.function_scope.last_mut().expect("no function scope")
	}

	pub fn get_block(&mut self, name: &str) -> Option<&BasicBlock<'ll>> {
		self.get_current_scope().get_block(name)
	}

	pub fn set_block(&mut self, name: &str, block: BasicBlock<'ll>) {
		self.get_current_scope().set_block(name, block);
	}

	pub fn get_value(&mut self, name: &str) -> Option<&BasicValueEnum<'ll>> {
		self.get_current_scope().get_value(name)
	}

	pub fn set_value(&mut self, name: &str, value: BasicValueEnum<'ll>) {
		self.get_current_scope().set_value(name, value);
	}

	pub fn get_ptr_value(&mut self, name: &str) -> Option<PointerValue<'ll>> {
		self.get_current_scope().get_ptr_value(name)
	}

	pub fn get_temp(&mut self) -> String {
		self.get_current_scope().get_temp()
	}
}
