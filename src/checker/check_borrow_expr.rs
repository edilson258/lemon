use super::{typed_value::TypedValue, types::BorrowType, CheckResult, Checker, ExpectSome};
use crate::ast;

impl Checker<'_> {
	pub fn check_borrow_expr(&mut self, expr: &mut ast::BorrowExpr) -> CheckResult {
		let range = expr.get_range();
		let mut target = self.check_expr(&mut expr.expr).some(range)?;

		let target_id = self.infer_default_type(target.type_id);

		let borrow_type = BorrowType::new(target_id, expr.mutable.is_some());
		let target_id = self.ctx.type_store.add_type(borrow_type.into());

		target.infer_type(target_id);
		self.register_type(target_id, range);

		let borrow_result = if expr.mutable.is_some() {
			self.ctx.borrow.borrow_mutable(&mut target)
		} else {
			self.ctx.borrow.borrow_immutable(&mut target)
		};

		let ref_id = match borrow_result {
			Ok(id) => id,
			Err(err) => return Err(err.range(range)),
		};

		Ok(Some(TypedValue::new(target.type_id, ref_id)))
	}
}
