#![allow(dead_code)]
mod mod_id;
use std::path::{Path, PathBuf};

pub use mod_id::*;
use rustc_hash::FxHashMap;

use crate::{
	ast, error_resolve, file_system::FileSystem, message::MessageResult, shio::ShioConfig,
	source::Source,
};

pub struct Loader {
	shio: ShioConfig,
	file_system: FileSystem,
	root: FxHashMap<ModId, Source>,
	mods: FxHashMap<ModId, ast::Program>,
}

impl Loader {
	pub fn new(shio: ShioConfig, file_system: FileSystem) -> Self {
		Self { shio, file_system, root: FxHashMap::default(), mods: FxHashMap::default() }
	}

	pub fn load_entry(&mut self) -> MessageResult<ModId> {
		let pathname = self.shio.loader.main.display().to_string();
		let (raw, abs_path) = self.file_system.load_mod_entry(&pathname)?;
		let source = Source::new(raw, abs_path, pathname);
		Ok(self.register_source(source))
	}

	pub fn load_source(&mut self, path: &str, base_mod_id: ModId) -> MessageResult<ModId> {
		let mod_path = self.lookup_source_unchecked(base_mod_id).abs_path.clone();
		let (raw, abs_mod_path) = self.file_system.load_mod_from_base(mod_path, path)?;
		let path = self.resolve_path(path);
		let source = Source::new(raw, abs_mod_path, path.display().to_string());
		Ok(self.register_source(source))
	}
	fn register_source(&mut self, source: Source) -> ModId {
		let mod_id = ModId::new(self.root.len() as u64);
		self.root.insert(mod_id, source);
		mod_id
	}

	fn resolve_path(&self, path: &str) -> PathBuf {
		let given_path = Path::new(path);
		if given_path.extension().is_none() {
			return given_path.join("mod.ln");
		}
		given_path.to_path_buf()
	}

	pub fn lookup_source(&self, mod_id: ModId) -> Option<&Source> {
		self.root.get(&mod_id)
	}

	pub fn lookup_source_unchecked(&self, mod_id: ModId) -> &Source {
		self.lookup_source_result(mod_id).unwrap_or_else(|message| message.report(self))
	}

	pub fn lookup_source_result(&self, mod_id: ModId) -> MessageResult<&Source> {
		match self.root.get(&mod_id) {
			Some(source) => Ok(source),
			None => Err(error_resolve!("source for '{}' not found", mod_id)),
		}
	}

	pub fn add_mod(&mut self, mod_id: ModId, ast: ast::Program) {
		self.mods.insert(mod_id, ast);
	}

	pub fn get_mod(&self, mod_id: ModId) -> Option<&ast::Program> {
		self.mods.get(&mod_id)
	}

	pub fn lookup_mod_result(&mut self, mod_id: ModId) -> MessageResult<&mut ast::Program> {
		match self.mods.get_mut(&mod_id) {
			Some(ast) => Ok(ast),
			None => Err(error_resolve!("ast for '{}' not found", mod_id)),
		}
	}
	pub fn take_mod_result(&mut self, mod_id: ModId) -> MessageResult<ast::Program> {
		match self.mods.remove(&mod_id) {
			Some(ast) => Ok(ast),
			None => Err(error_resolve!("ast for '{}' not found", mod_id)),
		}
	}
}
