use super::{diags::SyntaxErr, Checker, TyResult};
use crate::ast;

use super::types::TypeId;

impl Checker<'_> {
	pub fn check_const_del_stmt(&mut self, const_del: &mut ast::ConstDelStmt) -> TyResult<TypeId> {
		if !self.ctx.is_global_scope() {
			return Err(SyntaxErr::const_outside_global_scope(const_del.range.clone()));
		}

		Ok(TypeId::NOTHING)
	}

	pub fn check_const_bind(&mut self, const_bind: &mut ast::Binding) -> TyResult<TypeId> {
		todo!()
	}
}
