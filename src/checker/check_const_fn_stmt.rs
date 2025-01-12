use super::{diags::SyntaxErr, Checker, TyResult};
use crate::ast;

use super::types::TypeId;

impl Checker<'_> {
	pub fn check_const_fn_stmt(&mut self, const_fn: &mut ast::ConstFnStmt) -> TyResult<TypeId> {
		if !self.ctx.is_global_scope() {
			return Err(SyntaxErr::const_outside_global_scope(const_fn.range.clone()));
		}
		Ok(TypeId::UNIT)
	}
}
