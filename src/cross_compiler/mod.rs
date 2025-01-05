#![allow(dead_code)]
use std::path::Path;

use inkwell::module::Module;
use inkwell::targets::{
	CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine, TargetTriple,
};
use inkwell::OptimizationLevel;

use crate::report::throw_cross_compile_error;

#[derive(Debug)]
pub struct CrossCompiler {
	triple: TargetTriple, // target
	cpu: String,          // cpu (eg.: "generic")
	features: String,     // feat target (eg.: SIMD)
	optimization: OptimizationLevel,
	reloc_mode: RelocMode, // (eg.: Default, PIC)
	code_model: CodeModel, //  (eg.: Default, Small)
}

impl CrossCompiler {
	pub fn new(triple: &str) -> Self {
		Self {
			triple: TargetTriple::create(triple),
			cpu: "generic".into(),
			features: "".into(),
			optimization: OptimizationLevel::Aggressive,
			reloc_mode: RelocMode::Default,
			code_model: CodeModel::Default,
		}
	}

	pub fn set_cpu(&mut self, cpu: &str) -> &mut Self {
		self.cpu = cpu.into();
		self
	}

	pub fn set_features(&mut self, features: &str) -> &mut Self {
		self.features = features.into();
		self
	}
	pub fn set_optimize(&mut self, level: OptimizationLevel) -> &mut Self {
		self.optimization = level;
		self
	}

	pub fn target_machine(&self) -> TargetMachine {
		Target::initialize_all(&InitializationConfig::default());
		let target_string = match Target::from_triple(&self.triple).map_err(|e| e.to_string()) {
			Ok(target) => target,
			Err(err) => throw_cross_compile_error(err),
		};

		let target = target_string.create_target_machine(
			&self.triple,
			&self.cpu,
			&self.features,
			self.optimization,
			self.reloc_mode,
			self.code_model,
		);

		if let Some(target) = target {
			return target;
		}
		throw_cross_compile_error("failed to create target machine");
	}
	pub fn create_object(&self, m: &Module, file_type: FileType, path: &Path) {
		let target = self.target_machine();
		match target.write_to_file(m, file_type, path) {
			Ok(_) => {}
			Err(llvm_msg) => throw_cross_compile_error(llvm_msg.to_string()),
		}
	}
}
