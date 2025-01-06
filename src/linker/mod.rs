#![allow(dead_code)]
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::report::throw_linker_error;
use crate::utils::{get_current_user_machine, Machine};

pub enum Choice {
	Mold,
	Llvm,
	Clang,
}
#[derive(Debug, Clone)]
pub struct Linker {
	output: Option<PathBuf>,
	path: String,
}

impl Linker {
	pub fn new(choice: Choice) -> Self {
		let os = get_current_user_machine();
		let binary = match (choice, os) {
			(Choice::Mold, Machine::Linux) => "mold",
			(Choice::Llvm, Machine::Linux) => "ld.lld",
			(Choice::Mold, Machine::Mac) => "ld",
			(Choice::Llvm, Machine::Mac) => "ld.lld",
			(Choice::Mold, Machine::Win) => "lld-link",
			(Choice::Llvm, Machine::Win) => "lld-link",
			(Choice::Mold, Machine::Unknown) => "mold",
			(Choice::Llvm, Machine::Unknown) => "ld.lld",
			(Choice::Clang, _) => "clang",
		};

		let path = match which::which(binary).map(|p| p.to_string_lossy().to_string()) {
			Ok(path) => path,
			Err(_) => throw_linker_error(format!("'{}' not found", binary)),
		};

		Self { path, output: None }
	}

	pub fn add_output(&mut self, output: &Path) -> &mut Self {
		self.output = Some(output.to_path_buf());
		self
	}

	pub fn link(&self) -> Output {
		if self.output.is_none() {
			throw_linker_error("not output file");
		}
		let output_path = self.output.as_ref().unwrap();
		let mut command = Command::new(&self.path);
		let path = self.resolve_output_path(output_path);
		command.arg(output_path.to_str().unwrap()).arg("-o").arg(path.to_str().unwrap());
		match get_current_user_machine() {
			Machine::Linux => {
				// command.args(["-lc", "-lm"]); // (libc e libm)
			}
			Machine::Mac => {
				if !output_path.to_str().unwrap().contains("clang") {
					// command.args(["-ar ch", "arm64"]);
				}
			}
			Machine::Win => {
				// command.args(["/defaultlib:libcmt"]);
			}
			Machine::Unknown => {}
		};

		let output = match command.output() {
			Ok(output) => output,
			Err(err) => throw_linker_error(format!("failed to link: {}", err)),
		};

		if !output.status.success() {
			throw_linker_error(format!("{}", String::from_utf8_lossy(&output.stderr)));
		}
		std::fs::remove_file(output_path).unwrap_or_else(|err| throw_linker_error(format!("{}", err)));
		output
	}
	#[inline(always)]
	fn resolve_output_path(&self, output: &Path) -> PathBuf {
		match get_current_user_machine() {
			Machine::Win => output.with_extension("exe"),
			_ => output.with_extension(""),
		}
	}
}

fn format_command(command: &Command) -> String {
	let program = command.get_program().to_string_lossy();
	let args: Vec<String> = command.get_args().map(|arg| arg.to_string_lossy().to_string()).collect();
	format!("{} {}", program, args.join(" "))
}
