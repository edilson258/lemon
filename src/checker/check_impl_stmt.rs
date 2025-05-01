use crate::ast;

use super::{context::scope::ScopeKind, diags::SyntaxErr, CheckResult, Checker};

impl Checker<'_> {
	pub fn check_impl_stmt(&mut self, impl_stmt: &mut ast::ImplStmt) -> CheckResult {
		let self_name = impl_stmt.self_name.lexeme();
		let self_type_id = self.ctx.type_store.lookup_type_definition(self_name).copied();
		if self_type_id.is_none() {
			return Err(SyntaxErr::not_found_type(self_name, impl_stmt.self_name.get_range()));
		}
		// todo: is correct?
		let self_type_id = self_type_id.unwrap();
		let self_type = self.lookup_stored_mut_type(self_type_id);

		if !self_type.can_implemented() {
			let t = self.display_type(self_type_id);
			return Err(SyntaxErr::expect_instaced_type(t, impl_stmt.self_name.get_range()));
		}
		self_type.set_impl(true);
		self.ctx.enter_scope(ScopeKind::implementation(self_type_id));
		for item in impl_stmt.items.iter_mut() {
			self.check_fn_stmt(item)?;
		}
		self.ctx.exit_scope();
		Ok(None)
	}
}
