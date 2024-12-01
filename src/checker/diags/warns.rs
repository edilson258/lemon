use crate::{
  diag::{self, Severity},
  range::Range,
};

#[derive(Debug, Clone)]
pub enum TypeWarn {
  Unused(String, Range),
}

impl From<TypeWarn> for diag::Diag {
  fn from(warn: TypeWarn) -> Self {
    match warn {
      TypeWarn::Unused(name, range) => {
        let text = format!("unused value '{}'", name);
        diag::Diag::new(Severity::Warn, text, range)
      }
    }
  }
}
