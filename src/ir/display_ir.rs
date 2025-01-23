use super::{
	ir::{self},
	IrValue,
};
use crate::{
	checker::types::{TypeId, TypeStore},
	report::text_green,
};

// colors
//

impl ir::Root {
	pub fn display_ir(&self, type_store: &TypeStore) -> String {
		let mut text = String::new();

		for struct_ir in self.structs.iter() {
			struct_ir.display_ir(type_store, &mut text);
		}

		for fn_ir in self.fns.iter() {
			fn_ir.display_ir(type_store, &mut text);
		}
		text
	}
}

impl ir::LnFn {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let ret = type_store.get_display_type(self.ret);

		let args = self.args.iter().map(|param| param.display_ir(type_store));
		let args = args.collect::<Vec<_>>().join(", ");
		let instr = if self.args.is_empty() {
			format!("{} {} -> {}\n", text_green("fn"), self.fn_id, ret)
		} else {
			format!("{} {} {} -> {}\n", text_green("fn"), self.fn_id, args.trim(), ret)
		};
		text.push_str(&instr);
		for block in self.blocks.iter() {
			block.display_ir(type_store, text);
		}
		text.push('\n');
	}
}

impl ir::ExFn {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let args = self.args.iter().map(|param| param.display_ir(type_store));
		let mut args = args.collect::<Vec<_>>().join(", ");
		if self.var_packed {
			args += ", ..."
		}
		let ret = type_store.get_display_type(self.ret);
		let instr = if self.args.is_empty() {
			format!("{} {} -> {}\n", text_green("extern fn"), self.fn_id, ret)
		} else {
			format!("{} {} {} -> {}\n", text_green("extern fn"), self.fn_id, args.trim(), ret)
		};
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::Fn {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		match self {
			ir::Fn::Ln(fn_ir) => fn_ir.display_ir(type_store, text),
			ir::Fn::Ex(fn_ir) => fn_ir.display_ir(type_store, text),
		}
	}
}

impl ir::Block {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let block_id = self.block_id.as_colored();
		text.push_str(format!("{}:", block_id).as_str());

		for (index, instr) in self.instrs.iter().enumerate() {
			if index == 0 {
				text.push(' ');
				instr.display_ir(type_store, text);
			} else {
				text.push_str("    ");
				instr.display_ir(type_store, text);
			}
		}
		if self.instrs.is_empty() {
			text.push('\n');
		}
		text.push_str(format!("    {}\n", text_green("_")).as_str());
	}
}

impl ir::Bind {
	fn display_ir(&self, type_store: &TypeStore) -> String {
		let type_str = type_store.get_display_type(self.type_id);
		let register = self.register.as_colored();
		format!("{} {}", register, type_str)
	}
}

impl ir::Instr {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		match self {
			ir::Instr::Add(binary) => binary.display_ir("add", type_store, text),
			ir::Instr::Sub(binary) => binary.display_ir("sub", type_store, text),
			ir::Instr::Div(binary) => binary.display_ir("div", type_store, text),
			ir::Instr::Mul(binary) => binary.display_ir("mul", type_store, text),
			ir::Instr::Mod(binary) => binary.display_ir("mod", type_store, text),
			ir::Instr::CmpGt(binary) => binary.display_ir("cmp_gt", type_store, text),
			ir::Instr::CmpEq(binary) => binary.display_ir("cmp_eq", type_store, text),
			ir::Instr::CmpLt(binary) => binary.display_ir("cmp_lt", type_store, text),
			ir::Instr::CmpLe(binary) => binary.display_ir("cmp_le", type_store, text),
			ir::Instr::CmpGe(binary) => binary.display_ir("cmp_ge", type_store, text),
			ir::Instr::Load(unary) => unary.display_ir("load", type_store, text),
			ir::Instr::Deref(unary) => unary.display_ir("deref", type_store, text),
			ir::Instr::Store(store) => store.display_ir(type_store, text),
			ir::Instr::Heap(heap) => heap.display_ir(type_store, text),
			ir::Instr::Free(free) => free.display_ir(text),
			ir::Instr::Own(own) => own.display_ir(type_store, text),
			ir::Instr::OwnHeap(own) => own.display_ir(type_store, text),
			ir::Instr::Borrow(unary) => unary.display_ir("borrow", type_store, text),
			ir::Instr::BorrowMut(unary) => unary.display_ir("borrow_mut", type_store, text),
			ir::Instr::JmpIf(jmp_if) => jmp_if.display_ir(type_store, text),
			ir::Instr::Ret(ret) => ret.display_ir(type_store, text),
			ir::Instr::Call(call) => call.display_ir(type_store, text),
			ir::Instr::Goto(goto) => goto.display_ir(text),
			ir::Instr::Cache(_) => todo!(),
			ir::Instr::Drop(drop) => drop.display_ir(text),
			ir::Instr::LoadField(load_field) => load_field.display_ir(type_store, text),
			ir::Instr::StoreField(store_field) => store_field.display_ir(type_store, text),
			// ir::Instr::SetCache(set_cache) => set_cache.display_ir(text),
			// ir::Instr::LoadCache(load_cache) => load_cache.display_ir(text),
		}
	}
}

impl ir::StructInstr {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		// let type_str = type_store.get_display_type(self.type_id);
		let instr = format!("{} {} = ", text_green("struct"), self.struct_id);
		text.push_str(&instr);
		text.push('\n');
		for field in self.fields.iter() {
			let field_type = type_store.get_display_type(field.type_id);
			let register = field.register.as_colored();
			text.push_str(&format!(" {} {}", field_type, register));
			text.push('\n');
		}
		text.push_str(format!(" {}\n", text_green("_")).as_str());
		text.push('\n');
	}
}

