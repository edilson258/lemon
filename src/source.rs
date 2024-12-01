use std::path::PathBuf;

pub struct Source {
  pub raw: String,
  pub pathbuf: PathBuf,
}

impl Source {
  pub fn new(raw: String, pathbuf: PathBuf) -> Self {
    Self { raw, pathbuf }
  }
  pub fn raw(&self) -> &str {
    &self.raw
  }

  pub fn path(&self) -> &PathBuf {
    &self.pathbuf
  }
}
