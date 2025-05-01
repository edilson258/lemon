use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct Source {
	pub raw: String,
	pub abs_path: PathBuf,
	pub pathname: String,
}

impl Source {
	pub fn new(raw: String, abs_path: PathBuf, pathname: String) -> Self {
		Self { raw, abs_path, pathname }
	}
}
