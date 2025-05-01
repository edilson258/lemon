use super::{diags::SyntaxErr, CheckResult, Checker};
use crate::ast;

impl Checker<'_> {
	pub fn check_const_fn_stmt(&mut self, c: &mut ast::ConstFnStmt) -> CheckResult {
		if !self.ctx.is_global_scope() {
			return Err(SyntaxErr::const_outside_global_scope(c.range));
		}
		todo!()
		// Ok(TypedValue::default())
	}
}
