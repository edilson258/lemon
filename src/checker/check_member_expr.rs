use super::context::scope::ScopeKind;
use super::types::Type;
use super::{CheckResult, Checker, ExpectSome};
use crate::ast;

impl Checker<'_> {
	pub fn check_member_expr(&mut self, member_expr: &mut ast::MemberExpr) -> CheckResult {
		let range = member_expr.get_range();
		let self_value = self.check_expr(&mut member_expr.left).some(range)?;
		self.ctx.enter_scope(ScopeKind::accessor(self_value.type_id, false));
		let ret_value = self.check_ident_expr(&mut member_expr.method).some(range)?;
		// todo: refactor
		if !self.ctx.type_store.is_module(self_value.type_id) {
			if let Type::Fn(fn_type) = self.lookup_stored_mut_type(ret_value.type_id) {
				fn_type.args.remove(0);
			}
		}
		self.ctx.exit_scope();
		// removo self type
		self.register_type(self_value.type_id, member_expr.get_range());
		Ok(Some(ret_value))
	}
}
