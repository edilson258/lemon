#![allow(dead_code, unused_variables)]
mod utils;

use rustc_hash::FxHashMap;
use std::{fs, path::PathBuf};
use toml::Value;
use utils::{
	get_toml_array_value, get_toml_bool_value, get_toml_integer_value, get_toml_text_value,
};

const DEFAULT_MAX_THREADS: usize = 4;

pub struct ShioLoader {
	pub main: PathBuf,
	pub cwd: PathBuf,
	pub strict: bool,
	pub max_threads: usize,
}

impl ShioLoader {
	pub fn new(main: PathBuf, cwd: PathBuf, strict: bool, max_threads: usize) -> Self {
		Self { main, cwd, strict, max_threads }
	}

	pub fn from_toml(toml: &Value) -> Result<Self, String> {
		let loader = toml.get("loader").expect("shio.toml must have a loader section");
		let main = get_toml_text_value(loader, "main")?;
		let cwd = get_toml_text_value(loader, "cwd")?;
		let strict = get_toml_bool_value(loader, "strict").unwrap_or(true);
		#[rustfmt::skip]
		let max_threads = get_toml_integer_value(loader, "max_threads").unwrap_or(DEFAULT_MAX_THREADS);
		let main = PathBuf::from(main);
		let cwd = PathBuf::from(cwd);
		Ok(Self::new(main, cwd, strict, max_threads))
	}

	pub fn with_main(main: PathBuf) -> Self {
		let cwd = match main.parent() {
			Some(parent) => parent.to_path_buf(),
			None => PathBuf::from("."),
		};
		Self::new(main, cwd, true, DEFAULT_MAX_THREADS)
	}
	pub fn with_cwd(main: PathBuf, cwd: PathBuf) -> Self {
		// todo: check if main is the same as cwd
		// e.g. main  = path/to/main.rs
		//      cwd   = path/to
		//
		Self::new(main, cwd, true, DEFAULT_MAX_THREADS)
	}
}

pub struct ShioPackage {
	pub name: String,
	pub version: String,
	pub description: String,
	pub authors: Vec<String>,
	pub license: String,
}

impl ShioPackage {
	pub fn new(
		name: String,
		version: String,
		description: String,
		authors: Vec<String>,
		license: String,
	) -> Self {
		Self { name, version, description, authors, license }
	}

	pub fn from_toml(toml: &Value) -> Result<Self, String> {
		let package = toml.get("package").expect("shio.toml must have a package section");
		let name = get_toml_text_value(package, "name")?;
		let version = get_toml_text_value(package, "version")?;
		let description = get_toml_text_value(package, "description")?;
		let authors = get_toml_array_value(package, "authors")?;
		let license = get_toml_text_value(package, "license")?;
		Ok(Self { name, version, description, authors, license })
	}
}

impl Default for ShioPackage {
	fn default() -> Self {
		let name = "undefined".to_string();
		let version = "0.1.0".to_string();
		let description = "undefined".to_string();
		let authors = vec![];
		let license = "undefined".to_string();
		Self { name, version, description, authors, license }
	}
}

pub struct ShioDependencies {
	local: FxHashMap<String, PathBuf>,
}

impl ShioDependencies {
	pub fn new() -> Self {
		Self { local: FxHashMap::default() }
	}

	pub fn from_toml(toml: &Value) -> Result<Self, String> {
		let dependencies = match toml.get("dependencies") {
			Some(dependencies) => dependencies,
			None => return Ok(Self::new()),
		};
		let mut local = FxHashMap::default();
		for (name, path) in dependencies.as_table().unwrap() {
			let path = PathBuf::from(path.as_str().unwrap());
			local.insert(name.to_string(), path);
		}
		Ok(Self { local })
	}

	pub fn add_local(&mut self, name: String, path: PathBuf) {
		self.local.insert(name, path);
	}

	pub fn get_local(&self, name: &str) -> Option<&PathBuf> {
		self.local.get(name)
	}

	pub fn exists_local(&self, name: &str) -> bool {
		self.local.contains_key(name)
	}
}

pub struct ShioConfig {
	pub package: ShioPackage,
	pub loader: ShioLoader,
	pub dependencies: ShioDependencies,
}

impl ShioConfig {
	pub fn new(package: ShioPackage, loader: ShioLoader, dependencies: ShioDependencies) -> Self {
		Self { package, loader, dependencies }
	}
	pub fn load_from_toml(path: &PathBuf) -> Result<Self, String> {
		let shio_text = fs::read_to_string(path).expect("failed to read shio.toml");
		let shio_toml: Value = toml::from_str(&shio_text).expect("failed to parse shio.toml");
		let package = ShioPackage::from_toml(&shio_toml)?;
		let loader = ShioLoader::from_toml(&shio_toml)?;
		let dependencies = ShioDependencies::from_toml(&shio_toml)?;
		Ok(Self::new(package, loader, dependencies))
	}
}
