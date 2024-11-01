#![allow(dead_code)]

use crate::{diag::DiagGroup, utils::source::Source};

use std::path::PathBuf;

pub struct Loader {
  cwd: PathBuf,
  pub files: Vec<Source>,
  pub diags: DiagGroup,
}
