use super::report::report_diag;
use crate::utils::{range::Range, source::Source};

#[derive(Debug)]
pub enum Severity {
  Error,
  Warning,
  Info,
}

#[derive(Debug)]
pub struct Diag<'a> {
  pub severity: Severity,
  pub message: String,
  pub hint: Option<String>,
  pub range: &'a Range,
}

impl<'a> Diag<'a> {
  pub fn new(severity: Severity, message: String, hint: Option<String>, range: &'a Range) -> Self {
    Self { severity, message, hint, range }
  }

  pub fn add_hint(&mut self, hint: String) {
    self.hint = Some(hint);
  }
}

pub struct Diags<'a> {
  list: Vec<Diag<'a>>,
}

impl<'a> Diags<'a> {
  pub fn new() -> Self {
    Self { list: Vec::new() }
  }

  pub fn add(&mut self, diag: Diag<'a>) {
    self.list.push(diag);
  }

  pub fn error(&mut self, message: String, hint: Option<String>, range: &'a Range) {
    self.add(Diag::new(Severity::Error, message, hint, range));
  }

  pub fn warning(&mut self, message: String, hint: Option<String>, range: &'a Range) {
    self.add(Diag::new(Severity::Warning, message, hint, range));
  }

  pub fn show(&self, source: &'a Source) {
    self.list.iter().for_each(|d| report_diag(d, &source));
  }

  pub fn show_and_exit(&self, source: &'a Source) -> ! {
    self.show(source);
    std::process::exit(1);
  }

  pub fn has_error(&self) -> bool {
    self.list.iter().any(|d| matches!(d.severity, Severity::Error))
  }

  pub fn has_warning(&self) -> bool {
    self.list.iter().any(|d| matches!(d.severity, Severity::Warning))
  }

  pub fn has_info(&self) -> bool {
    self.list.iter().any(|d| matches!(d.severity, Severity::Info))
  }

  pub fn empty(&self) -> bool {
    self.list.is_empty()
  }
}
