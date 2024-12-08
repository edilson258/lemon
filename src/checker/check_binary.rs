use crate::ast;

use super::{diags::errs::TypeErr, types::Type, Checker, CheckerResult};

impl<'a> Checker<'a> {
  pub fn check_binary_expr(&mut self, binary: &ast::BinaryExpr) -> CheckerResult<Type> {
    let left_type = self.check_expr(&binary.left)?.unwrap();
    let right_type = self.check_expr(&binary.right)?.unwrap();
    let operator = &binary.operator;
    let range = binary.range.clone();
    if !self.operator_supported(&left_type, &right_type, operator) {
      let diag = TypeErr::NotSupported(&left_type, &right_type, operator, range);
      return Err(diag.into());
    }
    let result = self.resulting_type(&left_type, &right_type);
    let ty = self.resulting_operator_type(operator, result);
    Ok(Some(ty))
  }

  fn resulting_operator_type(&self, operator: &ast::Operator, ty: Type) -> Type {
    match operator {
      ast::Operator::ADD
      | ast::Operator::SUB
      | ast::Operator::MUL
      | ast::Operator::DIV
      | ast::Operator::MOD
      | ast::Operator::RANGE
      | ast::Operator::SHL
      | ast::Operator::SHR
      | ast::Operator::POW
      | ast::Operator::PIPE
      | ast::Operator::ADDEQ
      | ast::Operator::SUBEQ
      | ast::Operator::MULEQ
      | ast::Operator::DIVEQ
      | ast::Operator::MODEQ => ty,
      ast::Operator::EQ => Type::Bool,
      ast::Operator::NOTEQ => Type::Bool,
      ast::Operator::LT => Type::Bool,
      ast::Operator::GT => Type::Bool,
      ast::Operator::AND => Type::Bool,
      ast::Operator::OR => Type::Bool,
      ast::Operator::XOR => Type::Bool,
      ast::Operator::BOR => Type::Bool,
      ast::Operator::LE => Type::Bool,
      ast::Operator::GE => Type::Bool,
      ast::Operator::NOT => Type::Bool,
    }
  }
}
