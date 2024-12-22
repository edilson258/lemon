#![allow(dead_code)]

use core::fmt;

use serde::{Deserialize, Serialize};

use crate::{
	range::Range,
	report::{self},
	source::Source,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Severity {
	Err,
	Warn,
	Note,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Diag {
	pub severity: Severity,
	pub message: String,
	pub note: Option<String>,
	pub range: Range,
}

impl Diag {
	pub fn new(severity: Severity, message: String, range: Range) -> Self {
		Self { severity, message, note: None, range }
	}

	pub fn error(message: impl Into<String>, range: Range) -> Self {
		Self::new(Severity::Err, message.into(), range)
	}

	pub fn warning(message: impl Into<String>, range: Range) -> Self {
		Self::new(Severity::Warn, message.into(), range)
	}

	pub fn note(message: impl Into<String>, range: Range) -> Self {
		Self::new(Severity::Note, message.into(), range)
	}

	pub fn with_note(mut self, note: impl Into<String>) -> Self {
		self.note = Some(note.into());
		self
	}
	pub fn get_range(&self) -> &Range {
		&self.range
	}

	pub fn report_syntax_err(&self, source: &Source) {
		report::report_syntax_err(self, source);
	}

	pub fn report_type_err(&self, source: &Source) {
		report::report_type_err(self, source);
	}

	pub fn report_err(&self, source: &Source) {
		report::report_err(self, source);
	}

	pub fn report_syntax_err_wrap(&self, source: &Source) -> ! {
		self.report_syntax_err(source);
		std::process::exit(1);
	}

	pub fn report_type_err_wrap(&self, source: &Source) -> ! {
		self.report_type_err(source);
		std::process::exit(1);
	}

	pub fn report_err_wrap(&self, source: &Source) -> ! {
		self.report_err(source);
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
	pub fn report(&self, source: &Source) {
		for diag in &self.diags {
			diag.report_err(source);
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
			Severity::Note => write!(f, "note"),
		}
	}
}
