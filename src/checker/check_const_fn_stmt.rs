use super::{diags::SyntaxErr, Checker};
use crate::{ast, message::MessageResult};

use super::types::TypeId;

impl Checker<'_> {
	pub fn check_const_fn_stmt(&mut self, const_fn: &mut ast::ConstFnStmt) -> MessageResult<TypeId> {
		if !self.ctx.is_global_scope() {
			return Err(SyntaxErr::const_outside_global_scope(const_fn.range));
		}
		Ok(TypeId::UNIT)
	}
}
