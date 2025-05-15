use crate::message::MessageResult;
use crate::range::Range;

use super::diags::SyntaxErr;
use super::types::{Type, TypeId};
use super::Checker;

impl Checker<'_> {
	pub fn infer_type_from_expected(&self, expected: TypeId, found: TypeId) -> TypeId {
		if found.is_builtin_type() {
			return found;
		}
		if let Type::NumRange(found_range) = self.lookup_stored_type(found) {
			return found_range.try_resolve_with_type(expected).unwrap_or(found);
		}
		found
	}

	pub fn infer_default_type(&self, found: TypeId) -> TypeId {
		if found.is_builtin_type() {
			return found;
		}
		if let Type::NumRange(found_range) = self.lookup_stored_type(found) {
			return TypeId::I64;
		}
		found
	}

	pub fn unify_types(&self, left: TypeId, right: TypeId) -> MessageResult<Option<TypeId>> {
		if left.is_builtin_type() && right.is_builtin_type() {
			return Ok(None);
		}
		let left_type = self.lookup_stored_type(left);
		let right_type = self.lookup_stored_type(right);

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

	pub fn unify_types_with_default(&self, left: TypeId, right: TypeId) -> MessageResult<TypeId> {
		let result = self.unify_types(left, right)?;
		Ok(result.unwrap_or_else(|| self.infer_default_type(left)))
	}

	#[rustfmt::skip]
	pub fn unify_types_expected(&self, expected: TypeId,found: TypeId,range: Range) -> MessageResult<TypeId> {
		let result = self.unify_types(expected, found)?;
		if let Some(result) = result {
			return Ok(result);
		}
		let expected = self.display_type(expected);
		let found = self.display_type(found);
		Err(SyntaxErr::type_mismatch(expected, found, range))
	}
}
