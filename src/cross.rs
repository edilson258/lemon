#![allow(dead_code)]
use std::path::Path;

use inkwell::module::Module;
use inkwell::targets::{
	CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
};
use inkwell::OptimizationLevel;

use crate::report::throw_cross_compile_error;

#[derive(Debug)]
pub struct Cross {
	triple: TargetTriple,
	cpu: String,
	feats: String,
	opt: OptimizationLevel,
	reloc: RelocMode,
	model: CodeModel,
}

impl Cross {
	pub fn new(triple: &str) -> Self {
		Self {
			triple: TargetTriple::create(triple),
			cpu: "generic".to_string(),
			feats: String::new(),
			opt: OptimizationLevel::default(),
			reloc: RelocMode::Default,
			model: CodeModel::Default,
		}
	}

	pub fn cpu(&mut self, cpu: &str) -> &mut Self {
		self.cpu = cpu.to_string();
		self
	}

	pub fn feats(&mut self, feats: &str) -> &mut Self {
		self.feats = feats.to_string();
		self
	}

	pub fn opt(&mut self, level: OptimizationLevel) -> &mut Self {
		self.opt = level;
		self
	}

	pub fn machine(&self) -> Result<TargetMachine, String> {
		Target::initialize_all(&InitializationConfig::default());
		let t = Target::from_triple(&self.triple).map_err(|e| e.to_string())?;

		let result = t.create_target_machine(
			&self.triple,
			&self.cpu,
			&self.feats,
			self.opt,
			self.reloc,
			self.model,
		);

		if let Some(target) = result {
			return Ok(target);
		}
		throw_cross_compile_error("failed to create target machine");
	}

	pub fn emit(&self, module: &Module, ty: FileType, path: &Path) -> Result<(), String> {
		let machine = self.machine()?;
		machine.write_to_file(module, ty, path).map_err(|e| e.to_string())
	}
}
