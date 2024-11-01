#![allow(dead_code)]

use std::mem::take;

use serde::{Deserialize, Serialize};

use crate::utils::range::Range;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Severity {
  Err,
  Warn,
  Info,
}

impl Severity {
  pub fn to_string(&self) -> &str {
    match self {
      Severity::Err => "error",
      Severity::Warn => "warning",
      Severity::Info => "info",
    }
  }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Diag {
  pub severity: Severity,
  pub message: String,
  pub range: Range,
  pub filename: String,
}

impl Diag {
  pub fn new(severity: Severity, message: String, range: Range, filename: String) -> Self {
    Self { severity, message, range, filename }
  }

  pub fn create_err(message: String, range: Range, filename: String) -> Self {
    Self::new(Severity::Err, message, range, filename)
  }

  pub fn create_warn(message: String, range: Range, filename: String) -> Self {
    Self::new(Severity::Warn, message, range, filename)
  }

  pub fn create_info(message: String, range: Range, filename: String) -> Self {
    Self::new(Severity::Info, message, range, filename)
  }

  pub fn get_range(&self) -> &Range {
    &self.range
  }
}

pub struct DiagGroup {
  pub diags: Vec<Diag>,
}

impl DiagGroup {
  pub fn new() -> Self {
    Self { diags: Vec::new() }
  }

  pub fn add(&mut self, diag: Diag) {
    self.diags.push(diag);
  }

  pub fn report(&self) {
    //
  }

  pub fn ok(&mut self) -> Result<(), Self> {
    if self.diags.is_empty() {
      Ok(())
    } else {
      // report here?
      Err(DiagGroup { diags: take(&mut self.diags) })
    }
  }
}
