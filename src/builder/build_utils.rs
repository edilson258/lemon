use crate::{checker::types::TypeId, error_build};

use super::Builder;

impl Builder<'_> {
	pub fn is_need_heap_allocation(&self, value_type: TypeId) -> Option<usize> {
		if value_type.is_builtin_type() || self.type_store.is_borrow(value_type) {
			return None;
		}
		let value_name = match self.type_store.lookup_struct_name(value_type) {
			Some(name) => name,
			None => {
				let type_text = self.type_store.get_display_ir_type(value_type);
				error_build!("cannot find struct name for type {}", type_text).report(self.loader);
			}
		};
		match self.ctx.struct_sizes.get(value_name).copied() {
			Some(size) => Some(size),
			None => error_build!("cannot find size of {}", value_name).report(self.loader),
		}
	}
}
