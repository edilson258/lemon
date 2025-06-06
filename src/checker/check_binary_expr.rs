use super::{
	diags::SyntaxErr, typed_value::TypedValue, types::TypeId, CheckResult, Checker, ExpectSome,
};
use crate::{
	ast::{self, Operator, OperatorKind},
	message::MessageResult,
	range::Range,
};

impl Checker<'_> {
	pub fn check_binary_expr(&mut self, binary_expr: &mut ast::BinaryExpr) -> CheckResult {
		let left = self.check_expr(&mut binary_expr.left).some(binary_expr.left.get_range())?;
		let right = self.check_expr(&mut binary_expr.right).some(binary_expr.right.get_range())?;

		let range = binary_expr.get_range();
		let operator = &binary_expr.operator;
		self.check_binary_operator(left.type_id, right.type_id, operator, range)
	}

	fn check_binary_operator(
		&mut self,
		left: TypeId,
		right: TypeId,
		operator: &Operator,
		range: Range,
	) -> CheckResult {
		use OperatorKind::*;
		let left = self.infer_type_from_expected(right, left);
		let right = self.infer_type_from_expected(left, right);
		let found_id = match operator.kind {
			// math
			ADD | SUB | MUL | DIV => self._check_math_operator(left, right, operator)?,

			// compare
			GT | LE | GE | EQ | LT => {
				let type_id = self.unify_types(left, right)?.unwrap_or(left);
				self.register_type(type_id, range);
				self.register_type(TypeId::BOOL, operator.get_range());
				return self.check_cmp_operator(left, right, operator);
			}

			// range and mod
			RANGE => self._check_range_operator(left, right, operator)?,
			MOD => self._check_mod_operator(left, right, operator)?,

			// bitwise
			AND | OR | SHL | XOR => self._check_bitwise(left, right, operator)?,
			SHR => self._check_bitwise(left, right, operator)?,
			_ => todo!(),
		};
		self.register_type(found_id, range);
		self.register_type(found_id, operator.get_range());
		let owner = self.ctx.borrow.create_owner();
		Ok(Some(TypedValue::new(found_id, owner)))
	}

	fn _check_bitwise(
		&self,
		left: TypeId,
		right: TypeId,
		operator: &Operator,
	) -> MessageResult<TypeId> {
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
		&mut self,
		left: TypeId,
		right: TypeId,
		operator: &Operator,
	) -> CheckResult {
		if !self.equal_type_id(left, right) {
			let (left, right) = self.display_double_type(left, right);
			return Err(SyntaxErr::unsupported_operator(left, right, operator));
		}
		let owner = self.ctx.borrow.create_owner();
		Ok(Some(TypedValue::new(TypeId::BOOL, owner)))
	}

	fn _check_math_operator(
		&self,
		left: TypeId,
		right: TypeId,
		operator: &Operator,
	) -> MessageResult<TypeId> {
		if !self.equal_type_id(left, right) {
			let (left, right) = self.display_double_type(left, right);
			return Err(SyntaxErr::unsupported_operator(left, right, operator));
		}
		Ok(left)
	}

	fn _check_mod_operator(
		&self,
		left: TypeId,
		right: TypeId,
		operator: &Operator,
	) -> MessageResult<TypeId> {
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
