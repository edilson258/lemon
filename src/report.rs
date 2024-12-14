#![allow(dead_code)]

use std::{fs, path::PathBuf};

use crate::diag::{Diag, Severity};
use dunh::{high_err_ctx, high_info_ctx, high_warn_ctx};

const CTX_LOC: usize = 1; // -- context lines

enum ReportKind {
  SyntaxErr,
  TypeErr,
  Err,
}

impl ReportKind {
  fn as_str(&self) -> &'static str {
    match self {
      ReportKind::SyntaxErr => "syntax error",
      ReportKind::TypeErr => "type error",
      ReportKind::Err => "error",
    }
  }
}

// ---
//

pub fn report_err(diag: &Diag, path: &PathBuf) {
  report(diag, path, ReportKind::Err)
}

pub fn report_type_err(diag: &Diag, path: &PathBuf) {
  report(diag, path, ReportKind::TypeErr)
}
pub fn report_wrap(diag: &Diag, path: &PathBuf) {
  report(diag, path, ReportKind::Err);
  std::process::exit(1);
}

pub fn report_syntax_err(diag: &Diag, path: &PathBuf) {
  report(diag, path, ReportKind::SyntaxErr);
  std::process::exit(1);
}

// -- utils --
//

fn report(diag: &Diag, path: &PathBuf, kind: ReportKind) {
  println!(); // -- new line

  let slug = match diag.severity {
    Severity::Err => text_red(kind.as_str()),
    Severity::Warn => text_yellow("warning"),
    Severity::Info => text_green("info"),
  };
  println!("{}: {}", slug, diag.message); // -- message
  println!("---> {}", text_gray(&path.display().to_string())); // -- filename

  let raw = fs::read_to_string(path).unwrap();
  let code = match diag.severity {
    Severity::Err => high_err_ctx(diag.range.start, diag.range.end, &raw, CTX_LOC),
    Severity::Warn => high_warn_ctx(diag.range.start, diag.range.end, &raw, CTX_LOC),
    Severity::Info => high_info_ctx(diag.range.start, diag.range.end, &raw, CTX_LOC),
  };
  println!("{}", code);
  // println!("note: run with `lemon --debug` for more info");
}

pub fn throw_error(text: impl Into<String>) -> ! {
  println!("{} {}", text_red("error: "), text_white(text.into().as_str()));
  // println!("{} {}", text_red("ERROR >>>"), text_white(text.into().as_str()));
  std::process::exit(1);
}

pub fn text_red(text: &str) -> String {
  format!("\x1b[31m{}\x1b[0m", text)
}

pub fn text_yellow(text: &str) -> String {
  format!("\x1b[33m{}\x1b[0m", text)
}

pub fn text_green(text: &str) -> String {
  format!("\x1b[32m{}\x1b[0m", text)
}

pub fn text_blue(text: &str) -> String {
  format!("\x1b[34m{}\x1b[0m", text)
}

pub fn text_magenta(text: &str) -> String {
  format!("\x1b[35m{}\x1b[0m", text)
}

pub fn text_cyan(text: &str) -> String {
  format!("\x1b[36m{}\x1b[0m", text)
}

pub fn text_white(text: &str) -> String {
  format!("\x1b[2m\x1b[37m{}\x1b[0m", text) // "Dim" + Branco
}

pub fn text_gray(text: &str) -> String {
  format!("\x1b[90m{}\x1b[0m", text)
}
