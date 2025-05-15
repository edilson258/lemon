use crate::{error_codegen, ir};

use super::Llvm;

impl Llvm<'_> {
	pub fn llvm_compile_load(&mut self, binary: &ir::UnInstr) {
		let value = self.llvm_compile_value(&binary.src);
		if value.is_pointer_value() {
			let ptr = value.into_pointer_value();
			let basic_type = self.compile_type_to_basic_type(binary.dest.type_id);
			let value = self.load(basic_type, ptr, binary.dest.value.as_str());
			self.env.set_value(binary.dest.value.as_str(), value);
			return;
		}
		error_codegen!("cannot load from non-pointer '{}'", binary.src).report(self.loader);
	}
}
