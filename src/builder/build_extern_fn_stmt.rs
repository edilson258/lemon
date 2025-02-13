use crate::{ast, ir};

use super::Builder;

impl Builder<'_> {
	pub fn build_extern_fn_stmt(&mut self, fn_stmt: &mut ast::ExternFnStmt) {
		let name = fn_stmt.name.lexeme().to_owned();
		let args: Vec<ir::IrBind> = fn_stmt.params.iter_mut().map(|arg| self.build_bind(arg)).collect();
		let ret = self.build_type(fn_stmt.ret_id, fn_stmt.get_range());
		let comptime = false;
		let mut func = ir::Function::new(name, comptime, args, ret);
		func.as_extern_function(fn_stmt.var_packed.is_some());
		self.push_function_with_blocks(func);
	}
}
