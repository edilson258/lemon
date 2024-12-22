use crate::ast::{self};

// use super::types::TypeId;
use super::types::{RefType, Type, TypeId};
use super::{Checker, TypeResult};

impl Checker<'_> {
	pub fn check_ref_expr(&mut self, ref_expr: &ast::RefExpr) -> TypeResult<TypeId> {
		let found_id = self.check_expr(&ref_expr.expr)?;
		let ref_type = RefType::new(ref_expr.mutable.is_some(), found_id);
		Ok(self.ctx.type_store.add_type(Type::Ref(ref_type)))
	}
}
