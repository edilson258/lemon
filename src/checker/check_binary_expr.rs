use super::{diags::SyntaxErr, types::TypeId, Checker, TypedValue};
use crate::{
	ast::{self, Operator, OperatorKind},
	message::MessageResult,
};

impl Checker<'_> {
	pub fn check_binary_expr(
		&mut self,
		binary_expr: &mut ast::BinaryExpr,
	) -> MessageResult<TypedValue> {
		let left = self.check_expr(&mut binary_expr.left)?;
		let right = self.check_expr(&mut binary_expr.right)?;
		let range = binary_expr.get_range();
		let operator = &binary_expr.operator;
		let type_id = match operator.kind {
			// math
			OperatorKind::ADD | OperatorKind::SUB | OperatorKind::MUL | OperatorKind::DIV => {
				self._check_math_operator(left.type_id, right.type_id, operator)?
			}
			// compare
			OperatorKind::GT | OperatorKind::LE | OperatorKind::GE | OperatorKind::EQ => {
				self.check_cmp_operator(left.type_id, right.type_id, operator)?
			}
			OperatorKind::LT => self.check_cmp_operator(left.type_id, right.type_id, operator)?,

			// range and mod
			OperatorKind::RANGE => self._check_range_operator(left.type_id, right.type_id, operator)?,
			OperatorKind::MOD => self._check_mod_operator(left.type_id, right.type_id, operator)?,

			// bitwise
			OperatorKind::AND | OperatorKind::OR | OperatorKind::SHL | OperatorKind::XOR => {
				self._check_bitwise(left.type_id, right.type_id, operator)?
			}
			OperatorKind::SHR => self._check_bitwise(left.type_id, right.type_id, operator)?,
			_ => todo!(),
		};
		self.register_type(type_id, range);
		Ok(self.owned_typed_value(type_id))
	}

	fn _check_bitwise(
		&self,
		_left: TypeId,
		_right: TypeId,
		operator: &Operator,
	) -> MessageResult<TypeId> {
		let left = self.infer_type_from_expected(_right, _left);
		let right = self.infer_type_from_expected(_left, _right);
		if !left.is_int_type() || !right.is_int_type() {
			let (left, right) = self.display_double_type(left, right);
			return Err(SyntaxErr::unsupported_operator(left, right, operator));
		}
		if !self.equal_type_id(left, right) {
			let (left, right) = self.display_double_type(left, right);
			return Err(SyntaxErr::unsupported_operator(left, right, operator));
		}
		Ok(left)
	}

	fn _check_range_operator(
		&self,
		lt: TypeId,
		rt: TypeId,
		operator: &Operator,
	) -> MessageResult<TypeId> {
		todo!("check range operator")
	}

	fn check_cmp_operator(
		&self,
		lt: TypeId,
		rt: TypeId,
		operator: &Operator,
	) -> MessageResult<TypeId> {
		let left = self.infer_type_from_expected(rt, lt);
		let right = self.infer_type_from_expected(lt, rt);
		if !self.equal_type_id(left, right) {
			let (left, right) = self.display_double_type(left, right);
			return Err(SyntaxErr::unsupported_operator(left, right, operator));
		}
		Ok(TypeId::BOOL)
	}

	fn _check_math_operator(
		&self,
		lt: TypeId,
		rt: TypeId,
		operator: &Operator,
	) -> MessageResult<TypeId> {
		let left = self.infer_type_from_expected(rt, lt);
		let right = self.infer_type_from_expected(left, rt);
		if !self.equal_type_id(left, right) {
			let (left, right) = self.display_double_type(left, right);
			return Err(SyntaxErr::unsupported_operator(left, right, operator));
		}
		Ok(left)
	}

	fn _check_mod_operator(
		&self,
		lt: TypeId,
		rt: TypeId,
		operator: &Operator,
	) -> MessageResult<TypeId> {
		let left = self.infer_type_from_expected(rt, lt);
		let right = self.infer_type_from_expected(lt, rt);

		if left.is_float() || right.is_float() {
			let (left, right) = self.display_double_type(left, right);
			return Err(SyntaxErr::unsupported_operator(left, right, operator));
		}

		if !self.equal_type_id(left, right) {
			let (left, right) = self.display_double_type(left, right);
			return Err(SyntaxErr::unsupported_operator(left, right, operator));
		}
		Ok(left)
	}
}