impl ir::DropInstr {
	pub fn display_ir(&self, text: &mut String) {
		let value = self.value.as_colored();
		let instr = format!("drop {}", value);
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::LoadFieldInstr {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let type_str = type_store.get_display_type(self.type_id);
		let value = self.value.as_colored();
		let field = self.field.as_str();
		let dest = self.dest.as_colored();
		let instr = format!("load_field {} {} {} -> {}", type_str, value, field, dest);
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::StoreFieldInstr {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let type_str = type_store.get_display_type(self.type_id);
		let value = self.value.as_colored();
		let field = self.field.as_str();
		let dest = self.dest.as_colored();
		let instr = format!("store_field {} {} {} {}", type_str, value, field, dest);
		text.push_str(&instr);
		text.push('\n');
	}
}

// impl ir::StoreStructInstr {
// 	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
// 		let type_str = type_store.get_display_type(self.type_id);
// 		let instr = format!("struct {} {} ", type_str, self.name);
// 		text.push_str(&instr);
// 		text.push('\n');
// 	}
// }

impl ir::BinaryInstr {
	pub fn display_ir(&self, operator: &str, type_store: &TypeStore, text: &mut String) {
		let type_str = type_store.get_display_type(self.type_id);
		let lhs = self.lhs.as_colored();
		let rhs = self.rhs.as_colored();
		let dest = self.dest.as_colored();
		let instr = format!("{} {} {} {} -> {}", operator, lhs, rhs, type_str, dest);
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::UnaryInstr {
	pub fn display_ir(&self, operator: &str, type_store: &TypeStore, text: &mut String) {
		let type_str = type_store.get_display_type(self.type_id);
		let value = self.value.as_colored();
		let dest = self.dest.as_colored();
		let instr = format!("{} {} {} -> {}", operator, type_str, value, dest);
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::JmpIfInstr {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let cond = self.cond.as_colored();
		let type_str = type_store.get_display_type(TypeId::BOOL);
		let l0 = self.l0.as_string();
		let l1 = self.l1.as_string();
		let instr = format!("jmp_if {} {} -> {}, {}", type_str, cond, l0, l1);
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::GotoInstr {
	pub fn display_ir(&self, text: &mut String) {
		let block_id = self.block_id.as_string();
		let instr = format!("goto {}", text_green(&block_id));
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::FreeInstr {
	pub fn display_ir(&self, text: &mut String) {
		let register = self.register.as_colored();
		let instr = format!("free {}", register);
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::OwnInstr {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let type_str = type_store.get_display_type(self.type_id);
		let value = self.value.as_colored();
		let dest = self.dest.as_colored();
		let instr = format!("own {} {} -> {}", type_str, value, dest);
		text.push_str(&instr);
		text.push('\n');
	}
}
impl ir::OwnHeapInstr {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let type_str = type_store.get_display_type(self.type_id);
		let value = self.value.as_colored();
		let dest = self.dest.as_colored();
		let instr = format!("own_heap {} {} -> {} size={}", type_str, value, dest, self.size);
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::HeapInstr {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let type_str = type_store.get_display_type(self.type_id);
		let value = self.value.as_string();
		let dest = self.dest.as_colored();
		let instr = format!("heap {} {} -> {} size={}", type_str, value, dest, self.size);
		text.push_str(&instr);
		text.push('\n');
	}
}
impl ir::StoreInstr {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let type_str = type_store.get_display_type(self.type_id);
		let value = self.value.as_string();
		let dest = self.dest.as_colored();
		let instr = format!("store {} {} -> {}", type_str, value, dest);
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::RetInstr {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let type_str = type_store.get_display_type(self.type_id);
		let instr = match &self.value {
			Some(value) => {
				format!("ret {} {}", type_str, value.as_colored())
			}
			None => format!("ret {}", type_str),
		};
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::CallInstr {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let type_str = type_store.get_display_type(self.type_id);
		let dest = self.dest.as_colored();
		let args = self.args.iter().map(|r| r.display_ir(type_store)).collect::<Vec<_>>().join(", ");
		let instr = format!("call {} {} {} -> {}", type_str, self.fn_id, args, dest);
		text.push_str(&instr);
		text.push('\n');
	}
}

// == ir values ==
impl From<String> for IrValue {
	fn from(value: String) -> Self {
		IrValue::String(value)
	}
}

impl From<char> for IrValue {
	fn from(value: char) -> Self {
		IrValue::Char(value)
	}
}
impl From<bool> for IrValue {
	fn from(value: bool) -> Self {
		IrValue::Bool(value)
	}
}

impl From<i64> for IrValue {
	fn from(value: i64) -> Self {
		IrValue::Int(value)
	}
}
impl From<f64> for IrValue {
	fn from(value: f64) -> Self {
		IrValue::Float(value)
	}
}

impl IrValue {
	pub fn as_string(&self) -> String {
		match self {
			IrValue::Int(int) => int.to_string(),
			IrValue::Float(float) => float.to_string(),
			IrValue::Bool(bool) => bool.to_string(),
			IrValue::String(string) => {
				let string = string.replace("\n", "\\n").replace("\t", "\\t").replace("\r", "\\r");
				format!("\"{}\"", string)
			}
			IrValue::Char(char) => format!("'{}'", char),
			IrValue::Reg(reg) => reg.as_colored(),
			IrValue::Value(value) => value.to_string(),
		}
	}
}
