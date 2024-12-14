#![allow(dead_code)]

use core::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{
  range::Range,
  report::{report_err, report_syntax_err, report_type_err},
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

  pub fn report_syntax_err(&self, path: &PathBuf) {
    report_syntax_err(self, path);
  }

  pub fn report_type_err(&self, path: &PathBuf) {
    report_type_err(self, path);
  }

  pub fn report_err(&self, path: &PathBuf) {
    report_err(self, path);
  }

  pub fn report_syntax_err_wrap(&self, path: &PathBuf) -> ! {
    self.report_syntax_err(path);
    std::process::exit(1);
  }

  pub fn report_type_err_wrap(&self, path: &PathBuf) -> ! {
    self.report_type_err(path);
    std::process::exit(1);
  }

  pub fn report_err_wrap(&self, path: &PathBuf) -> ! {
    self.report_err(path);
    std::process::exit(1);
  }
}

pub struct DiagGroup<'ckr> {
  pub diags: Vec<Diag>,
  pub source: &'ckr Source,
}

impl<'ckr> DiagGroup<'ckr> {
  pub fn new(source: &'ckr Source) -> Self {
    Self { diags: Vec::new(), source }
  }
  pub fn add(&mut self, diag: Diag) {
    self.diags.push(diag);
  }
  pub fn report(&self, path: &PathBuf) {
    for diag in &self.diags {
      diag.report_err(path);
    }
  }
}

impl fmt::Display for Diag {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    writeln!(f, "severity: {}", self.severity)?;
    writeln!(f, "message: {}", self.message)?;
    writeln!(f, "range: {}", self.range)
  }
}

impl fmt::Display for Severity {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Severity::Err => write!(f, "error"),
      Severity::Warn => write!(f, "warning"),
      Severity::Info => write!(f, "info"),
    }
  }
}
