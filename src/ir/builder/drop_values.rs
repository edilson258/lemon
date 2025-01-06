use crate::{checker::types::TypeId, ir::ir};

use super::Builder;

impl Builder<'_> {
	pub fn drop_values(&mut self) {
		let free_values = self.ctx.get_free_fn_scope();
		for register in free_values.iter() {
			let free_instr = ir::Instr::Free(*register);
			self.add_instr(free_instr);
		}
	}

	#[inline(always)]
	#[allow(unused_variables)]
	pub fn can_free_value(&mut self, register: ir::Register, type_id: TypeId) {
		// let value_type = match self.type_store.get_type(type_id) {
		// 	Some(value_type) => value_type,
		// 	None => return, // todo: error ?
		// };

		// if !value_type.can_free_value() {}
		// self.ctx.add_free_fn_scope(register);
	}
}
