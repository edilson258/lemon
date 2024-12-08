use crate::{ast, range::TraitRange};

use super::{diags::errs::TypeErr, types::Type, CheckResult, Checker};

impl<'ckr> Checker<'ckr> {
  pub fn check_binary_expr(&mut self, binary: &'ckr ast::BinaryExpr) -> CheckResult<Type> {
    let left_type = self.check_expr(&binary.left)?.unwrap(); // we throw error in parser
    let right_type = self.check_expr(&binary.right)?.unwrap(); // we throw error in parser
    let operator = &binary.operator;
    let range = binary.range();
    if !self.operator_supported(&left_type, &right_type, operator) {
      let diag = TypeErr::not_supportd(&left_type, &right_type, operator, range);
      return Err(diag);
    }

    let result = self.take_common_type(&left_type, &right_type);

    let ty = self.resulting_operator_type(operator, result);
    Ok(Some(ty))
  }

  #[rustfmt::skip]
  fn resulting_operator_type(&self, operator: &'ckr ast::Operator, ty: Type) -> Type {
    match operator {
      ast::Operator::EQ  | ast::Operator::NOTEQ | ast::Operator::LT |
      ast::Operator::GT  | ast::Operator::AND   | ast::Operator::OR |
      ast::Operator::XOR | ast::Operator::BOR   | ast::Operator::LE |
      ast::Operator::GE  | ast::Operator::NOT => Type::Bool,
      _ => ty,
    }
  }
}
