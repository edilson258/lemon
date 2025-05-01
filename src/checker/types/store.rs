use std::hash::{Hash, Hasher};

use rustc_hash::{FxHashMap, FxHasher};

use crate::loader::ModId;

use super::{type_id::TypeId, InferType, Number, Type};

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum LookUpGeneric {
// 	NumberPropertyOfSelf, // Property(Box<Self>, PropertyKey<'static>),
// }

// pub type LookUpGenericMap = FxHashMap<TypeId, LookUpGeneric>;

#[derive(Debug)]
pub struct TypeStore {
	/// Contains all of the types. Indexed by [`TypeId`]
	types: Vec<Type>,

	// lookup_generic_map: FxHashMap<TypeId, LookUpGenericMap>,
	type_definitions: FxHashMap<String, TypeId>,
	generics: FxHashMap<String, TypeId>,
	// is good?
	cache: FxHashMap<u64, TypeId>,
	pub mods: FxHashMap<ModId, TypeId>,
}

impl TypeStore {
	pub fn new(types: Vec<Type>) -> Self {
		let type_definitions = FxHashMap::default();
		let cache = FxHashMap::default();
		let generics = FxHashMap::default();
		let mods = FxHashMap::default();
		Self { types, type_definitions, cache, generics, mods }
	}

	pub fn add_mod(&mut self, mod_id: ModId, type_id: TypeId) {
		self.mods.insert(mod_id, type_id);
	}

	pub fn lookup_mod(&self, mod_id: ModId) -> Option<&TypeId> {
		self.mods.get(&mod_id)
	}

	pub fn add_mod_name(&mut self, type_id: TypeId, name: impl Into<String>) {
		if type_id.is_builtin_type() {
			return;
		}
		if let Some(Type::Mod(module_type)) = self.lookup_mut_type(type_id) {
			module_type.set_name(name.into());
		}
	}

	pub fn add_infer_type(&mut self, generic: InferType) -> TypeId {
		let id = generic.id.clone();
		let type_id = self.add_type(generic.into());
		self.generics.insert(id, type_id);
		type_id
	}

	pub fn lookup_infer_type(&self, id: &str) -> Option<&InferType> {
		let type_id = self.lookup_infer_id(id)?;
		match self.lookup_type(*type_id) {
			Some(Type::Infer(infer)) => Some(infer),
			_ => None,
		}
	}

	pub fn lookup_infer_id(&self, id: &str) -> Option<&TypeId> {
		self.generics.get(id)
	}

	pub fn lookup_type_definition(&self, name: &str) -> Option<&TypeId> {
		self.type_definitions.get(name)
	}

	pub fn add_type_definition(&mut self, name: String, type_id: TypeId) {
		self.type_definitions.insert(name, type_id);
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
	pub fn lookup_type(&self, type_id: TypeId) -> Option<&Type> {
		self.types.get(type_id.as_usize())
	}
	pub fn lookup_mut_type(&mut self, type_id: TypeId) -> Option<&mut Type> {
		self.types.get_mut(type_id.as_usize())
	}

	pub fn resolve_borrow_type(&self, type_id: TypeId) -> TypeId {
		if type_id.is_builtin_type() {
			return type_id;
		}
		let type_value = self.lookup_type(type_id).unwrap();
		match type_value {
			Type::Borrow(borrow) => self.resolve_borrow_type(borrow.value),
			_ => type_id,
		}
	}

	pub fn lookup_display_type(&self, type_id: TypeId) -> String {
		let mut text = String::new();
		let type_value = self.lookup_type(type_id).unwrap();
		type_value.display_type(&mut text, self, false);
		text
	}

	pub fn lookup_display_ir_type(&self, type_id: TypeId) -> String {
		let mut text = String::new();
		let type_value = self.lookup_type(type_id).unwrap();
		if type_value.is_borrow() {
			return "ptr".to_owned();
		}
		type_value.display_ir_type(&mut text, self);
		text
	}

	pub fn lookup_struct_name(&self, type_id: TypeId) -> Option<&str> {
		let type_value = self.lookup_type(type_id);
		if let Some(Type::Struct(struct_type)) = type_value {
			Some(struct_type.name.as_str())
		} else {
			None
		}
	}

	pub fn is_borrow(&self, type_id: TypeId) -> bool {
		if type_id.is_builtin_type() {
			return false;
		}
		let type_value = self.lookup_type(type_id).expect("type not found");
		type_value.is_borrow()
	}

	pub fn is_module(&self, type_id: TypeId) -> bool {
		if type_id.is_builtin_type() {
			return false;
		}
		let type_value = self.lookup_type(type_id).expect("type not found");
		type_value.is_module()
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
			Number::I8.into(),    // 5
			Number::I16.into(),   // 6
			Number::I32.into(),   // 7
			Number::I64.into(),   // 8
			Number::Isize.into(), // 9
			// usize
			Number::U8.into(),    // 10
			Number::U16.into(),   // 11
			Number::U32.into(),   // 12
			Number::U64.into(),   // 13
			Number::Usize.into(), // 14
			// float
			Number::F32.into(), // 15
			Number::F64.into(), // 16
			// internal
			Type::Unit, // 17
		];
		assert_eq!(types.len(), TypeId::LENGTH);
		Self::new(types)
	}
}
