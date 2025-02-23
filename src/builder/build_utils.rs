use crate::{checker::types::TypeId, report::throw_ir_build_error};

use super::Builder;

impl Builder<'_> {
	pub fn is_need_heap_allocation(&self, value_type: TypeId) -> Option<usize> {
		if value_type.is_known() || self.type_store.is_borrow(value_type) {
			return None;
		}
		let value_name = match self.type_store.get_struct_name(value_type) {
			Some(name) => name,
			None => {
				let type_text = self.type_store.get_display_ir_type(value_type);
				throw_ir_build_error(format!("cannot find struct name for type {}", type_text))
			}
		};
		match self.ctx.struct_table_size.get(value_name).copied() {
			Some(size) => Some(size),
			None => throw_ir_build_error(format!("cannot find size of {}", value_name)),
		}
	}
}
