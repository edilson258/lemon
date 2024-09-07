use crate::utils::range::Range;

use super::diag::{Diag, Severity};

pub enum DiagWarning<'a> {
  UnusedValue { range: &'a Range, name: String },
  UnusedFn { range: &'a Range, name: String },
  UnusedModule { range: &'a Range, name: String },
  UnusedType { range: &'a Range, name: String },
}

impl<'a> From<DiagWarning<'a>> for Diag<'a> {
  fn from(warning: DiagWarning<'a>) -> Self {
    let severity = Severity::Warning;
    match warning {
      DiagWarning::UnusedValue { range, name } => {
        let message = format!("unused value `{}` in this scope.", name);
        let hint = format!("maybe you forgot to use it?");
        Diag::new(severity, message, Some(hint), range)
      }

      DiagWarning::UnusedFn { range, name } => {
        let message = format!("unused function `{}` in this scope.", name);
        let hint = format!("maybe you forgot to use it?");
        Diag::new(severity, message, Some(hint), range)
      }

      DiagWarning::UnusedModule { range, name } => {
        let message = format!("unused module `{}`.", name);
        let hint = format!("maybe you forgot to use it?");
        Diag::new(severity, message, Some(hint), range)
      }

      DiagWarning::UnusedType { range, name } => {
        let message = format!("unused type `{}`.", name);
        let hint = format!("maybe you forgot to use it?");
        Diag::new(severity, message, Some(hint), range)
      }
    }
  }
}
