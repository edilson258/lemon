use crate::{
	ast::{FnStmt, ImplStmt},
	ir,
};

use super::Builder;

impl Builder<'_> {
	pub fn build_impl_stmt(&mut self, impl_stmt: &mut ImplStmt) {
		let self_name = impl_stmt.self_name.lexeme();
		let self_type = self.type_store.get_type_by_name(self_name).copied();

		let self_type = self.build_type(self_type, impl_stmt.get_range());
		self.ctx.push_impl_scope(self_name, self_type);

		for item in impl_stmt.items.iter_mut() {
			self.build_method(impl_stmt.self_name.lexeme(), item);
		}
	}

	pub fn create_bind_method_with_selfname(&mut self, self_name: &str, method_name: &str) -> String {
		format!("{}.{}", self_name, method_name)
	}

	fn build_method(&mut self, self_name: &str, method: &mut FnStmt) {
		let method_name = self.create_bind_method_with_selfname(self_name, method.lexeme());
		let ret_type = self.build_type(method.ret_id, method.get_range());
		self.ctx.push_function_scope(ret_type);

		let args: Vec<_> = method.params.iter_mut().map(|arg| self.build_bind(arg)).collect();

		if let Some(self_ref) = args.first() {
			let ref_name = self_ref.value.as_str();
			self.ctx.add_self_ref(ref_name.to_string());
		}

		let ret = self.build_type(method.ret_id, method.get_range());
		let comptime = false;
		let func = ir::Function::new(method_name, comptime, args, ret);
		self.build_fn_body(&mut method.body);
		self.ctx.pop_scope();
		self.push_function_with_blocks(func);
	}
}
