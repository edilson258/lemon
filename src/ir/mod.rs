#![allow(dead_code)]
mod value;
use std::ops::{Rem, Sub};

use crate::checker::types::TypeId;
pub use value::*;

#[derive(Debug, Clone)]
pub struct IrBind {
	pub name: String,
	pub kind: TypeId,
}

impl IrBind {
	pub fn new(name: String, kind: TypeId) -> Self {
		Self { name, kind }
	}
}

#[derive(Debug, Clone)]
pub enum Instr {
	//
	Add(BinInstr), // r0 = add r1, r2
	Sub(BinInstr), // r0 = sub r1, r2
	Mul(BinInstr), // r0 = mul r1, r2
	Div(BinInstr), // r0 = div r1, r2
	Mod(BinInstr), // r0 = mod r1, r2

	Neg(BinInstr), // r0 = neg r1
	Not(BinInstr), // r0 = not r1

	CmpEq(BinInstr), // r0 = cmp_eq r1, r2
	CmpNe(BinInstr), // r0 = cmp_ne r1, r2
	CmpLt(BinInstr), // r0 = cmp_lt r1, r2
	CmpGt(BinInstr), // r0 = cmp_gt r1, r2
	CmpLe(BinInstr), // r0 = cmp_le r1, r2
	CmpGe(BinInstr), // r0 = cmp_ge r1, r2

	And(BinInstr), // r0 = and r1, r2
	Or(BinInstr),  // r0 = or r1, r2
	Shl(BinInstr), // r0 = shl r1, r2
	Shr(BinInstr), // r0 = shr r1, r2

	Jmp(JmpInstr),     // jmp l1
	JmpIf(JmpIfInstr), // jmp_if r1, l1, l2

	Ret(Option<IrBasicValue>), // ret r1
	Call(CallInstr),           // r0 = call fn_name, r1, r2, ...

	// memory
	Load(UnInstr),       // r0 = load r1
	Mov(UnInstr),        // mov r0, r1 # only stack
	Drop(IrBasicValue),  // drop r1 # only heap alloc
	Set(UnInstr),        // set r0, r1 # only heap alloc
	Salloc(SallocInstr), // r0 = salloc r1 # only stack
	Halloc(UnInstr),     // r0 = halloc r1 # only heap

	// heap pointer
	Getptr(GetPtrInstr), // r0 = Person getptr r0, 0
}

#[derive(Debug, Clone)]
pub struct GetPtrInstr {
	pub dest: IrBasicValue,
	pub self_base: IrBasicValue,
	pub self_name: String,
	pub offset: usize,
}

impl GetPtrInstr {
	pub fn new(self_name: String, self_ptr: IrBasicValue, offset: usize, dest: IrBasicValue) -> Self {
		Self { dest, self_base: self_ptr, self_name, offset }
	}
}

impl From<GetPtrInstr> for Instr {
	fn from(value: GetPtrInstr) -> Self {
		Instr::Getptr(value)
	}
}

#[derive(Debug, Clone)]
pub struct SallocInstr {
	pub dest: IrBasicValue,
	pub size: TypeId,
}

impl SallocInstr {
	pub fn new(dest: IrBasicValue, size: TypeId) -> Self {
		Self { dest, size }
	}
}

impl From<SallocInstr> for Instr {
	fn from(value: SallocInstr) -> Self {
		Instr::Salloc(value)
	}
}
#[derive(Debug, Clone)]
pub struct BinInstr {
	pub dest: IrBasicValue,
	pub left: IrBasicValue,
	pub right: IrBasicValue,
}

impl BinInstr {
	pub fn new(dest: IrBasicValue, left: IrBasicValue, right: IrBasicValue) -> Self {
		Self { dest, left, right }
	}
}
#[derive(Debug, Clone)]
pub struct UnInstr {
	pub dest: IrBasicValue,
	pub src: IrBasicValue,
}

impl UnInstr {
	pub fn new(dest: IrBasicValue, src: IrBasicValue) -> Self {
		Self { dest, src }
	}
}
#[derive(Debug, Clone)]
pub struct JmpInstr {
	pub label: usize,
}

impl JmpInstr {
	pub fn new(label: usize) -> Self {
		Self { label }
	}

	pub fn llvm_label(&self) -> String {
		format!("lx{}", self.label)
	}

	pub fn display_label(&self) -> String {
		format!("l_{}", self.label)
	}
}
#[derive(Debug, Clone)]
pub struct JmpIfInstr {
	pub cond: IrBasicValue,
	pub true_label: usize,
	pub false_label: usize,
}

impl JmpIfInstr {
	pub fn new(cond: IrBasicValue, true_label: usize, false_label: usize) -> Self {
		Self { cond, true_label, false_label }
	}

	pub fn display_true_label(&self) -> String {
		format!("l_{}", self.true_label)
	}

	pub fn display_false_label(&self) -> String {
		if self.false_label == 1 {
			return "entry".to_string();
		}
		format!("l_{}", self.false_label)
	}

	pub fn llvm_true_label(&self) -> String {
		if self.true_label == 1 {
			return "entry".to_string();
		}
		format!("lx{}", self.true_label)
	}

