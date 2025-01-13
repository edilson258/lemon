use std::{collections::HashMap, path::PathBuf};

use crate::{report::throw_error, source::Source};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FileId(u64);

impl FileId {
	pub fn new(id: u64) -> Self {
		Self(id)
	}
	pub fn as_usize(&self) -> usize {
		self.0 as usize
	}
}

pub struct Loader {
	sources: HashMap<FileId, Source>,
}

impl Loader {
	pub fn new() -> Self {
		Self { sources: HashMap::new() }
	}

	fn validate_extension(&self, ext: Option<&str>) {
		if let Some(ext) = ext {
			if ext != "ln" && ext != "lemon" {
				throw_error("unknown file extension, expected 'ln'")
			}
		} else {
			throw_error("extention not found, expected 'ln'")
		}
	}

	pub fn load(&mut self, path_str: &str) -> FileId {
		let path = PathBuf::from(path_str);
		let ext = path.extension().and_then(|ext| ext.to_str());
		self.validate_extension(ext);
		let file_id = self.add_file(path);
		self.load_file(file_id);
		file_id
	}

	pub fn load_file(&mut self, file_id: FileId) {
		let path = self.get_file(file_id);
		let raw = std::fs::read_to_string(path).unwrap_or_else(|err| match err.kind() {
			std::io::ErrorKind::NotFound => throw_error(format!("not found '{}'.", path.display())),
			_ => throw_error(format!("reading file `{}`, reason '{}'.", path.display(), err)),
		});
		let source = Source::new(raw, path.to_owned());
		self.sources.insert(file_id, source);
	}

	pub fn get_file(&self, file_id: FileId) -> &PathBuf {
		match self.sources.get(&file_id).map(|source| &source.pathbuf) {
			Some(path) => path,
			None => throw_error(format!("file not found file_{}", file_id.as_usize())),
		}
	}

	pub fn add_file(&mut self, path: PathBuf) -> FileId {
		let file_id = FileId::new(self.sources.len() as u64);
		self
			.sources
			.insert(file_id, Source::new(String::new(), path));
		file_id
	}

	pub fn get_source(&self, file_id: FileId) -> &Source {
		match self.sources.get(&file_id) {
			Some(source) => source,
			None => throw_error(format!("file not found file_{}", file_id.as_usize())),
		}
	}
}

impl Default for Loader {
	fn default() -> Self {
		Self::new()
	}
}
