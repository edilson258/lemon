use crate::{
	ast::{self},
	diag::Diag,
	range::Range,
};
use ast::Operator;

use super::{diags::TypeCheckError, types::TypeId, Checker, TypeResult};

impl Checker<'_> {
	pub fn check_binary_expr(&mut self, binary_expr: &mut ast::BinaryExpr) -> TypeResult<TypeId> {
		let left = self.check_expr(&mut binary_expr.left)?;
		let right = self.check_expr(&mut binary_expr.right)?;
		let range = binary_expr.get_range();
		let typed_id = self.apply_operator(&binary_expr.operator, left, right, range)?;
		binary_expr.set_type_id(typed_id);
		Ok(typed_id)
	}

	fn apply_operator(
		&mut self,
		operator: &Operator,
		left: TypeId,
		right: TypeId,
		range: Range,
	) -> TypeResult<TypeId> {
		match operator {
			Operator::ADD | Operator::SUB | Operator::MUL | Operator::DIV => {
				self.numeric_operation(left, right, operator, range)
			}
			Operator::LT | Operator::GT | Operator::LE | Operator::GE | Operator::EQ => {
				self.comparison_operation(left, right, operator, range)
			}
			_ => Err(self.unsupported_operator(left, right, operator, range)),
		}
	}

	fn numeric_operation(
		&self,
		left: TypeId,
		right: TypeId,
		operator: &Operator,
		range: Range,
	) -> TypeResult<TypeId> {
		let left_infered = self.infer_type(right, left)?;
		let right_infered = self.infer_type(left, right)?;

		let resolved_left = self.resolve_par(left)?;
		let resolved_right = self.resolve_par(right)?;

		if !resolved_left.is_numeric() || !resolved_right.is_numeric() {
			return Err(self.unsupported_operator(left, right, operator, range));
		}

		let common_type = self.infer_type(left, right)?;
		Ok(common_type)
	}

	fn comparison_operation(
		&self,
		left: TypeId,
		right: TypeId,
		operator: &Operator,
		range: Range,
	) -> TypeResult<TypeId> {
		let left_infered = self.infer_type(right, left)?;
		let right_infered = self.infer_type(left, right)?;

		let resolved_left = self.resolve_par(left_infered)?;
		let resolved_right = self.resolve_par(right_infered)?;

		if resolved_left != resolved_right {
			return Err(self.unsupported_operator(left, right, operator, range));
		}

		Ok(TypeId::BOOL)
	}

	fn unsupported_operator(
		&self,
		left: TypeId,
		right: TypeId,
		operator: &Operator,
		range: Range,
	) -> Diag {
		TypeCheckError::unsupported_operator(self.format(left), self.format(right), operator, range)
	}
}
