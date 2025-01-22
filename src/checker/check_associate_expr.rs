use super::context::scope::ScopeType;
use super::types::TypeId;
use super::{Checker, TyResult};
use crate::ast;

impl Checker<'_> {
	pub fn check_associate_expr(&mut self, associate: &mut ast::AssociateExpr) -> TyResult<TypeId> {
		let self_type_id = self.check_expr(&mut associate.left)?;
		self.ctx.enter_scope(ScopeType::new_accessor_associate(self_type_id));
		let ret_type = self.check_ident_expr(&mut associate.method)?;
		self.ctx.exit_scope();
		Ok(ret_type)
	}
}
