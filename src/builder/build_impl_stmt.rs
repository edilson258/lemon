use crate::{
	ast::{FnStmt, ImplStmt},
	ir,
};

use super::Builder;

impl Builder<'_> {
	pub fn build_impl_stmt(&mut self, impl_stmt: &mut ImplStmt) {
		let self_name = impl_stmt.self_name.lexeme();
		let self_type = self.type_store.lookup_type_definition(self_name).copied();
		let self_type = self_type.unwrap_or_else(|| {
			self.internal_error_with_range(
				"could not resolve type of self",
				impl_stmt.self_name.get_range(),
				self.loader,
			)
		});
		self.ctx.push_implementation_scope(self_name, self_type);

		for item in impl_stmt.items.iter_mut() {
			self.build_method(impl_stmt.self_name.lexeme(), item);
		}
	}

	#[inline(always)]
	pub fn create_bind_method_with_selfname(&mut self, self_name: &str, method_name: &str) -> String {
		format!("{}.{}", self_name, method_name)
	}

	fn build_method(&mut self, self_name: &str, method: &mut FnStmt) {
		let method_name = self.create_bind_method_with_selfname(self_name, method.lexeme());
		let ret_type = self.lookup_event_type(method.get_range());
		self.ctx.push_function_scope(ret_type);

		let args: Vec<_> = method.params.iter_mut().map(|arg| self.build_bind(arg)).collect();

		// if let Some(self_ref) = args.first() {
		// let ref_name = self_ref.value.as_str();
		// self.ctx.add_self_ref(ref_name.to_string());
		// }

		let ret = self.lookup_event_type(method.get_range());
		let comptime = false;
		let func = ir::Function::new(method_name, comptime, args, ret);
		self.build_fn_body(&mut method.body);
		self.ctx.pop_scope();
		self.push_function_with_blocks(func);
	}
}
