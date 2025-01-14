#![allow(dead_code)]

use crate::checker::types::TypeId;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct BlockId(pub usize);
impl BlockId {
	pub fn new(id: usize) -> Self {
		Self(id)
	}
	pub fn as_usize(&self) -> usize {
		self.0
	}
	pub fn as_string(&self) -> String {
		format!("l{}", self.0)
	}
	pub fn next_block(&mut self) -> Self {
		Self(self.0 + 1)
	}
}

#[derive(Debug, Clone)]
pub struct FnId(pub String);
impl FnId {
	pub fn new(name: &str) -> Self {
		Self(name.to_owned())
	}
	pub fn as_string(&self) -> &str {
		&self.0
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Register(pub usize);
impl Register {
	pub fn new(id: usize) -> Self {
		Self(id)
	}
	pub fn as_usize(&self) -> usize {
		self.0
	}
	pub fn next_register(&mut self) -> Self {
		Self(self.0 + 1)
	}
	pub fn as_string(&self) -> String {
		format!("r{}", self.0)
	}
}

impl From<u64> for Register {
	fn from(value: u64) -> Self {
		Self(value as usize)
	}
}

impl From<Register> for u64 {
	fn from(value: Register) -> Self {
		value.0 as u64
	}
}

impl From<Register> for IrValue {
	fn from(value: Register) -> Self {
		IrValue::Reg(value)
	}
}

// == Intructions ==
#[derive(Debug, Clone)]
pub enum Instr {
	// Binary
	// add i32 lhs, rhs -> dest
	Add(BinaryInstr),
	// sub i32 lhs, rhs -> dest
	Sub(BinaryInstr),
	Div(BinaryInstr),
	Mul(BinaryInstr),
	Mod(BinaryInstr),
	CmpGt(BinaryInstr),
	CmpEq(BinaryInstr),
	CmpLt(BinaryInstr),
	CmpLe(BinaryInstr),
	CmpGe(BinaryInstr),
	// Control flow
	// jumpif lhs, l0, l1
	JmpIf(JmpIfInstr),
	Goto(GotoInstr),
	// Memory
	// load i32 value -> dest
	Load(UnaryInstr),
	// store i32 value -> dest
	Store(StoreInstr),
	// heap i32 value -> dest size=4
	Heap(HeapInstr),
	// free register
	Free(FreeInstr),
	// own_ i32 register -> dest
	Own(OwnInstr),
	// own_heap register -> dest
	OwnHeap(OwnHeapInstr),
	// borrow i32 register -> dest
	Borrow(UnaryInstr),
	// borrow_mut i32 register -> dest
	BorrowMut(UnaryInstr),
	// deref i32 register -> dest
	Deref(UnaryInstr),

	// set_cache i32 register -> dest
	SetCache(SetCacheInstr),
	// load_cache i32 register -> dest
	LoadCache(LoadCacheInstr),

	// Other
	// ret i32 register
	Ret(RetInstr),
	// call fn_id(args) -> dest
	Call(CallInstr),
}

// == structures ==
#[derive(Debug, Clone)]
pub struct BinaryInstr {
	pub type_id: TypeId,
	pub lhs: Register,
	pub rhs: Register,
	pub dest: Register,
}
impl BinaryInstr {
	pub fn new(type_id: TypeId, lhs: Register, rhs: Register, dest: Register) -> Self {
		Self { type_id, lhs, rhs, dest }
	}
}

#[derive(Debug, Clone)]
pub struct UnaryInstr {
	pub type_id: TypeId,
	pub value: Register,
	pub dest: Register,
}
impl UnaryInstr {
	pub fn new(type_id: TypeId, value: Register, dest: Register) -> Self {
		Self { type_id, value, dest }
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FreeInstr {
	pub register: Register,
}
impl FreeInstr {
	pub fn new(register: Register) -> Self {
		Self { register }
	}
}

impl From<FreeInstr> for Instr {
	fn from(free: FreeInstr) -> Self {
		Self::Free(free)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SetCacheInstr {
	pub register: Register,
}
impl SetCacheInstr {
	pub fn new(register: Register) -> Self {
		Self { register }
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LoadCacheInstr {
	pub register: Register,
}
impl LoadCacheInstr {
	pub fn new(register: Register) -> Self {
		Self { register }
	}
}

#[derive(Debug, Clone)]
pub struct RetInstr {
	pub value: Option<Register>,
	pub type_id: TypeId,
}

impl From<RetInstr> for Instr {
	fn from(ret: RetInstr) -> Self {
		Self::Ret(ret)
	}
}

impl RetInstr {
	pub fn new(type_id: Option<TypeId>, value: Option<Register>) -> Self {
		Self { type_id: type_id.unwrap_or(TypeId::UNIT), value }
	}
}

#[derive(Debug, Clone)]
pub struct JmpIfInstr {
	pub cond: Register,
	pub l0: BlockId,
	pub l1: BlockId,
}

impl From<JmpIfInstr> for Instr {
	fn from(jmp_if: JmpIfInstr) -> Self {
		Self::JmpIf(jmp_if)
	}
}

#[derive(Debug, Clone)]
pub struct CallInstr {
	pub type_id: TypeId,
	pub fn_id: FnId,
	pub args: Vec<Register>,
	pub dest: Register,
}

impl From<CallInstr> for Instr {
	fn from(call: CallInstr) -> Self {
		Self::Call(call)
	}
}

#[derive(Debug, Clone)]
pub struct GotoInstr {
	pub block_id: BlockId,
}

impl From<GotoInstr> for Instr {
	fn from(goto: GotoInstr) -> Self {
		Self::Goto(goto)
	}
}

#[derive(Debug, Clone)]
pub struct StoreInstr {
	pub type_id: TypeId,
	pub value: IrValue,
	pub dest: Register,
}

impl StoreInstr {
	pub fn new(value: IrValue, type_id: TypeId, dest: Register) -> Self {
		Self { value, type_id, dest }
	}
}

impl From<StoreInstr> for Instr {
	fn from(store: StoreInstr) -> Self {
		Self::Store(store)
	}
}
#[derive(Debug, Clone)]
pub struct HeapInstr {
	pub type_id: TypeId,
	pub value: IrValue,
	pub dest: Register,
	pub size: usize,
}

impl From<HeapInstr> for Instr {
	fn from(store: HeapInstr) -> Self {
		Self::Heap(store)
	}
}

#[derive(Debug, Clone)]
pub struct OwnInstr {
	pub type_id: TypeId,
	pub value: Register,
	pub dest: Register,
}

impl From<OwnInstr> for Instr {
	fn from(own_stack: OwnInstr) -> Self {
		Self::Own(own_stack)
	}
}

#[derive(Debug, Clone)]
pub struct OwnHeapInstr {
	pub type_id: TypeId,
	pub value: Register,
	pub dest: Register,
	pub size: usize,
}

impl From<OwnHeapInstr> for Instr {
	fn from(own_heap: OwnHeapInstr) -> Self {
		Self::OwnHeap(own_heap)
	}
}

// == ir values ==
#[derive(Debug, Clone)]
pub enum IrValue {
	Int(i64),
	Float(f64),
	Bool(bool),
	String(String),
	Char(char),
	Reg(Register),
	Fn(FnId),
}

impl IrValue {
	pub fn new_int(value: i64) -> Self {
		Self::Int(value)
	}
	pub fn new_float(value: f64) -> Self {
		Self::Float(value)
	}
	pub fn new_bool(value: bool) -> Self {
		Self::Bool(value)
	}
	pub fn new_string(value: &str) -> Self {
		Self::String(value.to_string())
	}
	pub fn new_char(value: char) -> Self {
		Self::Char(value)
	}
	pub fn new_fn(value: FnId) -> Self {
		Self::Fn(value)
	}
}

// == blocks ==
#[derive(Debug, Clone)]
pub struct Block {
	pub block_id: BlockId,
	pub instrs: Vec<Instr>,
	pub preds_ids: Vec<BlockId>,
	pub succs_ids: Vec<BlockId>,
}
impl Block {
	pub fn new(block_id: BlockId) -> Self {
		Self { block_id, instrs: vec![], preds_ids: vec![], succs_ids: vec![] }
	}
	pub fn add_instr(&mut self, instr: Instr) {
		self.instrs.push(instr);
	}
}

#[derive(Debug, Clone)]
pub struct LnFn {
	pub fn_id: FnId,
	pub args: Vec<Bind>,
	pub ret: TypeId,
	pub blocks: Vec<Block>,
}
impl LnFn {
	pub fn new(fn_id: FnId, args: Vec<Bind>, ret: TypeId) -> Self {
		Self { fn_id, args, ret, blocks: vec![] }
	}

	pub fn add_block(&mut self, block: Block) {
		self.blocks.push(block);
	}

	pub fn add_blocks(&mut self, blocks: Vec<Block>) {
		self.blocks.extend(blocks);
	}
}

#[derive(Debug, Clone)]
pub struct ExFn {
	pub fn_id: FnId,
	pub args: Vec<Bind>,
	pub ret: TypeId,
	pub var_packed: bool,
}

#[derive(Debug, Clone)]
pub enum Fn {
	Ln(LnFn),
	Ex(ExFn),
}

impl Fn {
	pub fn new_ln(fn_id: FnId, args: Vec<Bind>, ret: TypeId) -> Self {
		Self::Ln(LnFn { fn_id, args, ret, blocks: vec![] })
	}
	pub fn new_ex(fn_id: FnId, args: Vec<Bind>, ret: TypeId, var_packed: bool) -> Self {
		Self::Ex(ExFn { fn_id, args, ret, var_packed })
	}

	pub fn is_extern_fn(&self) -> bool {
		matches!(self, Self::Ex(_))
	}

	pub fn is_ln_fn(&self) -> bool {
		matches!(self, Self::Ln(_))
	}
}

#[derive(Debug, Clone)]
pub struct Bind {
	pub register: Register,
	pub type_id: TypeId,
}

#[derive(Debug, Clone)]
pub struct Root {
	reg_size: usize,
	pub fns: Vec<Fn>,
}

impl Default for Root {
	fn default() -> Self {
		Self::new()
	}
}

impl Root {
	pub fn new() -> Self {
		Self { fns: Vec::new(), reg_size: 0 }
	}
	pub fn add_fn(&mut self, fn_ir: Fn) {
		self.fns.push(fn_ir);
	}

	pub fn add_blocks(&mut self, blocks: Vec<Block>) {
		if let Some(Fn::Ln(fn_ir)) = self.fns.last_mut() {
			fn_ir.add_blocks(blocks);
		}
	}

	pub fn set_reg_size(&mut self, size: usize) {
		self.reg_size = size;
	}
	pub fn get_reg_size(&self) -> usize {
		self.reg_size
	}
}
