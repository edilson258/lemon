use crate::{
	ast,
	ir::ir::{self},
};

use super::Builder;

impl Builder<'_> {
	pub fn build_impl_stmt(&mut self, impl_stmt: &ast::ImplStmt) {
		let struct_name = impl_stmt.self_name.lexeme();
		for method in impl_stmt.items.iter() {
			let method_name = method.name.lexeme();
			self.build_fn_stmt(method, Some(struct_name.to_owned()));
		}
	}
	pub fn create_bind_method_with_selfname(&self, self_name: &str, method_name: &str) -> String {
		format!("{}__{}", self_name, method_name)
	}
}
