use super::{ConstType, FloatType, FnType, IntType, RefType, Type, TypeId, TypeStore, UsizeType};

pub struct TypeFormatter<'tf> {
	pub type_store: &'tf TypeStore,
}

impl<'tf> TypeFormatter<'tf> {
	pub fn new(type_store: &'tf TypeStore) -> Self {
		Self { type_store }
	}

	pub fn format(&self, type_id: TypeId) -> String {
		let mut type_str = String::new();
		self.format_inner(type_id, &mut type_str);
		type_str
	}

	fn format_inner(&self, type_id: TypeId, type_str: &mut String) {
		match self.type_store.get_type(type_id) {
			Some(found_type) => self.format_inner_type(found_type, type_str),
			None => type_str.push_str("???"),
		}
	}

	fn format_inner_type(&self, found_type: &Type, type_str: &mut String) {
		match found_type {
			Type::None => type_str.push_str("nothing"),
			Type::Bool => type_str.push_str("bool"),
			Type::Str => type_str.push_str("str"),
			Type::String => type_str.push_str("string"),
			Type::Char => type_str.push_str("char"),
			Type::Int(int) => self.format_inner_int(int, type_str),
			Type::Usize(usize) => self.format_inner_usize(usize, type_str),
			Type::Float(float) => self.format_inner_float(float, type_str),
			Type::Ref(ref_type) => self.format_inner_ref(ref_type, type_str),
			Type::Fn(fn_type) => self.format_inner_fn(fn_type, type_str),
			Type::Par { target } => self.format_inner(*target, type_str),
			Type::InferInt { bits } => self.format_inner_infer_int(bits, type_str),
			Type::Const(const_type) => self.format_inner_const(const_type, type_str),
		}
	}

	fn format_inner_int(&self, int: &IntType, type_str: &mut String) {
		match int {
			IntType::I8 => type_str.push_str("i8"),
			IntType::I16 => type_str.push_str("i16"),
			IntType::I32 => type_str.push_str("i32"),
			IntType::I64 => type_str.push_str("i64"),
			IntType::Int => type_str.push_str("int"),
		}
	}

	fn format_inner_usize(&self, usize: &UsizeType, type_str: &mut String) {
		match usize {
			UsizeType::U8 => type_str.push_str("u8"),
			UsizeType::U16 => type_str.push_str("u16"),
			UsizeType::U32 => type_str.push_str("u32"),
			UsizeType::U64 => type_str.push_str("u64"),
			UsizeType::Usize => type_str.push_str("usize"),
		}
	}

	fn format_inner_float(&self, float: &FloatType, type_str: &mut String) {
		match float {
			FloatType::F32 => type_str.push_str("f32"),
			FloatType::F64 => type_str.push_str("f64"),
		}
	}

	fn format_inner_ref(&self, ref_type: &RefType, type_str: &mut String) {
		type_str.push('&');
		if ref_type.mutable {
			type_str.push_str("mut ");
		}
		self.format_inner(ref_type.value, type_str);
	}

	fn format_inner_fn(&self, fn_type: &FnType, type_str: &mut String) {
		type_str.push_str("fn(");
		for (i, arg) in fn_type.args.iter().enumerate() {
			if i > 0 {
				type_str.push_str(", ");
			}
			self.format_inner(*arg, type_str);
		}
		type_str.push_str(") -> ");
		self.format_inner(fn_type.ret, type_str);
	}

	fn format_inner_infer_int(&self, bits: &u8, type_str: &mut String) {
		type_str.push('i');
		type_str.push_str(if *bits <= 32 { "32" } else { "64" });
		type_str.push('?');
	}

	fn format_inner_const(&self, const_type: &ConstType, type_str: &mut String) {
		self.format_inner(const_type.value, type_str);
	}
}
