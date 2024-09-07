use crate::{
  diagnostics::diag::Severity,
  utils::{
    highlight::{
      highlight_text_with_cyan, highlight_text_with_green, highlight_text_with_red, highlight_text_with_white,
      highlight_text_with_yellow,
    },
    range::Range,
    source::Source,
  },
};
use code_highlighter::{highlight_error_with_context, highlight_warning_with_context};

use super::diag::Diag;

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

pub fn report_and_exit(message: &str, range: &Range, source: &Source) -> ! {
  report_error(message, range, &source, false);
  std::process::exit(1);
}

pub fn report_diag_and_exit(diag: &Diag, source: &Source) -> ! {
  report_diag(diag, source);
  std::process::exit(1);
}

pub fn report_diag(diag: &Diag, source: &Source) {
  let message = highlight_text_with_white(&diag.message);
  let hint = if let Some(hint) = &diag.hint { Some(highlight_text_with_white(hint)) } else { None };
  let range = diag.range;

  let context = 2; // number of lines of context
  println!(); // new line

  // message
  match diag.severity {
    Severity::Error => {
      let error = highlight_text_with_red("ERROR >>>");
      println!("{} {}", error, message);
    }
    Severity::Warning => {
      let warning = highlight_text_with_yellow("WARNING >>>");
      let message = format!("{} {}", warning, message);
      println!("{}", message);
    }
    Severity::Info => {
      let info = highlight_text_with_green("INFO >>>");
      let message = format!("{} {}", info, message);
      println!("{}", message);
    }
  };

  // file name
  let file_highlight = highlight_text_with_cyan(&source.name);
  println!("{}", file_highlight);
  println!("");
  // code highlighter
  match diag.severity {
    Severity::Error => {
      let code_highliter = format!("{}", highlight_error_with_context(range.start, range.end, &source.raw, context));
      println!("{}", code_highliter);
    }
    Severity::Warning => {
      let code_highliter = format!("{}", highlight_warning_with_context(range.start, range.end, &source.raw, context));
      println!("{}", code_highliter);
    }
    Severity::Info => {
      // let code_highliter = format!("{}", highlight_info_with_context(range.start, range.end, &source.raw, context));
      // println!("{}", code_highliter);
      todo!()
    }
  }

  // hint
  if let Some(hint) = &hint {
    let hint = format!("{} {}", highlight_text_with_green("HINT >>>"), hint);
    println!("{}", hint);
  }
  println!(); // new line
}
