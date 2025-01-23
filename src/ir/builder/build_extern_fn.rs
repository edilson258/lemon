use crate::{ast, ir::ir};

use super::Builder;

impl Builder<'_> {
	pub fn build_extern_fn(&mut self, extern_fn_stmt: &ast::ExternFnStmt) {
		let ret_id = self.get_type_id(extern_fn_stmt.get_ret_id());
		let fn_id = extern_fn_stmt.name.lexeme().to_string();
		let binds = self.build_fn_binds(&extern_fn_stmt.params);
		let fn_native = ir::Fn::new_ex(fn_id, binds, ret_id, extern_fn_stmt.var_packed.is_some());
		self.root.add_fn(fn_native);
		self.ir_ctx.add_fn(extern_fn_stmt.name.lexeme());
	}
}
