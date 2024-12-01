use crate::{
  ast::Operator,
  diag::{self, Severity},
  range::Range,
};

#[derive(Debug, Clone)]
pub enum TypeErr {
  Mismatched(String, String, Range),
  UnsupportedOperator(String, String, Operator, Range),
  OutRange(String, String, String, String, Range),
  ExpectedValue(String, Range),
  NotInScope(String, Range),
  NotAFunction(String, Range),
  NotAnExpression(String, Range),
}

impl From<TypeErr> for diag::Diag {
  fn from(err: TypeErr) -> Self {
    match err {
      TypeErr::Mismatched(expected, actual, range) => {
        let text = format!("expected '{}' but found '{}'", expected, actual);
        diag::Diag::new(Severity::Err, text, range)
      }
      TypeErr::ExpectedValue(name, range) => {
        let text = format!("expected value but found '{}'", name);
        diag::Diag::new(Severity::Err, text, range)
      }
      TypeErr::NotInScope(name, range) => {
        let text = format!("not in scope '{}'", name);
        diag::Diag::new(Severity::Err, text, range)
      }
      TypeErr::NotAFunction(name, range) => {
        let text = format!("not a function '{}'", name);
        diag::Diag::new(Severity::Err, text, range)
      }
      TypeErr::NotAnExpression(name, range) => {
        let text = format!("not an expression '{}'", name);
        diag::Diag::new(Severity::Err, text, range)
      }
      TypeErr::UnsupportedOperator(lf, rt, operator, range) => {
        let text = match operator {
          Operator::ADD => format!("cannot add '{}' and '{}'", lf, rt),
          Operator::SUB => format!("cannot subtract '{}' from '{}'", lf, rt),
          Operator::MUL => format!("cannot multiply '{}' and '{}'", lf, rt),
          Operator::DIV => format!("cannot divide '{}' by '{}'", lf, rt),
          Operator::MOD => format!("cannot take modulus of '{}' and '{}'", lf, rt),
          Operator::EQ => format!("cannot compare '{}' and '{}'", lf, rt),
          Operator::AND => format!("cannot perform 'and' on '{}' and '{}'", lf, rt),
          Operator::OR => format!("cannot perform 'or' on '{}' and '{}'", lf, rt),
          Operator::NOTEQ => format!("cannot compare '{}' and '{}'", lf, rt),
          Operator::LT => format!("'{}' cannot be less than '{}'", lf, rt),
          Operator::GT => format!("'{}' cannot be greater than '{}'", lf, rt),
          Operator::LE => format!("'{}' cannot be less than or equal to '{}'", lf, rt),
          Operator::GE => format!("'{}' cannot be greater than or equal to '{}'", lf, rt),
          Operator::RANGE => format!("cannot concatenate '{}' and '{}'", lf, rt),
          _ => todo!(),
        };
        diag::Diag::new(Severity::Err, text, range)
      }
      // value `300` out of range for `u8` (0..=255)
      TypeErr::OutRange(value, tyy, start, end, range) => {
        let text = format!("value '{}' out of range for '{}' ({}..={})", value, tyy, start, end);
        diag::Diag::new(Severity::Err, text, range)
      }
    }
  }
}
