use super::types::{Type, TypeId};
use super::{Checker, TyResult};

impl Checker<'_> {
	pub fn infer_type(&self, expected: TypeId, found: TypeId) -> TyResult<TypeId> {
		if found.is_known() || !expected.is_number() {
			return Ok(found);
		}
		Ok(match self.get_stored_type(found) {
			Type::NumRange(num_range) => num_range.infer_with_type_id(expected).unwrap_or(found),
			_ => found,
		})
	}

	pub fn infer_no_type_anotation(&self, type_id: TypeId) -> TyResult<TypeId> {
		if type_id.is_known() {
			return Ok(type_id);
		}
		Ok(match self.get_stored_type(type_id) {
			Type::NumRange(range) => TypeId::from(&range.as_number()),
			_ => type_id,
		})
	}
}
