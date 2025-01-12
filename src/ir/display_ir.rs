use super::{
	ir::{self},
	IrValue,
};
use crate::checker::types::{TypeId, TypeStore};

impl ir::Root {
	pub fn display_ir(&self, type_store: &TypeStore) -> String {
		let mut text = String::new();
		for fn_ir in self.fns.iter() {
			fn_ir.display_ir(type_store, &mut text);
		}
		text
	}
}

impl ir::Fn {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let fn_id = self.fn_id.as_string();
		let ret = type_store.get_display_type(self.ret);

		let params = self.params.iter().map(|param| param.display_ir(type_store));
		let params = params.collect::<Vec<_>>().join(", ");
		let instr = if self.params.is_empty() {
			format!("fn {} -> {}\n", fn_id, ret)
		} else {
			format!("fn {} {} -> {}\n", fn_id, params.trim(), ret)
		};
		text.push_str(&instr);
		for block in self.blocks.iter() {
			block.display_ir(type_store, text);
		}
		text.push('\n');
	}
}

impl ir::Block {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let block_id = self.block_id.as_string();
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
	}
}

impl ir::Bind {
	fn display_ir(&self, type_store: &TypeStore) -> String {
		let type_str = type_store.get_display_type(self.type_id);
		let register = self.register.as_string();
		format!("{}: {}", register, type_str)
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
			ir::Instr::SetCache(set_cache) => set_cache.display_ir(text),
			ir::Instr::LoadCache(load_cache) => load_cache.display_ir(text),
		}
	}
}

impl ir::BinaryInstr {
	pub fn display_ir(&self, operator: &str, type_store: &TypeStore, text: &mut String) {
		let type_str = type_store.get_display_type(self.type_id);
		let lhs = self.lhs.as_string();
		let rhs = self.rhs.as_string();
		let dest = self.dest.as_string();
		let instr = format!("{} {} {} {} -> {}", operator, lhs, rhs, type_str, dest);
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::UnaryInstr {
	pub fn display_ir(&self, operator: &str, type_store: &TypeStore, text: &mut String) {
		let type_str = type_store.get_display_type(self.type_id);
		let value = self.value.as_string();
		let dest = self.dest.as_string();
		let instr = format!("{} {} {} -> {}", operator, type_str, value, dest);
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::JmpIfInstr {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let cond = self.cond.as_string();
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
		let instr = format!("goto {}", block_id);
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::FreeInstr {
	pub fn display_ir(&self, text: &mut String) {
		let register = self.register.as_string();
		let instr = format!("free {}", register);
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::OwnInstr {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let type_str = type_store.get_display_type(self.type_id);
		let value = self.value.as_string();
		let dest = self.dest.as_string();
		let instr = format!("own {} {} -> {}", type_str, value, dest);
		text.push_str(&instr);
		text.push('\n');
	}
}
impl ir::OwnHeapInstr {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let type_str = type_store.get_display_type(self.type_id);
		let value = self.value.as_string();
		let dest = self.dest.as_string();
		let instr = format!("own_heap {} {} -> {} size={}", type_str, value, dest, self.size);
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::HeapInstr {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let type_str = type_store.get_display_type(self.type_id);
		let value = self.value.as_string();
		let dest = self.dest.as_string();
		let instr = format!("heap {} {} -> {} size={}", type_str, value, dest, self.size);
		text.push_str(&instr);
		text.push('\n');
	}
}
impl ir::StoreInstr {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let type_str = type_store.get_display_type(self.type_id);
		let value = self.value.as_string();
		let dest = self.dest.as_string();
		let instr = format!("store {} {} -> {}", type_str, value, dest);
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::RetInstr {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let type_str = type_store.get_display_type(self.type_id);
		let instr = match &self.value {
			Some(value) => format!("ret {} -> {}", type_str, value.as_string()),
			None => format!("ret {}", type_str),
		};
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::CallInstr {
	pub fn display_ir(&self, type_store: &TypeStore, text: &mut String) {
		let type_str = type_store.get_display_type(self.type_id);
		let fn_id = self.fn_id.as_string();
		let dest = self.dest.as_string();
		let instr = format!("call {} {} -> {}", type_str, fn_id, dest);
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::SetCacheInstr {
	pub fn display_ir(&self, text: &mut String) {
		let register = self.register.as_string();
		let instr = format!("set_cache {}", register);
		text.push_str(&instr);
		text.push('\n');
	}
}

impl ir::LoadCacheInstr {
	pub fn display_ir(&self, text: &mut String) {
		let register = self.register.as_string();
		let instr = format!("load_cache {}", register);
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
			IrValue::String(string) => string.to_string(),
			IrValue::Char(char) => char.to_string(),
			IrValue::Reg(reg) => reg.as_string(),
			IrValue::Fn(fn_id) => fn_id.as_string().to_string(),
		}
	}
}
