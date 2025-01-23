use rustc_hash::FxHashMap;

use crate::range::Range;

use super::{Checker, TyResult};

use super::types::{FnType, Type, TypeId};
type GenericTable = FxHashMap<TypeId, TypeId>;

impl Checker<'_> {
	pub fn infer_generic(&self, callee: TypeId, args: &[(TypeId, Range)]) -> TyResult<Type> {
		let found = self.get_stored_type(callee);
		match found {
			Type::Fn(fn_type) => self.infer_fun_generic(fn_type, args),
			_ => Ok(found.clone()),
		}
	}

	fn infer_fun_generic(&self, fn_type: &FnType, args: &[(TypeId, Range)]) -> TyResult<Type> {
		let table = self.create_generic_table(&fn_type.generics, args);
		let ret = self.infer_generic_type(&fn_type.ret, &table);
		let args = self.infer_generics_type(&fn_type.args, &table);
		let table_values: Vec<TypeId> = table.values().copied().collect();
		let mut fn_type = FnType::new(args, ret);
		fn_type.set_generics(table_values);
		Ok(fn_type.into())
	}

	fn infer_generic_type(&self, type_id: &TypeId, table: &GenericTable) -> TypeId {
		*table.get(type_id).unwrap_or(type_id)
	}

	fn infer_generics_type(&self, types: &[TypeId], table: &GenericTable) -> Vec<TypeId> {
		types.iter().map(|type_id| self.infer_generic_type(type_id, table)).collect()
	}

	fn create_generic_table(&self, generics: &[TypeId], args: &[(TypeId, Range)]) -> GenericTable {
		let mut table = FxHashMap::default();
		for (generic, arg) in generics.iter().zip(args) {
			table.insert(*generic, arg.0);
		}
		table
	}
}
