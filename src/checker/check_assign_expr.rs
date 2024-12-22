use crate::ast::{self};

use super::{context::value::Value, diags::TypeCheckError, Checker, TypeResult};

impl Checker<'_> {
	pub fn check_assign_expr(&mut self, assign_expr: &ast::AssignExpr) -> TypeResult {
		let left = self.check_expr(&assign_expr.left)?;
		let value = self.check_expr(&assign_expr.right)?;
		if !left.is_mutable() {
			return Err(TypeCheckError::immutable("unknown", assign_expr.left.get_range()));
		}
		if value.type_id == left.type_id {
			return Ok(Value::nothing());
		}
		let left_type = self.ctx.get_type(left.type_id).unwrap();
		let value_type = self.ctx.get_type(value.type_id).unwrap();
		if !left_type.is_eq(value_type, &self.ctx.type_store) {
			return Err(TypeCheckError::type_mismatch(
				&self.ctx.type_store,
				left_type,
				value_type,
				assign_expr.get_range(),
			));
		}
		Ok(Value::nothing())
	}
}
