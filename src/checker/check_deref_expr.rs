use super::diags::SyntaxErr;
use super::{CheckResult, Checker, ExpectSome, TypedValue};
use crate::ast;

impl Checker<'_> {
	pub fn check_deref_expr(&mut self, deref_expr: &mut ast::DerefExpr) -> CheckResult {
		let range = deref_expr.get_range();
		let reference = self.check_expr(&mut deref_expr.expr).some(range)?;

		if reference.type_id.is_builtin_type() {
			let ty = self.display_type(reference.type_id);
			return Err(SyntaxErr::cannot_dereference(ty, range));
		}

		if let Some(borrow) = self.lookup_stored_borrow(reference.type_id).cloned() {
			self.register_type(borrow.value, range);
			let result = TypedValue::new_source(borrow.value, reference.source);
			return Ok(Some(result));
		}

		let ty = self.display_type(reference.type_id);
		Err(SyntaxErr::cannot_dereference(ty, range))
	}
}
