use crate::{
	ast::{self},
	range::Range,
};

use super::{types::TypeId, Checker, TypeResult};

impl Checker<'_> {
	pub fn check_binary_expr(&mut self, binary_expr: &ast::BinaryExpr) -> TypeResult<TypeId> {
		let left = self.check_expr(&binary_expr.left)?;
		let right = self.check_expr(&binary_expr.right)?;
		let range = binary_expr.get_range();
		match binary_expr.operator {
			ast::Operator::LT => {
				self.is_compatible(left, right, range)?;
				Ok(TypeId::BOOL)
			}
			ast::Operator::GT => {
				self.is_compatible(left, right, range)?;
				Ok(TypeId::BOOL)
			}
			ast::Operator::LE => {
				self.is_compatible(left, right, range)?;
				Ok(TypeId::BOOL)
			}
			ast::Operator::GE => {
				self.is_compatible(left, right, range)?;
				Ok(TypeId::BOOL)
			}
			ast::Operator::EQ => {
				self.is_compatible(left, right, range)?;
				Ok(TypeId::BOOL)
			}
			_ => todo!(),
		}
	}
	fn is_compatible(&self, left: TypeId, right: TypeId, range: Range) -> TypeResult<()> {
		self.equal_type_id(left, right, range)
	}
}
