use crate::range::Range;

use super::diags::SyntaxErr;
use super::{Checker, TyResult};

use super::types::TypeId;

impl Checker<'_> {
	pub fn equal_type_id(&self, expected: TypeId, found: TypeId) -> bool {
		if expected == found {
			return true;
		}
		if expected.is_unit() && found.is_void() || found.is_unit() && expected.is_void() {
			return true;
		}

		let expected_type = self.get_stored_type(expected);
		let found_type = self.get_stored_type(found);
		expected_type == found_type
	}

	pub fn equal_type_expected(&self, expected: TypeId, found: TypeId, range: Range) -> TyResult<()> {
		if !self.equal_type_id(expected, found) {
			let expected = self.display_type(expected);
			let found = self.display_type(found);
			return Err(SyntaxErr::type_mismatch(expected, found, range));
		}
		Ok(())
	}
}
