#![allow(dead_code)]

use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Source {
  pub raw: String,
  pub path: PathBuf,
}

impl Source {
  pub fn new(raw: &str, path: &str) -> Self {
    Self { raw: raw.to_string(), path: PathBuf::from(path) }
  }

  pub fn len(&self) -> usize {
    self.raw.len()
  }

  pub fn get_raw(&self) -> String {
    self.raw.clone()
  }

  pub fn get_filename(&self) -> String {
    self.path.display().to_string()
  }
}
