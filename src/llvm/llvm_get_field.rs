use super::Llvm;
use crate::{error_codegen, ir};
use inkwell::types::StructType;

type Pointer<'ll> = inkwell::values::PointerValue<'ll>;

impl<'ll> Llvm<'ll> {
	pub fn llvm_get_field(&mut self, get_field_instr: &ir::GetFieldInstr) {
		let self_value = get_field_instr.self_value;
		let field = get_field_instr.field;

		let struct_value = *self.stack.get_value(self_value);

		if !struct_value.is_pointer_value() {
			let struct_value = struct_value.into_struct_value();
			let struct_type = struct_value.get_type(); // is panic if is not a struct
			let struct_name = self.get_struct_name(&struct_type, self_value);
			let field_index = self.get_field_index(struct_name.as_str(), field) as u32;
			let temp = self.stack.temp_register();
			let field_value = match self.builder.build_extract_value(struct_value, field_index, &temp) {
				Ok(value) => value,
				Err(e) => throw_llvm_error(format!("failed to get field value: {}", e)),
			};
			self.stack.set_value(get_field_instr.dest, field_value);
			return;
		}

		let struct_ptr = struct_value.into_pointer_value();
		let struct_type = self.get_struct_type(self_value);
		let struct_name = self.get_struct_name(&struct_type, self_value);
		let field_index = self.get_field_index(struct_name.as_str(), field);

		let field_ptr = self.build_field_pointer(struct_type, struct_ptr, field_index);
		// let field_value = self.load_field_value(field_ptr, field_type, &get_field_instr.dest);
		self.stack.set_value(get_field_instr.dest, field_ptr.into());
	}

	pub fn get_struct_type(&mut self, self_value: ir::Register) -> StructType<'ll> {
		self.stack.get_register_type(self_value).copied().unwrap_or_else(|| {
			let register = self_value.as_string();
			throw_llvm_error(format!("register '{}' not found in struct table", register))
		})
	}

	pub fn get_struct_name(
		&mut self,
		struct_type: &StructType<'ll>,
		self_reg: ir::Register,
	) -> String {
		let struct_name = match struct_type.get_name() {
			Some(name) => name.to_str().unwrap_or_else(|_| throw_llvm_error("invalid struct name")),
			None => throw_llvm_error(format!("struct type '{}' not found", self_reg.as_string())),
		};
		struct_name.to_string()
	}

	pub fn get_field_index(&mut self, struct_name: &str, field: ir::Register) -> usize {
		self.stack.get_struct_field(struct_name, field).unwrap_or_else(|| {
			let error = format!("field '{}' not found in struct '{}'", field.as_string(), struct_name);
			throw_llvm_error(error)
		})
	}


	#[rustfmt::skip]
	pub fn build_field_pointer(&mut self, ll_type: StructType<'ll>,	ptr: Pointer<'ll>, idx: usize) -> Pointer<'ll> {
		let i32_type = self.ctx.i32_type();
		let zero = i32_type.const_zero();
		let position = i32_type.const_int(idx as u64, false);
		let temp = self.stack.temp_register();
		let params = [zero, position];
		unsafe {
			match self.builder.build_gep(ll_type, ptr, &params, &temp) {
				Ok(value) => value,
				Err(e) => throw_llvm_error(format!("failed to get field ptr: {}", e)),
			}
		}
	}
	// #[rustfmt::skip]
	// pub	fn load_field_value(&mut self, ptr: Pointer<'ll>, type_id: TypeId, dest: &ir::Register) -> BasicValueEnum<'ll> {
	// 	let field_type = self.resolve_llvm_type(type_id);
	// 	match self.builder.build_load(field_type, ptr, &dest.as_string()) {
	// 		Ok(value) => value,
	// 		Err(e) => throw_llvm_error(format!("failed to load field value: {}", e)),
	// 	}
	// }
}
