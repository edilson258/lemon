#![allow(dead_code)]

use core::fmt;

use crate::{
	loader::{FileId, Loader},
	range::Range,
	report::{self},
	source::Source,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Severity {
	Err,
	Warn,
	Note,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Diag {
	pub file_id: FileId,
	pub severity: Severity,
	pub message: String,
	pub note: Option<String>,
	pub range: Range,
}

impl Diag {
	pub fn new(severity: Severity, message: String, range: Range) -> Self {
		let file_id = FileId::new(u64::MAX);
		Self { file_id, severity, message, note: None, range }
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

	pub fn with_file_id(mut self, file_id: FileId) -> Self {
		self.file_id = file_id;
		self
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

	pub fn report_engine_err_wrap(&self) -> ! {
		report::report_engine_err(self);
		std::process::exit(1);
	}
}

pub struct DiagGroup<'ckr> {
	pub diags: Vec<Diag>,
	pub loader: &'ckr Loader,
}

impl<'ckr> DiagGroup<'ckr> {
	pub fn new(loader: &'ckr Loader) -> Self {
		Self { diags: Vec::new(), loader }
	}
	pub fn add(&mut self, diag: Diag) {
		self.diags.push(diag);
	}

	pub fn report(&self) {
		for diag in &self.diags {
			let source = self.loader.get_source(diag.file_id);
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
