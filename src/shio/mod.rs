#![allow(dead_code, unused_variables)]
mod utils;

use rustc_hash::FxHashMap;
use std::{fs, path::PathBuf};
use toml::Value;
use utils::{get_toml_array_value, get_toml_bool_value};
use utils::{get_toml_integer_value, get_toml_text_value};
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
		let loader = toml.get("loader").ok_or("shio.toml must have a loader section")?;
		let main: PathBuf = get_toml_text_value(loader, "main")?.into();
		let cwd = main.parent().map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
		let strict = get_toml_bool_value(loader, "strict").unwrap_or(true);
		let max_threads = get_toml_integer_value(loader, "max_threads").unwrap_or(4);
		Ok(Self::new(main, cwd, strict, max_threads))
	}

	pub fn with_main(main: PathBuf) -> Self {
		let cwd = main.parent().map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
		Self::new(main, cwd, true, 4)
	}

	pub fn with_cwd(main: PathBuf, cwd: PathBuf) -> Self {
		Self::new(main, cwd, true, 4)
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
		let package = toml.get("package").ok_or("shio.toml must have a package section")?;
		#[rustfmt::skip]
		let name = get_toml_text_value(package, "name")
	    .unwrap_or("undefined".to_string());
		#[rustfmt::skip]
		let version = get_toml_text_value(package, "version")
		  .unwrap_or("0.1.0".to_string());
		#[rustfmt::skip]
		let description = get_toml_text_value(package, "description")
		  .unwrap_or_default();

		let authors = get_toml_array_value(package, "authors").unwrap_or_default();
		let license = get_toml_text_value(package, "license").unwrap_or_default();
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
		let mut local = FxHashMap::default();

		if let Some(dependencies) = toml.get("dependencies").and_then(Value::as_table) {
			for (name, path) in dependencies {
				if let Some(path_str) = path.as_str() {
					local.insert(name.clone(), PathBuf::from(path_str));
				}
			}
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

impl Default for ShioDependencies {
	fn default() -> Self {
		Self::new()
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

	fn get_shio_toml_path() -> PathBuf {
		// find in current directory ./shio.toml
		PathBuf::from("./shio.toml")
	}

	pub fn load_from_toml(path: Option<PathBuf>) -> Result<Self, String> {
		let path = path.unwrap_or(Self::get_shio_toml_path());
		#[rustfmt::skip]
		let shio_text = fs::read_to_string(path)
		  .map_err(|_| "failed to read shio.toml".to_string())?;

		#[rustfmt::skip]
		let shio_toml: Value  = toml::from_str(&shio_text)
			.map_err(|_| "failed to parse shio.toml".to_string())?;

		let package = ShioPackage::from_toml(&shio_toml)?;
		let loader = ShioLoader::from_toml(&shio_toml)?;
		let dependencies = ShioDependencies::from_toml(&shio_toml)?;
		Ok(Self::new(package, loader, dependencies))
	}

	pub fn with_defaults(main: PathBuf) -> Self {
		let package = ShioPackage::default();
		let loader = ShioLoader::with_main(main);
		let dependencies = ShioDependencies::new();
		Self::new(package, loader, dependencies)
	}
}
