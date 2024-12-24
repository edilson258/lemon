use super::{type_id::TypeId, FloatType, IntType, Type, UsizeType};

#[derive(Debug)]
pub struct TypeStore {
	types: Vec<Type>,
}

impl TypeStore {
	pub fn new(types: Vec<Type>) -> Self {
		Self { types }
	}
	pub fn add_type(&mut self, ty: Type) -> TypeId {
		self.types.push(ty);
		TypeId(self.types.len() - 1)
	}

	pub fn get_type(&self, type_id: TypeId) -> Option<&Type> {
		self.types.get(type_id.as_usize())
	}
}

impl Default for TypeStore {
	fn default() -> Self {
		let types = vec![
			Type::None,                    // 0
			Type::Bool,                    // 1
			Type::Str,                     // 2
			Type::String,                  // 3
			Type::Char,                    // 4
			Type::Int(IntType::I8),        // 5
			Type::Int(IntType::I16),       // 6
			Type::Int(IntType::I32),       // 7
			Type::Int(IntType::I64),       // 8
			Type::Int(IntType::Int),       // 9
			Type::Usize(UsizeType::U8),    // 10
			Type::Usize(UsizeType::U16),   // 11
			Type::Usize(UsizeType::U32),   // 12
			Type::Usize(UsizeType::U64),   // 13
			Type::Usize(UsizeType::Usize), // 14
			Type::Float(FloatType::F32),   // 15
			Type::Float(FloatType::F64),   // 16
		];

		assert_eq!(types.len(), TypeId::LENGTH);
		Self::new(types)
	}
}
