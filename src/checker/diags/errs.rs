use crate::{
  ast::Operator,
  checker::types::Type,
  diag::{self, Diag, Severity},
  range::Range,
};

#[derive(Debug, Clone)]
pub enum TypeErr<'ckr> {
  Mismatched {
    expected: &'ckr Type,
    found: &'ckr Type,
    range: Range,
  },
  NotFound {
    name: &'ckr str,
    range: Range,
  },
  OutOfBounds {
    value: &'ckr Type,
    found: &'ckr Type,
    range: Range,
  },
  NotSupported {
    left: &'ckr Type,
    right: &'ckr Type,
    operator: &'ckr Operator,
    range: Range,
  },
  ExpectedValue {
    value: &'ckr Type,
    range: Range,
  },
  NoExpectedValue {
    value: &'ckr Type,
    range: Range,
  },
}

impl<'ckr> TypeErr<'ckr> {
  pub fn mismatched(expected: &'ckr Type, found: &'ckr Type, range: Range) -> Diag {
    Self::Mismatched { expected, found, range }.into()
  }

  pub fn not_found(name: &'ckr str, range: Range) -> Diag {
    Self::NotFound { name, range }.into()
  }

  pub fn out_of_range(value: &'ckr Type, found: &'ckr Type, range: Range) -> Diag {
    Self::OutOfBounds { value, found, range }.into()
  }

  pub fn not_supportd(
    left: &'ckr Type,
    right: &'ckr Type,
    operator: &'ckr Operator,
    range: Range,
  ) -> Diag {
    Self::NotSupported { left, right, operator, range }.into()
  }

  pub fn expected_value(value: &'ckr Type, range: Range) -> Diag {
    Self::ExpectedValue { value, range }.into()
  }

  pub fn no_expected_value(value: &'ckr Type, range: Range) -> Diag {
    Self::NoExpectedValue { value, range }.into()
  }
}

impl<'ckr> From<TypeErr<'ckr>> for diag::Diag {
  fn from(err: TypeErr<'ckr>) -> Self {
    match err {
      TypeErr::Mismatched { expected, found, range } => {
        let text = format!("expected '{}', found '{}'", expected, found);
        diag::Diag::new(Severity::Err, text, range)
      }

      TypeErr::NotFound { name, range } => {
        let text = format!("not found '{}'", name);
        diag::Diag::new(Severity::Err, text, range)
      }

      TypeErr::OutOfBounds { value, found, range } => {
        let text = format!("'{}' out of bounds, expected '{}'", value, found);
        diag::Diag::new(Severity::Err, text, range)
      }

      TypeErr::NotSupported { left, right, operator, range } => {
        let text = format!("not supported: '{}' '{}' '{:?}'", left, right, operator);
        diag::Diag::new(Severity::Err, text, range)
      }

      TypeErr::ExpectedValue { value, range } => {
        let text = format!("expected a value, found '{}'", value);
        diag::Diag::new(Severity::Err, text, range)
      }
      TypeErr::NoExpectedValue { value, range } => {
        let text = format!("no expected value, found '{}'", value);
        diag::Diag::new(Severity::Err, text, range)
      }
    }
  }
}
