use crate::message::MessageResult;

use super::types::{Type, TypeId};
use super::Checker;

impl Checker<'_> {
	pub fn infer_type_from_expected(&self, expected: TypeId, found: TypeId) -> TypeId {
		if found.is_builtin_type() {
			return found;
		}
		if let Type::NumRange(found_range) = self.get_stored_type(found) {
			return found_range.try_resolve_with_type(expected).unwrap_or(found);
		}
		found
	}

	pub fn infer_default_type(&self, found: TypeId) -> TypeId {
		if found.is_builtin_type() {
			return found;
		}
		if let Type::NumRange(found_range) = self.get_stored_type(found) {
			return found_range.into();
		}
		found
	}

	pub fn unify_types(&self, left: TypeId, right: TypeId) -> MessageResult<Option<TypeId>> {
		if left.is_builtin_type() && right.is_builtin_type() {
			return Ok(None);
		}
		let left_type = self.get_stored_type(left);
		let right_type = self.get_stored_type(right);

		let result = match (left_type, right_type) {
			(Type::NumRange(lt_range), Type::NumRange(rt_range)) => {
				lt_range.unify_range(rt_range).map(|max| TypeId::from(&max.to_number()))
			}
			(Type::NumRange(_), _) => Some(self.infer_type_from_expected(right, left)),
			(_, Type::NumRange(_)) => Some(self.infer_type_from_expected(left, right)),
			_ => None,
		};

		Ok(result)
	}
}
