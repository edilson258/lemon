#![allow(dead_code)]

use crate::{
	diag::{Diag, Severity},
	source::Source,
};

enum ReportKind {
	SyntaxErr,
	TypeErr,
	Err,
}

impl std::fmt::Display for ReportKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ReportKind::SyntaxErr => write!(f, "syntax error"),
			ReportKind::TypeErr => write!(f, "type error"),
			ReportKind::Err => write!(f, "error"),
		}
	}
}

pub fn report_err(diag: &Diag, source: &Source) {
	report(diag, ReportKind::Err, source)
}

pub fn report_type_err(diag: &Diag, source: &Source) {
	report(diag, ReportKind::TypeErr, source)
}
pub fn report_wrap(diag: &Diag, source: &Source) {
	report(diag, ReportKind::Err, source);
	std::process::exit(1);
}

pub fn report_syntax_err(diag: &Diag, source: &Source) {
	report(diag, ReportKind::SyntaxErr, source);
	std::process::exit(1);
}

fn report(diag: &Diag, kind: ReportKind, source: &Source) {
	let slug = match diag.severity {
		Severity::Err => text_red(kind.to_string().as_str()),
		Severity::Warn => text_yellow("warning"),
		Severity::Note => text_green("info"),
	};
	println!("{}: {}", slug, diag.message); // -- message
	println!("---> {}", text_gray(source.path_str().as_str())); // -- filename
	let start = diag.range.start;
	let end = diag.range.end;
	let code = match diag.severity {
		Severity::Err => codelighter::highlight_error(start, end, source.raw()),
		Severity::Warn => codelighter::highlight_warn(start, end, source.raw()),
		Severity::Note => codelighter::highlight_note(start, end, source.raw()),
	};
	println!("{}", code);
	if let Some(note) = &diag.note {
		println!("== {} {}", text_cyan("note:"), note);
	}
}

pub fn throw_error(text: impl Into<String>) -> ! {
	println!("{} {}", text_red("error: "), text_white(text.into().as_str()));
	std::process::exit(1);
}

fn text_red(text: &str) -> String {
	format!("\x1b[31m{}\x1b[0m", text)
}

fn text_yellow(text: &str) -> String {
	format!("\x1b[33m{}\x1b[0m", text)
}

fn text_green(text: &str) -> String {
	format!("\x1b[32m{}\x1b[0m", text)
}

fn text_blue(text: &str) -> String {
	format!("\x1b[34m{}\x1b[0m", text)
}

fn text_magenta(text: &str) -> String {
	format!("\x1b[35m{}\x1b[0m", text)
}

fn text_cyan(text: &str) -> String {
	format!("\x1b[36m{}\x1b[0m", text)
}

fn text_white(text: &str) -> String {
	format!("\x1b[2m\x1b[37m{}\x1b[0m", text)
}

fn text_gray(text: &str) -> String {
	format!("\x1b[90m{}\x1b[0m", text)
}
