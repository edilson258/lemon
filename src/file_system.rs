#![allow(dead_code, unused_variables)]
use rustc_hash::FxHashMap;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub struct FileSystem {
	pub shio_deps: PathBuf,                      //  ~/.shio/deps/
	pub custom_deps: FxHashMap<String, PathBuf>, // custom deps. e.g. math = { path = "../math"  }
	pub shio_cache: PathBuf,                     //  ~/.shio/cache/
	pub shio_remote: PathBuf,                    //  ~/.shio/remote/
	pub workspace: Option<PathBuf>,              // ./packages/math/mod.ln ... math is global
	pub cwd: PathBuf,                            // current working directory
	pub root: FxHashMap<PathBuf, String>,        // cache root : path -> raw string
}

impl FileSystem {
	pub fn new(cwd: PathBuf, workspace: Option<PathBuf>) -> Self {
		let root = dirs::home_dir().expect("failed to get home directory");
		let shio_root = root.join(".shio");
		let shio_deps = shio_root.join("deps");
		let shio_cache = shio_root.join("cache");
		let shio_remote = shio_root.join("remote");
		let root = FxHashMap::default();
		let custom_deps = FxHashMap::default();
		Self { shio_deps, shio_cache, shio_remote, workspace, cwd, root, custom_deps }
	}

	pub fn with_cwd(cwd: PathBuf) -> Self {
		Self::new(cwd, None)
	}

	pub fn with_workspace(cwd: PathBuf, workspace: PathBuf) -> Self {
		Self::new(cwd, Some(workspace))
	}

	// pub fn ensure_exists(&self, _path: &PathBuf) {
	// 	todo!("impl ensure exists directory");
	// }

	pub fn load_dependency(&mut self, pathname: &str) -> Result<(String, PathBuf), String> {
		let full_path = Self::resolve_module_path(&self.shio_deps, pathname);
		let raw = self.load_file(&full_path, pathname)?;
		Ok((raw, full_path))
	}

	pub fn load_cwd_dependency(&mut self, pathname: &str) -> Result<(String, PathBuf), String> {
		let full_path = Self::resolve_module_path(&self.cwd, pathname);
		let raw = self.load_file(&full_path, pathname)?;
		Ok((raw, full_path))
	}

	pub fn load_custom_dependency(&mut self, pathname: &str) -> Result<(String, PathBuf), String> {
		if let Some(base_path) = self.custom_deps.get(pathname) {
			let full_path = Self::resolve_module_path(base_path, pathname);
			let raw = self.load_file(&full_path, pathname)?;
			return Ok((raw, full_path));
		}
		Err(format!("file '{}' not found", pathname))
	}

	pub fn load_remote(&mut self, pathname: &str) -> Result<(String, PathBuf), String> {
		let full_path = Self::resolve_module_path(&self.shio_remote, pathname);
		let raw = self.load_file(&full_path, pathname)?;
		Ok((raw, full_path))
	}

	pub fn load_workspace(&mut self, pathname: &str) -> Result<(String, PathBuf), String> {
		if let Some(workspace) = &self.workspace {
			let full_path = Self::resolve_module_path(workspace, pathname);
			let raw = self.load_file(&full_path, pathname)?;
			return Ok((raw, full_path));
		}
		Err(format!("file '{}' not found", pathname))
	}

	fn load_file(&mut self, path: &Path, pathname: &str) -> Result<String, String> {
		if let Some(cache) = self.root.get(path) {
			return Ok(cache.clone());
		}
		let result = Self::_load_file(path, pathname);
		if let Ok(contents) = result.as_ref() {
			self.root.insert(path.to_path_buf(), contents.clone());
		}
		result
	}

	fn _load_file(path: &Path, pathname: &str) -> Result<String, String> {
		fs::read_to_string(path).map_err(|err| match err.kind() {
			io::ErrorKind::NotFound => format!("file '{}' not found", pathname),
			io::ErrorKind::PermissionDenied => format!("permission denied reading '{}'", pathname),
			io::ErrorKind::IsADirectory => format!("expected file, found directory '{}'", pathname),
			io::ErrorKind::InvalidData => {
				format!("expected utf8 data, unsupported encoding '{}'", pathname)
			}
			_ => format!("failed to read '{}', unexpected error", pathname),
		})
	}

	#[inline(always)]
	fn resolve_module_path(base_dir: &Path, path_name: &str) -> PathBuf {
		// todo: resolve only if ext is ln?
		let root = base_dir.join(path_name);
		if root.is_file() {
			return root;
		}
		root.join("mod.ln")
	}
}