	pub fn llvm_false_label(&self) -> String {
		format!("lx{}", self.false_label)
	}
}
#[derive(Debug, Clone)]
pub struct CallInstr {
	pub dest: IrBasicValue,
	pub callee: String,
	pub ret_id: TypeId,
	pub args: Vec<IrBasicValue>,
}

impl CallInstr {
	pub fn new(dest: IrBasicValue, callee: String, ret_id: TypeId, args: Vec<IrBasicValue>) -> Self {
		Self { dest, callee, ret_id, args }
	}
}

#[derive(Debug, Clone)]
pub struct IrBlock {
	pub label: usize,
	pub instrs: Vec<Instr>,
}

impl Default for IrBlock {
	fn default() -> Self {
		Self::new(1)
	}
}
impl IrBlock {
	pub fn new(label: usize) -> Self {
		Self { label, instrs: Vec::new() }
	}

	pub fn append_instr(&mut self, instr: Instr) {
		self.instrs.push(instr);
	}

	pub fn format_label(&self) -> String {
		format!("l{}", self.label)
	}

	pub fn llvm_name(&self) -> String {
		// blcoks start from 1
		if self.label == 1 {
			return "entry".to_string();
		}
		format!("lx{}", self.label)
	}
}
#[derive(Debug, Clone)]
pub struct Function {
	pub extern_function: bool,
	pub name: String,
	pub comptime: bool,
	// only for extern fn
	pub variadic_args: bool,
	pub ret: TypeId,
	pub args: Vec<IrBasicValue>,
	pub blocks: Vec<IrBlock>,
}

impl Function {
	pub fn new(name: String, comptime: bool, args: Vec<IrBasicValue>, ret: TypeId) -> Self {
		Self {
			name,
			comptime,
			args,
			blocks: Vec::new(),
			ret,
			variadic_args: false,
			extern_function: false,
		}
	}

	pub fn is_main(&self) -> bool {
		self.name == "main"
	}

	pub fn is_variadic_args(&self) -> bool {
		self.variadic_args
	}

	pub fn is_extern_function(&self) -> bool {
		self.extern_function
	}

	pub fn as_extern_function(&mut self, variadic_args: bool) {
		self.extern_function = true;
		self.variadic_args = variadic_args;
	}

	pub fn add_block(&mut self, block: IrBlock) {
		self.blocks.push(block);
	}

	pub fn extend_blocks(&mut self, blocks: Vec<IrBlock>) {
		self.blocks.extend(blocks);
	}
}

pub struct Struct {
	pub fields: Vec<TypeId>,
	pub name: Option<String>,
	pub size: usize,
}

impl Struct {
	pub fn new(fields: Vec<TypeId>) -> Self {
		Self { fields, name: None, size: 0 }
	}

	pub fn new_with_name(fields: Vec<TypeId>, name: impl Into<String>) -> Self {
		Self { fields, name: Some(name.into()), size: 0 }
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Self { fields: Vec::with_capacity(capacity), name: None, size: 0 }
	}

	pub fn set_name(&mut self, name: impl Into<String>) {
		self.name = Some(name.into());
	}

	pub fn add_field(&mut self, field: TypeId) -> usize {
		let field_size = field.get_size();
		let field_align = field.get_align();
		if self.size.rem(field_align).ne(&0) {
			self.size += field_align.sub(self.size.rem(field_align));
		}
		let index = self.fields.len();
		self.fields.push(field);
		self.size += field_size;
		index
	}

	pub fn lazy_size(&mut self) {
		let max_align = self.fields.iter().map(|field| field.get_align()).max().unwrap_or(1);
		if self.size.rem(max_align).ne(&0) {
			self.size += max_align.sub(self.size.rem(max_align));
		}
	}

	pub fn get_fields(&self) -> &[TypeId] {
		&self.fields
	}

	pub fn get_field(&mut self, index: usize) -> Option<&mut TypeId> {
		self.fields.get_mut(index)
	}
}

impl Default for Struct {
	fn default() -> Self {
		Self::new(Vec::new())
	}
}

pub struct IR {
	pub pathnname: String,
	pub functions: Vec<Function>,
	pub structs: Vec<Struct>,
}

impl Default for IR {
	fn default() -> Self {
		Self::new("untitled.ln".to_string())
	}
}

impl IR {
	pub fn new(pathnname: String) -> Self {
		Self { pathnname, functions: Vec::new(), structs: Vec::new() }
	}

	pub fn add_function(&mut self, function: Function) {
		self.functions.push(function);
	}

	pub fn add_struct(&mut self, struct_: Struct) {
		self.structs.push(struct_);
	}
}

impl From<CallInstr> for Instr {
	fn from(value: CallInstr) -> Self {
		Instr::Call(value)
	}
}

impl From<JmpIfInstr> for Instr {
	fn from(value: JmpIfInstr) -> Self {
		Instr::JmpIf(value)
	}
}

impl From<JmpInstr> for Instr {
	fn from(value: JmpInstr) -> Self {
		Instr::Jmp(value)
	}
}
