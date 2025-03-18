#![allow(dead_code, unused_variables)]

use rustc_hash::FxHashMap;
use std::{
	env, fs, io,
	path::{Path, PathBuf},
};

/// Represents the file system context for loading mods and dependencies.
pub struct FileSystem {
	/// directory where dependency mods are stored (e.g., ~/.shio/deps/)
	pub dependency_dir: PathBuf,
	/// directory used for caching (e.g., ~/.shio/cache/)
	pub cache_dir: PathBuf,
	/// directory where remote mods are stored (e.g., ~/.shio/remote/)
	pub remote_dir: PathBuf,
	/// optional workspace directory (e.g., ./packages/math)
	pub workspace_dir: Option<PathBuf>,
	/// current working directory
	pub current_dir: PathBuf,
	/// In-memory cache: maps file paths to their contents
	pub in_memory_cache: FxHashMap<PathBuf, String>,
	/// Custom dependency mapping (e.g., "math" → "../math")
	pub custom_dependencies: FxHashMap<String, PathBuf>,
}

impl FileSystem {
	pub fn new(current_dir: PathBuf, workspace_dir: Option<PathBuf>) -> Self {
		let home = dirs::home_dir().expect("failed to get home directory");
		let shio_root = home.join(".shio");
		let dependency_dir = shio_root.join("deps");
		let cache_dir = shio_root.join("cache");
		let remote_dir = shio_root.join("remote");
		let current_dir = Self::abs_and_canonical(current_dir.to_str().unwrap());
		let in_memory_cache = FxHashMap::default();
		let custom_dependencies = FxHashMap::default();
		Self {
			dependency_dir,
			cache_dir,
			remote_dir,
			workspace_dir,
			current_dir,
			in_memory_cache,
			custom_dependencies,
		}
	}

	/// constructs a FileSystem using only the current directory.
	pub fn from_current_dir(current_dir: PathBuf) -> Self {
		Self::new(current_dir, None)
	}

	/// constructs a FileSystem using both a current directory and a workspace.
	pub fn from_workspace(current_dir: PathBuf, workspace_dir: PathBuf) -> Self {
		Self::new(current_dir, Some(workspace_dir))
	}

	/// loads a mod entry from a given path.
	pub fn load_mod_entry(&mut self, path: &str) -> Result<(String, PathBuf), String> {
		let abs = Self::abs_and_canonical(path);
		self.load_and_cache_file(&abs, path).map(|contents| (contents, abs))
	}

	/// loads a dependency from the dependency directory.
	pub fn load_dependency(&mut self, path: &str) -> Result<(String, PathBuf), String> {
		let abs = Self::resolve_mod_path(&self.dependency_dir, path);
		self.load_and_cache_file(&abs, path).map(|contents| (contents, abs))
	}

	/// loads a dependency path to the current directory.
	pub fn load_dependency_from_cwd(&mut self, path: &str) -> Result<(String, PathBuf), String> {
		let abs = Self::resolve_mod_path(&self.current_dir, path);
		self.load_and_cache_file(&abs, path).map(|contents| (contents, abs))
	}

	/// loads a mod using a provided base directory.
	pub fn load_mod_from_base(
		&mut self,
		base: PathBuf,
		path: &str,
	) -> Result<(String, PathBuf), String> {
		let abs = Self::resolve_mod_path(&base, path);
		self.load_and_cache_file(&abs, path).map(|contents| (contents, abs))
	}

	/// Loads a custom dependency using the custom dependency mapping.
	pub fn load_custom_dependency(&mut self, key: &str) -> Result<(String, PathBuf), String> {
		if let Some(dep_base) = self.custom_dependencies.get(key) {
			let abs = Self::resolve_mod_path(dep_base, key);
			return self.load_and_cache_file(&abs, key).map(|contents| (contents, abs));
		}
		Err(format!("Custom dependency '{}' not found", key))
	}

	/// loads a remote dependency from the remote directory.
	pub fn load_remote_mod(&mut self, relative: &str) -> Result<(String, PathBuf), String> {
		let abs = Self::resolve_mod_path(&self.remote_dir, relative);
		self.load_and_cache_file(&abs, relative).map(|contents| (contents, abs))
	}

	/// loads a mod from the workspace directory.
	pub fn load_workspace_mod(&mut self, relative: &str) -> Result<(String, PathBuf), String> {
		if let Some(workspace) = &self.workspace_dir {
			let abs = Self::resolve_mod_path(workspace, relative);
			return self.load_and_cache_file(&abs, relative).map(|contents| (contents, abs));
		}
		Err(format!("Workspace file '{}' not found", relative))
	}

	/// reads a file from disk and caches its contents.
	fn load_and_cache_file(&mut self, abs: &Path, relative: &str) -> Result<String, String> {
		if let Some(contents) = self.in_memory_cache.get(abs) {
			return Ok(contents.clone());
		}
		let contents = Self::read_file(abs, relative)?;
		self.in_memory_cache.insert(abs.to_path_buf(), contents.clone());
		Ok(contents)
	}

	fn read_file(file_path: &Path, relative: &str) -> Result<String, String> {
		fs::read_to_string(file_path).map_err(|err| match err.kind() {
			io::ErrorKind::NotFound => format!("File '{}' not found", relative),
			io::ErrorKind::PermissionDenied => format!("Permission denied reading '{}'", relative),
			io::ErrorKind::IsADirectory => {
				format!("Expected a file, found directory '{}'", relative)
			}
			io::ErrorKind::InvalidData => format!("Invalid data encoding in '{}'", relative),
			_ => format!("Failed to read '{}', unexpected error", relative),
		})
	}

	/// resolves a mod path given a base directory and a mod path string.
	/// if the resolved path is a directory, "mod.ln" is appended.
	#[inline(always)]
	fn resolve_mod_path(base_dir: &Path, mod_path: &str) -> PathBuf {
		let canonical_mod = Self::to_canonical(PathBuf::from(mod_path));
		let joined = if base_dir.is_file() {
			base_dir.parent().unwrap().join(mod_path)
		} else {
			base_dir.join(canonical_mod)
		};
		if joined.is_dir() {
			joined.join("mod.ln")
		} else {
			joined
		}
	}

	/// converts a path string to an absolute, canonical path.
	fn abs_and_canonical<P: AsRef<str>>(relative: P) -> PathBuf {
		let relative = relative.as_ref();
		let absolute = if Path::new(relative).is_absolute() {
			PathBuf::from(relative)
		} else {
			env::current_dir().unwrap().join(relative)
		};
		Self::to_canonical(absolute)
	}

	/// attempts to canonicalize a path; if it fails, returns the original path.
	fn to_canonical(path: PathBuf) -> PathBuf {
		fs::canonicalize(&path).unwrap_or(path)
	}
}
