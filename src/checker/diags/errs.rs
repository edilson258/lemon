use crate::{
  ast::Operator,
  checker::types::Type,
  diag::{self, Severity},
  range::Range,
};

#[derive(Debug, Clone)]
pub enum TypeErr<'a> {
  Mismatched(&'a Type, &'a Type, Range),
  NotFound(&'a str, Range),
  Unsupported(Range),
  OutOfRange(&'a Type, &'a Type, Range),
  NotSupported(&'a Type, &'a Type, &'a Operator, Range),
  ExpectedValue(&'a Type, Range),
  NoExpectedValue(&'a Type, Range),
  NotInScope(&'a Type, Range),
  NotAFunction(&'a Type, Range),
  NotAnExpression(&'a Type, Range),
}

impl<'a> From<TypeErr<'a>> for diag::Diag {
  fn from(err: TypeErr) -> Self {
    match err {
      TypeErr::Mismatched(expected, actual, range) => {
        let text = format!("expected '{}', found '{}'", expected, actual);
        diag::Diag::new(Severity::Err, text, range)
      }
      TypeErr::NoExpectedValue(expected, range) => {
        let text = format!("no expected value, found '{}'", expected);
        diag::Diag::new(Severity::Err, text, range)
      }
      TypeErr::NotFound(name, range) => {
        let text = format!("not found '{}' in scope", name);
        diag::Diag::new(Severity::Err, text, range)
      }
      TypeErr::Unsupported(range) => {
        let text = "unsupported value".to_owned();
        diag::Diag::new(Severity::Err, text, range)
      }
      TypeErr::OutOfRange(value, expected, range) => {
        let text = format!("'{}' is out of range, must be '{}'", value, expected);
        diag::Diag::new(Severity::Err, text, range)
      }
      TypeErr::ExpectedValue(name, range) => {
        let text = format!("expected '{}', found nothing", name);
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
      TypeErr::NotSupported(lf, rt, operator, range) => {
        let text = match operator {
          Operator::ADD => format!("cannot add '{}' and '{}'", lf, rt),
          Operator::SUB => format!("cannot subtract '{}' from '{}'", lf, rt),
          Operator::MUL => format!("cannot multiply '{}' by '{}'", lf, rt),
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
    }
  }
}
