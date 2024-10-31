use code_highlighter::{highlight_error_with_context, highlight_warning_with_context};

use super::{range::Range, source::Source};

pub fn report_and_exit(message: &str, range: &Range, source: &Source) -> ! {
  report_error(message, range, &source, false);
  std::process::exit(1);
}

fn report_error(message: &str, range: &Range, source: &Source, warning: bool) {
  let context = 2; // number of lines of context
  println!("");

  if !warning {
    println!("{} {}", highlight_text_with_red("ERROR >>>"), highlight_text_with_white(message));
  } else {
    let warning = highlight_text_with_yellow("WARNING >>>");
    let message = format!("{} {}", warning, highlight_text_with_white(message));
    println!("{}", message);
  }
  let file_highlight = highlight_text_with_cyan(&source.name);
  println!("{}", file_highlight);
  println!("");
  if warning {
    let code_highliter = format!("{}", highlight_warning_with_context(range.start, range.end, &source.raw, context));
    println!("{}", code_highliter);
  } else {
    let code_highliter = format!("{}", highlight_error_with_context(range.start, range.end, &source.raw, context));
    println!("{}", code_highliter);
  }
  println!();
}

pub fn highlight_text_with_red(text: &str) -> String {
  format!("\x1b[31m{}\x1b[0m", text)
}

pub fn highlight_text_with_yellow(text: &str) -> String {
  format!("\x1b[33m{}\x1b[0m", text)
}

pub fn highlight_text_with_green(text: &str) -> String {
  format!("\x1b[32m{}\x1b[0m", text)
}

pub fn highlight_text_with_blue(text: &str) -> String {
  format!("\x1b[34m{}\x1b[0m", text)
}

pub fn highlight_text_with_magenta(text: &str) -> String {
  format!("\x1b[35m{}\x1b[0m", text)
}

pub fn highlight_text_with_cyan(text: &str) -> String {
  format!("\x1b[36m{}\x1b[0m", text)
}

pub fn highlight_text_with_white(text: &str) -> String {
  format!("\x1b[97m{}\x1b[0m", text)
}

pub fn highlight_text_with_gray(text: &str) -> String {
  format!("\x1b[90m{}\x1b[0m", text)
}
