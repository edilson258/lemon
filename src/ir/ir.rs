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
		// l0 or b0 (ake label_0 or block_0)
		format!("l{}", self.0)
	}

	pub fn next_id(&mut self) -> Self {
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
	pub fn as_string(&self) -> String {
		format!("r{}", self.0)
	}
}

// instr
#[derive(Debug, Clone)]
pub enum Instr {
	//
	// binary instr

	// add lhs, rhs -> dest
	Add(BinaryInstr),
	// sub lhs, rhs -> dest
	Sub(BinaryInstr),
	// div lhs, rhs -> dest
	Div(BinaryInstr),
	// mul lhs, rhs -> dest
	Mul(BinaryInstr),
	// mod lhs, rhs -> dest
	Mod(BinaryInstr),
	// cmp_gt lhs, rhs -> dest
	CmpGt(BinaryInstr),
	// cmp_eq lhs, rhs -> dest
	CmpEq(BinaryInstr),
	// cmp_lt lhs, rhs -> dest
	CmpLt(BinaryInstr),
	// cmp_le lhs, rhs -> dest
	CmpLe(BinaryInstr),
	// cmp_ge lhs, rhs -> dest
	CmpGe(BinaryInstr),

	//
	// unary instr

	//
	// control flow instr

	// jmp_if cond, l1, l0
	JmpIf(JmpIfInstr),

	//
	// memory instr

	// load value -> dest
	Load(UnaryInstr),
	// store value -> dest
	Store(UnaryInstr),
	// free value
	Free(UnaryInstr),
	// own value -> dest
	Own(OwnInstr),
	// borrow value -> dest
	Borrow(UnaryInstr),
	// borrow_mut value -> dest
	BorrowMut(UnaryInstr),

	//
	// other instr

	// goto label
	Goto(GotoInstr),
	// ret value;
	Ret(RetInstr),
	// call fn(args) -> dest;
	Call(CallInstr),
}

#[derive(Debug, Clone)]
pub struct RetInstr {
	pub value: Option<Register>,
	pub type_id: TypeId,
}

impl RetInstr {
	pub fn new(type_id: Option<TypeId>, value: Option<Register>) -> Self {
		Self { type_id: type_id.unwrap_or(TypeId::NOTHING), value }
	}
}

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

// control flow instr

#[derive(Debug, Clone)]
pub struct JmpIfInstr {
	// pub type_id: TypeId,
	pub cond: Register,
	pub l0: BlockId,
	pub l1: BlockId,
}

// memory instr

#[derive(Debug, Clone)]
pub struct CallInstr {
	pub type_id: TypeId,
	pub fn_id: FnId,
	pub args: Vec<Register>,
	pub dest: Register,
}

#[derive(Debug, Clone)]
pub struct GotoInstr {
	pub block_id: BlockId,
}

#[derive(Debug, Clone)]
pub struct OwnInstr {
	pub type_id: TypeId,
	pub value: Value,
	pub dest: Register,
}

// suport values like 10, 10.0, true, false, "hello", 'c'

#[derive(Debug, Clone)]
pub enum Value {
	Int(i64),
	Float(f64),
	Bool(bool),
	String(String),
	Char(char),
	Bind(Bind),
	Register(Register),
	Fn(FnId),
	Null,
}

impl Value {
	pub fn new_int(value: i64) -> Self {
		Self::Int(value)
	}
	pub fn new_float(value: f64) -> Self {
		Self::Float(value)
	}
	pub fn new_bool(value: bool) -> Self {
		Self::Bool(value)
	}
	pub fn new_string(value: String) -> Self {
		Self::String(value)
	}
	pub fn new_char(value: char) -> Self {
		Self::Char(value)
	}
	pub fn new_bind(value: Bind) -> Self {
		Self::Bind(value)
	}
	pub fn new_fn(value: FnId) -> Self {
		Self::Fn(value)
	}

	pub fn new_register(value: Register) -> Self {
		Self::Register(value)
	}

	pub fn get_value(&self) -> Option<&Value> {
		match self {
			Self::Bind(_) => None,
			Self::Fn(_) => None,
			_ => Some(self),
		}
	}

	pub fn get_register(&self) -> Option<Register> {
		match self {
			Self::Bind(bind) => Some(bind.register),
			Self::Register(reg) => Some(*reg),
			_ => None,
		}
	}
	pub fn get_type_id(&self) -> Option<TypeId> {
		match self {
			Self::Bind(bind) => Some(bind.type_id),
			_ => None,
		}
	}
	pub fn get_fn_id(&self) -> Option<FnId> {
		match self {
			Self::Fn(fn_id) => Some(fn_id.clone()),
			_ => None,
		}
	}
	pub fn is_register(&self) -> bool {
		matches!(self, Self::Register(_))
	}
	pub fn get_bind(&self) -> Option<&Bind> {
		match self {
			Self::Bind(bind) => Some(bind),
			_ => None,
		}
	}
}

// bind(reg and type_id)
#[derive(Debug, Clone)]
pub struct Bind {
	pub register: Register, // register
	pub type_id: TypeId,    // type_id
}

#[derive(Debug, Clone)]
pub struct Block {
	pub block_id: BlockId, // (aka label)
	pub instrs: Vec<Instr>,
	pub preds_ids: Vec<BlockId>, // predecessors
	pub succs_ids: Vec<BlockId>, // successors
}

