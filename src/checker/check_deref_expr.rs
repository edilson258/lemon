use crate::{
	ast::{self},
	checker::diags::TypeCheckError,
};

use super::types::{Type, TypeId};
use super::{Checker, TypeResult};
impl Checker<'_> {
	pub fn check_deref_expr(&mut self, deref_expr: &ast::DerefExpr) -> TypeResult<TypeId> {
		let ref_id = self.check_expr(&deref_expr.expr)?;
		match self.resolve_par(ref_id)? {
			Type::Ref(ref_type) => Ok(ref_type.value),
			_ => Err(TypeCheckError::deref_of_non_ref(deref_expr.get_range())),
		}
	}
}
