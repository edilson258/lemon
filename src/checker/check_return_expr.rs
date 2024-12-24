use crate::ast::{self};
use crate::range::Range;

use super::diags::TypeCheckError;
use super::types::{Type, TypeId};
use super::{Checker, TypeResult};
impl Checker<'_> {
	pub fn check_return_expr(&mut self, ret_expr: &mut ast::RetExpr) -> TypeResult<TypeId> {
		if !self.ctx.has_fn_scope() {
			return Err(TypeCheckError::return_outside_fn(ret_expr.get_range()));
		}
		let ret_scope_id = self.ctx.get_fn_scope_ret_type().unwrap(); // we know it's a fn
		if let Some(value_expr) = &mut ret_expr.value {
			let type_id = self.check_expr(value_expr)?;
			let ret_id = self.check_ref_return_value(type_id, ret_expr.get_range())?;
			self.equal_type_id(ret_scope_id, ret_id, ret_expr.get_range())?;
			return Ok(ret_id);
		}
		self.equal_type_id(ret_scope_id, TypeId::NOTHING, ret_expr.get_range())?;
		Ok(TypeId::NOTHING)
	}

	fn check_ref_return_value(&mut self, ret_id: TypeId, range: Range) -> TypeResult<TypeId> {
		let ret_type = self.get_stored_type(ret_id)?;
		match ret_type {
			Type::Par { target } => Ok(*target),
			Type::Ref { .. } => Err(TypeCheckError::connot_return_local_rerefence(range)),
			_ => Ok(ret_id),
		}
	}
}
