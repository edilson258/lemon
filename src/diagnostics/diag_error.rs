use crate::utils::range::Range;

use super::diag::{Diag, Severity};

pub enum DiagError<'a> {
  UnknownType { range: &'a Range, name: String },
  UnknownFn { range: &'a Range, name: String },
  NotFoundValue { range: &'a Range, name: String },
  NotFoundModule { range: &'a Range, name: String },
  Expected { range: &'a Range, expected: String, found: String },
}

impl<'a> From<DiagError<'a>> for Diag<'a> {
  fn from(error: DiagError<'a>) -> Self {
    let severity = Severity::Error;
    match error {
      DiagError::Expected { range, expected, found } => {
        let message = format!("expected `{}`, found `{}`.", expected, found);
        let hint = format!("did you mean `{}`?", expected);
        Diag::new(severity, message, Some(hint), range)
      }
      DiagError::UnknownType { range, name } => {
        let message = format!("unknown type `{}`.", name);
        let hint = format!("you forgot to import it? or maybe define it?");
        Diag::new(Severity::Error, message, Some(hint), range)
      }
      DiagError::UnknownFn { range, name } => {
        let message = format!("unknown function `{}`.", name);
        let hint = format!("you forgot to import it? or maybe declare it?");
        Diag::new(Severity::Error, message, Some(hint), range)
      }
      DiagError::NotFoundValue { range, name } => {
        let message = format!("value `{}` not found.", name);
        let hint = format!("did you forgot to declare it?");
        Diag::new(Severity::Error, message, None, range)
      }
      DiagError::NotFoundModule { range, name } => {
        let message = format!("module `{}` not found.", name);
        let hint = format!("did you forget to create it?");
        Diag::new(Severity::Error, message, None, range)
      }
    }
  }
}
