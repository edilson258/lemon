use super::context::scope::ScopeType;
use super::types::{Type, TypeId};
use super::{Checker, TyResult};
use crate::ast;

impl Checker<'_> {
	pub fn check_member_expr(&mut self, member_expr: &mut ast::MemberExpr) -> TyResult<TypeId> {
		let self_type = self.check_expr(&mut member_expr.left)?;
		self.ctx.enter_scope(ScopeType::new_accessor_method(self_type));
		let ret = self.check_ident_expr(&mut member_expr.method)?;

		if let Type::Fn(fn_type) = self.get_stored_mut_type(ret) {
			fn_type.args.remove(0);
		}
		self.ctx.exit_scope();
		// removo self type
		member_expr.set_left_type(self_type);
		Ok(ret)
	}
}
