use super::context::scope::ScopeType;
use super::types::TypeId;
use super::{Checker, TyResult};
use crate::ast;

impl Checker<'_> {
	pub fn check_member_expr(&mut self, member_expr: &mut ast::MemberExpr) -> TyResult<TypeId> {
		println!("expr: {:#?}", member_expr);
		let self_type = self.check_expr(&mut member_expr.left)?;
		self.ctx.enter_scope(ScopeType::new_accessor_method(self_type));
		let ret = self.check_ident_expr(&mut member_expr.method)?;
		self.ctx.exit_scope();
		Ok(ret)
	}
}
