use inkwell::{types::BasicTypeEnum, AddressSpace};

use crate::{checker::types::TypeId, report::throw_llvm_error};

use super::Llvm;
impl<'ll> Llvm<'ll> {
	pub fn llvm_type_from_type(&self, type_id: TypeId) -> Option<BasicTypeEnum<'ll>> {
		let found = self.type_store.resolve_borrow_type(type_id);
		match found {
			TypeId::I8 | TypeId::U8 | TypeId::CHAR => Some(self.ctx.i8_type().into()),
			TypeId::I16 | TypeId::U16 => Some(self.ctx.i16_type().into()),
			TypeId::I32 | TypeId::U32 => Some(self.ctx.i32_type().into()),
			TypeId::I64 | TypeId::U64 => Some(self.ctx.i64_type().into()),
			TypeId::F32 => Some(self.ctx.f32_type().into()),
			TypeId::F64 => Some(self.ctx.f64_type().into()),
			TypeId::BOOL => Some(self.ctx.bool_type().into()),
			TypeId::STRING | TypeId::STR => {
				let address_space = AddressSpace::default();
				Some(self.ctx.ptr_type(address_space).into())
			}
			TypeId::UNIT | TypeId::VOID => None, // void
			found => {
				let text = self.type_store.get_display_type(found);
				throw_llvm_error(format!("type '{}' not found", text))
			}
		}
	}
}
