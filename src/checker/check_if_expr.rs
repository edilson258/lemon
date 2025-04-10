use super::types::TypeId;
use super::{Checker, TypedValue};
use crate::ast;
use crate::message::MessageResult;

impl Checker<'_> {
	pub fn check_if_expr(&mut self, if_expr: &mut ast::IfExpr) -> MessageResult<TypedValue> {
		let cond_type = self.check_expr(&mut if_expr.cond)?;
		self.equal_type_expected(TypeId::BOOL, cond_type.type_id, if_expr.cond.get_range())?;
		let then_type = self.check_expr(&mut if_expr.then)?;
		let otherwise_type = self.check_expr(&mut if_expr.otherwise)?;
		println!("{}", self.equal_type_id(then_type.type_id, otherwise_type.type_id));
		self.equal_type_expected(
			then_type.type_id,
			otherwise_type.type_id,
			if_expr.otherwise.get_range(),
		)?;
		Ok(TypedValue { type_id: then_type.type_id, ptr: 0 })
	}
}
