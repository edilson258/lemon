use super::{diags::SyntaxErr, types::TypeId, Checker, TyResult};
use crate::ast::{self, Operator, OperatorKind};

impl Checker<'_> {
	pub fn check_binary_expr(&mut self, binary_expr: &mut ast::BinaryExpr) -> TyResult<TypeId> {
		let left = self.check_expr(&mut binary_expr.left)?;
		let right = self.check_expr(&mut binary_expr.right)?;
		let range = binary_expr.get_range();
		let type_id = match binary_expr.operator.kind {
			OperatorKind::ADD | OperatorKind::SUB | OperatorKind::MUL | OperatorKind::DIV => {
				self.check_math_operation(left, right, &binary_expr.operator)?
			}
			OperatorKind::LT | OperatorKind::GT | OperatorKind::LE => {
				self.check_cmp_operation(left, right, &binary_expr.operator)?
			}
			OperatorKind::GE | OperatorKind::EQ => {
				self.check_cmp_operation(left, right, &binary_expr.operator)?
			}
			OperatorKind::RANGE => self.check_range_operation(left, right, &binary_expr.operator)?,
			OperatorKind::MOD => self.check_mod_operation(left, right, &binary_expr.operator)?,

			// bitwise
			OperatorKind::AND => self.check_bitwise_operation(left, right, &binary_expr.operator)?,
			OperatorKind::OR => self.check_bitwise_operation(left, right, &binary_expr.operator)?,
			OperatorKind::SHL => self.check_bitwise_operation(left, right, &binary_expr.operator)?,
			OperatorKind::XOR => self.check_bitwise_operation(left, right, &binary_expr.operator)?,
			OperatorKind::SHR => self.check_bitwise_operation(left, right, &binary_expr.operator)?,
			_ => todo!(),
		};
		let t = self.infer_type(right, left)?;
		let t = self.infer_type(t, right)?;
		binary_expr.set_type_id(t);
		Ok(type_id)
	}

	fn check_bitwise_operation(
		&self,
		lt: TypeId,
		rt: TypeId,
		operator: &Operator,
	) -> TyResult<TypeId> {
		let left = self.infer_type(rt, lt)?;
		let right = self.infer_type(lt, rt)?;
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

	fn check_range_operation(&self, lt: TypeId, rt: TypeId, operator: &Operator) -> TyResult<TypeId> {
		todo!("check range operator")
	}

	fn check_cmp_operation(&self, lt: TypeId, rt: TypeId, operator: &Operator) -> TyResult<TypeId> {
		let left = self.infer_type(rt, lt)?;
		let right = self.infer_type(lt, rt)?;
		if !self.equal_type_id(left, right) {
			let (left, right) = self.display_double_type(left, right);
			return Err(SyntaxErr::unsupported_operator(left, right, operator));
		}
		Ok(TypeId::BOOL)
	}

	fn check_math_operation(&self, lt: TypeId, rt: TypeId, operator: &Operator) -> TyResult<TypeId> {
		let left = self.infer_type(rt, lt)?;
		let right = self.infer_type(left, rt)?;
		if !self.equal_type_id(left, right) {
			let (left, right) = self.display_double_type(left, right);
			return Err(SyntaxErr::unsupported_operator(left, right, operator));
		}
		Ok(left)
	}

	fn check_mod_operation(&self, lt: TypeId, rt: TypeId, operator: &Operator) -> TyResult<TypeId> {
		let left = self.infer_type(rt, lt)?;
		let right = self.infer_type(lt, rt)?;

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
