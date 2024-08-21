use crate::utils::{
  highlight::{
    highlight_text_with_cyan, highlight_text_with_red, highlight_text_with_white, highlight_text_with_yellow,
  },
  range::Range,
};
use code_highlighter::{highlight_error, highlight_warning};

fn report_error(message: &str, range: &Range, raw: &str, file_name: &str, warning: bool) {
  println!("");
  if !warning {
    println!("{} {}", highlight_text_with_red("ERROR >>>"), highlight_text_with_white(message));
  } else {
    let warning = highlight_text_with_yellow("WARNING >>>");
    let message = format!("{} {}", warning, highlight_text_with_white(message));

    println!("{}", message);
  }
  let file_highlight = highlight_text_with_cyan(&file_name);
  println!("{}", file_highlight);
  println!("");
  if warning {
    let code_highliter = format!("{}", highlight_warning(range.start, range.end, raw));
    println!("{}", code_highliter);
  } else {
    let code_highliter = format!("{}", highlight_error(range.start, range.end, raw));
    println!("{}", code_highliter);
  }
  println!();
}

pub fn report_and_exit(message: &str, range: &Range, raw: &str, file_name: &str) -> ! {
  report_error(message, range, raw, file_name, false);
  std::process::exit(1);
}
