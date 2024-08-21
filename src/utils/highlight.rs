#![allow(dead_code)]

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
