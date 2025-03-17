#![allow(dead_code)]
mod mod_id;
pub use mod_id::*;
use rustc_hash::FxHashMap;

use crate::ast;
use crate::file_system::FileSystem;
use crate::report::throw_error;
use crate::shio::ShioConfig;
use crate::source::Source;

pub struct Loader {
	shio: ShioConfig,
	file_system: FileSystem,
	root: FxHashMap<ModId, Source>,
	mods: FxHashMap<ModId, ast::Program>,
}

impl Loader {
	pub fn new(shio: ShioConfig, file_system: FileSystem) -> Self {
		let root = FxHashMap::default();
		let mods = FxHashMap::default();
		Self { shio, file_system, root, mods }
	}

	pub fn load_entry(&mut self) -> Result<ModId, String> {
		let entry_file = self.shio.loader.main.display().to_string();
		self.load_source(&entry_file)
	}

	pub fn get_source(&self, mod_id: ModId) -> Option<&Source> {
		self.root.get(&mod_id)
	}

	pub fn get_source_unchecked(&self, mod_id: ModId) -> &Source {
		self.get_source_result(mod_id).unwrap_or_else(|err| throw_error(err))
	}

	pub fn get_source_result(&self, mod_id: ModId) -> Result<&Source, String> {
		match self.root.get(&mod_id) {
			Some(source) => Ok(source),
			None => Err(format!("'{}' not found", mod_id)),
		}
	}

	pub fn add_mod(&mut self, mod_id: ModId, ast: ast::Program) {
		self.mods.insert(mod_id, ast);
	}
	pub fn get_mod(&self, mod_id: ModId) -> Option<&ast::Program> {
		self.mods.get(&mod_id)
	}

	pub fn get_mod_result(&mut self, mod_id: ModId) -> Result<&mut ast::Program, String> {
		match self.mods.get_mut(&mod_id) {
			Some(ast) => Ok(ast),
			None => Err(format!("'{}' not found", mod_id)),
		}
	}

	pub fn load_source(&mut self, path: &str) -> Result<ModId, String> {
		let (raw, abs_path) = self.file_system.load_dependency(path)?;
		let source = Source::new(raw, abs_path, path.into());
		Ok(self.register_source(source))
	}

	fn register_source(&mut self, source: Source) -> ModId {
		let mod_id = ModId::new(self.root.len() as u64);
		self.root.insert(mod_id, source);
		mod_id
	}
}
