use crate::ast::{self};

use super::{context::scope::ScopeKind, diags::SyntaxErr, types::TypeId, Checker, TyResult};

impl Checker<'_> {
	pub fn check_impl_stmt(&mut self, impl_stmt: &mut ast::ImplStmt) -> TyResult<TypeId> {
		let self_name = impl_stmt.self_name.lexeme();
		let self_type_id = self.ctx.type_store.get_type_by_name(self_name).copied();
		if self_type_id.is_none() {
			return Err(SyntaxErr::not_found_type(self_name, impl_stmt.self_name.get_range()));
		}

		// todo: is correct?

		let self_type_id = self_type_id.unwrap();
		let self_type = self.get_stored_mut_type(self_type_id);

		if !self_type.can_implemented() {
			return Err(SyntaxErr::expect_instaced_type(
				self.display_type(self_type_id),
				impl_stmt.self_name.get_range(),
			));
		}

		self_type.set_impl(true);
		self.ctx.enter_scope(ScopeKind::implementation(self_type_id));
		for item in impl_stmt.items.iter_mut() {
			self.check_fn_stmt(item)?;
		}

		self.ctx.exit_scope();

		Ok(TypeId::UNIT)
	}
}
