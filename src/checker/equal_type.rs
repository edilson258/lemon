use crate::range::Range;

use super::{diags::TypeCheckError, Checker, TypeResult};

use super::types::{Type, TypeId};

impl Checker<'_> {
	pub fn equal_type_id(&self, expected: TypeId, found: TypeId, range: Range) -> TypeResult<()> {
		let found_id = self.infer_type(expected, found)?;
		let expected_ty = self.resolve_par(expected)?;

		let found_ty = self.resolve_par(found_id)?;
		if expected_ty != found_ty {
			let expected = self.format(expected);
			let found = self.format(found_id);
			return Err(TypeCheckError::type_mismatch(expected, found, range));
		}
		Ok(())
	}

	pub fn resolve_par(&self, par_id: TypeId) -> TypeResult<&Type> {
		let par_type = self.get_stored_type(par_id)?;
		match par_type {
			Type::Par { target } => self.resolve_par(*target),
			_ => Ok(par_type),
		}
	}
}
