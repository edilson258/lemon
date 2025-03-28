use super::{diags::SyntaxErr, types::TypeId, Checker, TyResult};
use crate::ast::{self, Operator, OperatorKind};

impl Checker<'_> {
	pub fn check_binary_expr(&mut self, binary_expr: &mut ast::BinaryExpr) -> TyResult<TypeId> {
		let left = self.check_expr(&mut binary_expr.left)?;
		let right = self.check_expr(&mut binary_expr.right)?;
		let range = binary_expr.get_range();
		let operator = &binary_expr.operator;
		let type_id = match operator.kind {
			// math
			OperatorKind::ADD | OperatorKind::SUB => self._check_math_operator(left, right, operator)?,
			OperatorKind::MUL | OperatorKind::DIV => self._check_math_operator(left, right, operator)?,

			// compare
			OperatorKind::GT | OperatorKind::LE => self.check_cmp_operator(left, right, operator)?,
			OperatorKind::GE | OperatorKind::EQ => self.check_cmp_operator(left, right, operator)?,
			OperatorKind::LT => self.check_cmp_operator(left, right, operator)?,

			// range and mod
			OperatorKind::RANGE => self._check_range_operator(left, right, operator)?,
			OperatorKind::MOD => self._check_mod_operator(left, right, operator)?,

			// bitwise
			OperatorKind::AND | OperatorKind::OR => self._check_bitwise(left, right, operator)?,
			OperatorKind::SHL | OperatorKind::XOR => self._check_bitwise(left, right, operator)?,
			OperatorKind::SHR => self._check_bitwise(left, right, operator)?,
			_ => todo!(),
		};
		binary_expr.set_type_id(type_id);
		Ok(type_id)
	}

	fn _check_bitwise(&self, _left: TypeId, _right: TypeId, operator: &Operator) -> TyResult<TypeId> {
		let left = self.infer_type_from_expected(_right, _left);
		let right = self.infer_type_from_expected(_left, _right);
		if !left.is_int() || !right.is_int() {
			let (left, right) = self.display_double_type(left, right);
			return Err(SyntaxErr::unsupported_operator(left, right, operator));
		}
		if !self.equal_type_id(left, right) {
			let (left, right) = self.display_double_type(left, right);
			return Err(SyntaxErr::unsupported_operator(left, right, operator));
		}
		Ok(left)
	}

	fn _check_range_operator(&self, lt: TypeId, rt: TypeId, operator: &Operator) -> TyResult<TypeId> {
		todo!("check range operator")
	}

	fn check_cmp_operator(&self, lt: TypeId, rt: TypeId, operator: &Operator) -> TyResult<TypeId> {
		let left = self.infer_type_from_expected(rt, lt);
		let right = self.infer_type_from_expected(lt, rt);
		if !self.equal_type_id(left, right) {
			let (left, right) = self.display_double_type(left, right);
			return Err(SyntaxErr::unsupported_operator(left, right, operator));
		}
		Ok(TypeId::BOOL)
	}

	fn _check_math_operator(&self, lt: TypeId, rt: TypeId, operator: &Operator) -> TyResult<TypeId> {
		let left = self.infer_type_from_expected(rt, lt);
		let right = self.infer_type_from_expected(left, rt);
		if !self.equal_type_id(left, right) {
			let (left, right) = self.display_double_type(left, right);
			return Err(SyntaxErr::unsupported_operator(left, right, operator));
		}
		Ok(left)
	}

	fn _check_mod_operator(&self, lt: TypeId, rt: TypeId, operator: &Operator) -> TyResult<TypeId> {
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
