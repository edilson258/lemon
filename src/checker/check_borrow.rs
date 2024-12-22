use crate::range::Range;

use super::context::value::ValueId;
use super::diags::TypeCheckError;
use super::types::{Type, TypeId};
use super::{Checker, TypeResult};

impl Checker<'_> {
	pub fn check_borrow(
		&mut self,
		value_id: ValueId,
		type_id: TypeId,
		range: Range,
	) -> TypeResult<()> {
		if let Type::Ref(ref_type) = self.get_stored_type(type_id)? {
			let id = self.ctx.add_borrow(value_id, ref_type.mutable);
			if id.is_none() {
				return Err(TypeCheckError::borrow_conflict(range));
			}
			return Ok(());
		}
		Ok(())
	}
}
