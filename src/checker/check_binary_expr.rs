use crate::{
	ast::{self},
	range::Range,
};
use ast::Operator;

use super::{diags::TypeCheckError, types::TypeId, Checker, TypeResult};

impl Checker<'_> {
	pub fn check_binary_expr(&mut self, binary_expr: &ast::BinaryExpr) -> TypeResult<TypeId> {
		let left = self.check_expr(&binary_expr.left)?;
		let right = self.check_expr(&binary_expr.right)?;
		let range = binary_expr.get_range();
		match binary_expr.operator {
			Operator::ADD => {
				self.can_apply(left, right, Operator::ADD, range)?;
				Ok(left)
			}
			Operator::SUB => {
				self.can_apply(left, right, Operator::SUB, range)?;
				Ok(left)
			}
			Operator::MUL => {
				self.can_apply(left, right, Operator::MUL, range)?;
				Ok(left)
			}
			Operator::LT => {
				self.can_apply(left, right, Operator::LT, range)?;
				Ok(TypeId::BOOL)
			}
			Operator::GT => {
				self.can_apply(left, right, Operator::GT, range)?;
				Ok(TypeId::BOOL)
			}
			Operator::LE => {
				self.can_apply(left, right, Operator::LE, range)?;
				Ok(TypeId::BOOL)
			}
			Operator::GE => {
				self.can_apply(left, right, Operator::GE, range)?;
				Ok(TypeId::BOOL)
			}
			Operator::EQ => {
				self.can_apply(left, right, Operator::EQ, range)?;
				Ok(TypeId::BOOL)
			}
			_ => todo!(),
		}
	}

	fn can_apply(&self, left: TypeId, right: TypeId, op: Operator, range: Range) -> TypeResult<()> {
		let right_id = self.infer_type(left, right)?;
		let left_id = self.infer_type(right, left)?;

		let left_type = self.resolve_par(left_id)?;
		let right_type = self.resolve_par(right_id)?;

		if left_type != right_type {
			let left = self.format(left_id);
			let right = self.format(right_id);
			return Err(TypeCheckError::unsupported_operator(left, right, &op, range));
		}
		Ok(())
	}
}
