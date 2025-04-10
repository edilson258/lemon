use super::context::scope::ScopeKind;
use super::types::Type;
use super::{Checker, TypedValue};
use crate::ast;
use crate::message::MessageResult;

impl Checker<'_> {
	pub fn check_member_expr(
		&mut self,
		member_expr: &mut ast::MemberExpr,
	) -> MessageResult<TypedValue> {
		let self_value = self.check_expr(&mut member_expr.left)?;
		self.ctx.enter_scope(ScopeKind::accessor(self_value.type_id, false));
		let ret = self.check_ident_expr(&mut member_expr.method)?;
		// todo: refactor
		if !self.ctx.type_store.is_module(self_value.type_id) {
			if let Type::Fn(fn_type) = self.get_stored_mut_type(ret.type_id) {
				fn_type.args.remove(0);
			}
		}

		self.ctx.exit_scope();
		// removo self type
		self.register_type(self_value.type_id, member_expr.get_range());
		Ok(ret)
	}
}
