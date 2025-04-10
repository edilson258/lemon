use super::{diags::SyntaxErr, Checker, TypedValue};
use crate::{ast, message::MessageResult};

impl Checker<'_> {
	pub fn check_const_fn_stmt(
		&mut self,
		const_fn: &mut ast::ConstFnStmt,
	) -> MessageResult<TypedValue> {
		if !self.ctx.is_global_scope() {
			return Err(SyntaxErr::const_outside_global_scope(const_fn.range));
		}
		Ok(TypedValue::default())
	}
}
