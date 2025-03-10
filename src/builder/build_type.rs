use crate::{checker::types::TypeId, range::Range, report::throw_error_with_range};

use super::Builder;

impl Builder<'_> {
	pub fn build_type(&mut self, type_id: Option<TypeId>, range: Range) -> TypeId {
		if let Some(type_id) = type_id {
			return type_id;
		}
		let source = self.loader.get_current_source();
		throw_error_with_range("not found `type_id`", range, source);
	}
}
