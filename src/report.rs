#![allow(dead_code)]

use std::fs;

use crate::diag::{Diag, Severity};
use dunh::{high_err_ctx, high_info_ctx, high_warn_ctx};

const LONES_CTX: usize = 2;

// ---
//

pub fn report_err(diag: &Diag) {
  report(diag)
}

pub fn report_warn(diag: &Diag) {
  report(diag)
}

pub fn report_info(diag: &Diag) {
  report(diag)
}

pub fn report_wrap(diag: &Diag) -> ! {
  report(diag);
  std::process::exit(1);
}

// -- utils --
//

fn report(diag: &Diag) {
  println!(""); // -- new line

  let slug = match diag.severity {
    Severity::Err => text_red("ERROR >>>"),
    Severity::Warn => text_yellow("WARNING >>>"),
    Severity::Info => text_gray("INFO >>>"),
  };
  println!("{} {}", slug, text_white(&diag.message)); // -- message

  println!("{}", text_gray(&diag.path.display().to_string())); // -- filename

  let raw = fs::read_to_string(&diag.path).unwrap();
  let code = match diag.severity {
    Severity::Err => high_err_ctx(diag.range.start, diag.range.end, &raw, LONES_CTX),
    Severity::Warn => high_warn_ctx(diag.range.start, diag.range.end, &raw, LONES_CTX),
    Severity::Info => high_info_ctx(diag.range.start, diag.range.end, &raw, LONES_CTX),
  };

  println!("{}", code);
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
  format!("\x1b[97m{}\x1b[0m", text)
}

pub fn text_gray(text: &str) -> String {
  format!("\x1b[90m{}\x1b[0m", text)
}
