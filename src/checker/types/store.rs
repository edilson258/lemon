use std::hash::{Hash, Hasher};

use rustc_hash::{FxHashMap, FxHasher};

use super::{
	monomorphic::MonomorphicStore, type_id::TypeId, ExternFnType, FnType, InferType, Number, Type,
};

#[derive(Debug)]
pub struct TypeStore {
	types: Vec<Type>,
	types_by_name: FxHashMap<String, TypeId>,
	generics: FxHashMap<String, TypeId>,
	// is good?
	cache: FxHashMap<u64, TypeId>,
	pub monomorphic_store: MonomorphicStore,
}

impl TypeStore {
	pub fn new(types: Vec<Type>) -> Self {
		let monomorphic_store = MonomorphicStore::default();
		let types_by_name = FxHashMap::default();
		let cache = FxHashMap::default();
		let generics = FxHashMap::default();
		Self { types, types_by_name, cache, generics, monomorphic_store }
	}

	pub fn add_monomo_fn(&mut self, fn_type: FnType) {
		self.monomorphic_store.add_fn(fn_type);
	}

	pub fn add_monomo_extern_fn(&mut self, fn_type: ExternFnType) {
		self.monomorphic_store.add_extern_fn(fn_type);
	}

	pub fn create_monomo_fn(&mut self, name: String) {
		self.monomorphic_store.create_fn(name);
	}

	pub fn end_monomo_fn(&mut self) {
		self.monomorphic_store.end_fn();
	}

	pub fn add_infer_type(&mut self, generic: InferType) -> TypeId {
		let id = generic.id.clone();
		let type_id = self.add_type(generic.into());
		self.generics.insert(id, type_id);
		type_id
	}

	pub fn get_infer_type(&self, id: &str) -> Option<&InferType> {
		let type_id = self.get_infer_id(id)?;
		match self.get_type(*type_id) {
			Some(Type::Infer(infer)) => Some(infer),
			_ => None,
		}
	}

	pub fn get_infer_id(&self, id: &str) -> Option<&TypeId> {
		self.generics.get(id)
	}

	pub fn get_type_by_name(&self, name: &str) -> Option<&TypeId> {
		self.types_by_name.get(name)
	}

	pub fn add_type_by_name(&mut self, name: String, type_id: TypeId) {
		self.types_by_name.insert(name, type_id);
	}

	pub fn add_type(&mut self, ty: Type) -> TypeId {
		// todo: is faster to generate hash and compare?
		let hash = self.type_hash(&ty);
		if let Some(type_id) = self.cache.get(&hash) {
			return *type_id;
		}
		let type_id = TypeId(self.types.len() as u64);

		self.cache.insert(hash, type_id);
		self.types.push(ty);
		type_id
	}

	pub fn type_hash(&self, ty: &Type) -> u64 {
		let mut hasher = FxHasher::default();
		ty.hash(&mut hasher);
		hasher.finish()
	}
	pub fn get_type(&self, type_id: TypeId) -> Option<&Type> {
		self.types.get(type_id.as_usize())
	}
	pub fn get_mut_type(&mut self, type_id: TypeId) -> Option<&mut Type> {
		self.types.get_mut(type_id.as_usize())
	}

	pub fn resolve_borrow_type(&self, type_id: TypeId) -> TypeId {
		if type_id.is_known() {
			return type_id;
		}
		let type_value = self.get_type(type_id).unwrap();
		match type_value {
			Type::Borrow(borrow) => self.resolve_borrow_type(borrow.value),
			_ => type_id,
		}
	}

	pub fn get_display_type(&self, type_id: TypeId) -> String {
		let mut text = String::new();
		let type_value = self.get_type(type_id).unwrap();
		type_value.display_type(&mut text, self, false);
		text
	}

	pub fn get_display_ir_type(&self, type_id: TypeId) -> String {
		let mut text = String::new();
		let type_value = self.get_type(type_id).unwrap();
		type_value.display_ir_type(&mut text, self);
		text
	}

	pub fn get_struct_name(&self, type_id: TypeId) -> &str {
		let type_value = self.get_type(type_id).unwrap();
		match type_value {
			Type::Struct(struct_type) => &struct_type.name,
			_ => panic!("not a struct type"),
		}
	}

	// checks if needs to free
	pub fn needs_free(&self, type_id: TypeId) -> bool {
		if type_id.is_known() {
			return false;
		}
		let type_value = self.get_type(type_id).expect("type not found");
		if let Type::Borrow(borrow) = type_value {
			return self.needs_free(borrow.value);
		}
		type_value.needs_free()
	}
}

impl Default for TypeStore {
	fn default() -> Self {
		let types = vec![
			Type::Void,   // 0
			Type::Bool,   // 1
			Type::Str,    // 2
			Type::String, // 3
			Type::Char,   // 4
			// isize
			Number::I8.as_type(),    // 5
			Number::I16.as_type(),   // 6
			Number::I32.as_type(),   // 7
			Number::I64.as_type(),   // 8
			Number::Isize.as_type(), // 9
			// usize
			Number::U8.as_type(),    // 10
			Number::U16.as_type(),   // 11
			Number::U32.as_type(),   // 12
			Number::U64.as_type(),   // 13
			Number::Usize.as_type(), // 14
			// float
			Number::F32.as_type(), // 15
			Number::F64.as_type(), // 16
			// internal
			Type::Unit, // 17
			Type::Any,  // 18
		];
		assert_eq!(types.len(), TypeId::LENGTH);
		Self::new(types)
	}
}