impl Block {
	pub fn new(block_id: BlockId) -> Self {
		Self { block_id, instrs: vec![], preds_ids: vec![], succs_ids: vec![] }
	}

	pub fn add_instr(&mut self, instr: Instr) {
		self.instrs.push(instr);
	}
}

// fn
#[derive(Debug, Clone)]
pub struct FnNative {
	pub fn_id: FnId,
	pub params: Vec<Bind>,
	pub ret: TypeId,
	pub blocks: Vec<Block>,
}

impl FnNative {
	pub fn new(fn_id: FnId, params: Vec<Bind>, ret: TypeId) -> Self {
		Self { fn_id, params, ret, blocks: vec![] }
	}

	pub fn add_block(&mut self, block: Block) {
		self.blocks.push(block);
	}
	pub fn add_blocks(&mut self, blocks: Vec<Block>) {
		self.blocks.extend(blocks);
	}
}

// fn! (aka comptime fn)
#[derive(Debug, Clone)]
pub struct FnComptime {
	pub fn_id: FnId,
	pub max_call_depth: u32,
	pub effect: bool, // enum here?... also we will really support side effects? :(
	pub params: Vec<Bind>,
	pub ret: TypeId,
	pub blocks: Vec<Block>,
}

impl FnComptime {
	pub fn new(fn_id: FnId, params: Vec<Bind>, ret: TypeId) -> Self {
		Self { fn_id, max_call_depth: 0, effect: false, params, ret, blocks: vec![] }
	}

	pub fn add_block(&mut self, block: Block) {
		self.blocks.push(block);
	}
	pub fn add_blocks(&mut self, blocks: Vec<Block>) {
		self.blocks.extend(blocks);
	}
}

// extern fn (aka external fn aka abi fn)
#[derive(Debug, Clone)]
pub struct FnExtern {
	pub fn_id: FnId,
	pub abi: String,    // enum here?
	pub symbol: String, // enum here?
	pub params: Vec<Bind>,
	pub ret: TypeId,
	pub blocks: Vec<Block>,
}

impl FnExtern {
	pub fn new(fn_id: FnId, abi: String, symbol: String, params: Vec<Bind>, ret: TypeId) -> Self {
		Self { fn_id, abi, symbol, params, ret, blocks: vec![] }
	}

	pub fn add_block(&mut self, block: Block) {
		self.blocks.push(block);
	}
	pub fn add_blocks(&mut self, blocks: Vec<Block>) {
		self.blocks.extend(blocks);
	}
}

// we can join all fn's into one struct? hum i like this :)
#[derive(Debug, Clone)]
pub enum Fn {
	Native(FnNative),
	Comptime(FnComptime),
	Extern(FnExtern),
}
impl Fn {
	pub fn add_block(&mut self, block: Block) {
		match self {
			Self::Native(fn_native) => fn_native.add_block(block),
			Self::Comptime(fn_comptime) => fn_comptime.add_block(block),
			Self::Extern(fn_extern) => fn_extern.add_block(block),
		}
	}
	pub fn add_blocks(&mut self, blocks: Vec<Block>) {
		match self {
			Self::Native(fn_native) => fn_native.add_blocks(blocks),
			Self::Comptime(fn_comptime) => fn_comptime.add_blocks(blocks),
			Self::Extern(fn_extern) => fn_extern.add_blocks(blocks),
		}
	}
}

// global variables (aka static variables)
#[derive(Debug, Clone)]
pub struct Global {
	pub instrs: Vec<Instr>,
	pub fns: Vec<Fn>,
}
impl Global {
	pub fn new() -> Self {
		Self { instrs: vec![], fns: vec![] }
	}

	pub fn add_blocks(&mut self, blocks: Vec<Block>) {
		if let Some(fn_ir) = self.fns.last_mut() {
			fn_ir.add_blocks(blocks);
		}
	}
}

impl Default for Global {
	fn default() -> Self {
		Self::new()
	}
}

// root ir
#[derive(Debug, Clone)]
pub struct Root {
	pub fns: Vec<Fn>,
	pub globals: Global,
}

impl Root {
	pub fn new() -> Self {
		Self { fns: Vec::new(), globals: Global::new() }
	}
	pub fn add_fn(&mut self, fn_ir: Fn) {
		self.fns.push(fn_ir);
	}

	pub fn add_fn_global(&mut self, fn_ir: Fn) {
		self.globals.fns.push(fn_ir);
	}

	pub fn add_block(&mut self, block: Block) {
		if let Some(fn_ir) = self.fns.last_mut() {
			fn_ir.add_block(block);
		}
	}

	pub fn add_blocks(&mut self, blocks: Vec<Block>) {
		if let Some(fn_ir) = self.fns.last_mut() {
			fn_ir.add_blocks(blocks);
		}
	}
	pub fn add_global_blocks(&mut self, blocks: Vec<Block>) {
		self.globals.add_blocks(blocks);
	}
	pub fn add_global(&mut self, instr: Instr) {
		self.globals.instrs.push(instr);
	}
}

impl Default for Root {
	fn default() -> Self {
		Self::new()
	}
}
