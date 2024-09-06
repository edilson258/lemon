use crate::utils::{
  highlight::{
    highlight_text_with_cyan, highlight_text_with_red, highlight_text_with_white, highlight_text_with_yellow,
  },
  range::Range,
  source::Source,
};
use code_highlighter::{highlight_error_with_context, highlight_warning_with_context};

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
