#![allow(dead_code)]

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
  range::Range,
  report::{report_err, report_info, report_warn},
  source::Source,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Severity {
  Err,
  Warn,
  Info,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Diag {
  pub severity: Severity,
  pub message: String,
  pub range: Range,
}

impl Diag {
  pub fn new(severity: Severity, message: String, range: Range) -> Self {
    Self { severity, message, range }
  }

  pub fn error(message: impl Into<String>, range: Range) -> Self {
    Self::new(Severity::Err, message.into(), range)
  }

  pub fn warning(message: impl Into<String>, range: Range) -> Self {
    Self::new(Severity::Warn, message.into(), range)
  }

  pub fn info(message: impl Into<String>, range: Range) -> Self {
    Self::new(Severity::Info, message.into(), range)
  }

  pub fn range(&self) -> &Range {
    &self.range
  }

  pub fn report(&self, path: &PathBuf) {
    match self.severity {
      Severity::Err => report_err(self, path),
      Severity::Warn => report_warn(self, path),
      Severity::Info => report_info(self, path),
    }
  }

  pub fn report_wrap(&self, path: &PathBuf) -> ! {
    self.report(path);
    std::process::exit(1);
  }
}

pub struct DiagGroup<'a> {
  pub diags: Vec<Diag>,
  pub source: &'a Source,
}

impl<'a> DiagGroup<'a> {
  pub fn new(source: &'a Source) -> Self {
    Self { diags: Vec::new(), source }
  }
  pub fn add(&mut self, diag: Diag) {
    self.diags.push(diag);
  }
  pub fn report(&self, path: &PathBuf) {
    for diag in &self.diags {
      diag.report(path);
    }
  }
}
