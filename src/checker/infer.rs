use super::{Checker, TyResult};

use super::types::{Type, TypeId};

impl Checker<'_> {
	pub fn infer_type(&self, expected: TypeId, found: TypeId) -> TyResult<TypeId> {
		if found.is_known() || !expected.is_number() {
			return Ok(found);
		}
		let found_type = self.get_stored_type(found);
		let type_id = match found_type {
			Type::NumRange(num_range) => num_range.infer_with_type_id(expected).unwrap_or(found),
			_ => found,
		};
		Ok(type_id)
	}

	pub fn infer_no_type_anotation(&self, type_id: TypeId) -> TyResult<TypeId> {
		if type_id.is_known() {
			return Ok(type_id);
		}
		let type_value = self.get_stored_type(type_id);
		match type_value {
			Type::NumRange(range) => Ok(TypeId::from(&range.as_number())),
			_ => Ok(type_id),
		}
	}
}
