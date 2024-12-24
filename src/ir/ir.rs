#![allow(dead_code)]

use crate::checker::types::TypeId;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Register(pub usize);
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Label(pub usize);
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bind {
	pub register: Register,
	pub type_id: TypeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeapValue {
	Char(char),
	String(String),
	Int(usize),
	Float(String),
	Bool(bool),
}

impl HeapValue {
	pub fn new_char(value: char) -> Self {
		Self::Char(value)
	}
	pub fn new_string(value: &str) -> Self {
		Self::String(value.to_owned())
	}
	pub fn new_int(value: &str) -> Self {
		let value = value.parse::<usize>().unwrap();
		Self::Int(value)
	}
	pub fn new_float(value: &str) -> Self {
		Self::Float(value.to_owned())
	}
	pub fn new_bool(value: bool) -> Self {
		Self::Bool(value)
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms, non_camel_case_types)]
pub enum Code {
	// add lhs, rhs -> dest
	ADD { lhs: Register, rhs: Register, dest: Register },
	// sub lhs, rhs -> dest
	SUB { lhs: Register, rhs: Register, dest: Register },
	// div lhs, rhs -> dest
	DIV { lhs: Register, rhs: Register, dest: Register },
	// mul lhs, rhs -> dest
	MUL { lhs: Register, rhs: Register, dest: Register },
	// mod lhs, rhs -> dest
	MOD { lhs: Register, rhs: Register, dest: Register },
	// cmp_gt lhs, rhs -> dest
	CMPGT { lhs: Register, rhs: Register, dest: Register },
	// cmp_eq lhs, rhs -> dest
	CMPEQ { lhs: Register, rhs: Register, dest: Register },
	// cmp_lt lhs, rhs -> dest
	CMPLT { lhs: Register, rhs: Register, dest: Register },
	// cmp_le lhs, rhs -> dest
	CMPLE { lhs: Register, rhs: Register, dest: Register },
	// jmp_if cond, l1, l0
	JMPIF { cond: Register, l0: Label, l1: Label },
	// goto label
	GOTO { to: Label },
	// own value -> dest
	OWN { value: Register, dest: Register },
	// borrow value -> dest
	BORROW { value: Register, dest: Register },
	// borrow_mut value -> dest
	BORROW_MUT { value: Register, dest: Register },
	// free value
	FREE { value: Register },
	// ret value;
	RET { value: Register },
	// call fn(args) -> dest;
	CALL { fn_id: FnId, args: Vec<Register>, dest: Register },

	// heap value -> dest
	HEAP { value: HeapValue, dest: Register },
	LOAD { value: Register, dest: Register },

	// arrys instuctions
	ARRAY_GET { array: Bind, index: Bind, dest: Bind },
	ARRAY_SET { array: Bind, index: Bind, value: Bind },

	// structs instructions
	STRUCT_GET { struct_id: Bind, field_id: Bind, dest: Bind },
	STRUCT_SET { struct_id: Bind, field_id: Bind, value: Bind },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Meta {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instr {
	pub meta: Meta, // metadata for optimization
	pub code: Code,
}

impl Instr {
	pub fn new(code: Code) -> Self {
		Self { meta: Meta {}, code }
	}

	pub fn new_heap(value: HeapValue, dest: Register) -> Self {
		Self::new(Code::HEAP { value, dest })
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternKind {
	pub abi: String, // e.g. C, Rust, SysV, WinApi, etc. (aka application binary interface)
	pub symbol: String, // e.g. add, sub, etc.
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputeKind {
	pub effect: bool, // true if the function has side effects
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FnKind {
	Native,
	Compute(ComputeKind),
	External(ExternKind),
}

impl FnKind {
	pub fn new_native() -> Self {
		Self::Native
	}
	pub fn new_compute(effect: bool) -> Self {
		Self::Compute(ComputeKind { effect })
	}
	pub fn new_external(abi: String, symbol: String) -> Self {
		Self::External(ExternKind { abi, symbol })
	}
	pub fn is_native(&self) -> bool {
		matches!(self, Self::Native)
	}
	pub fn is_compute(&self) -> bool {
		matches!(self, Self::Compute(_))
	}
	pub fn is_external(&self) -> bool {
		matches!(self, Self::External(_))
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fn {
	pub fn_id: FnId,
	pub fn_kind: FnKind,
	pub params: Vec<Bind>,
	pub ret_id: TypeId,
	pub body: Vec<Block>,
}

impl Fn {
	pub fn new(fn_id: FnId, params: Vec<Bind>, ret_id: TypeId) -> Self {
		let fn_kind = FnKind::new_native();
		Self { fn_id, fn_kind, params, ret_id, body: vec![] }
	}

	pub fn add_block(&mut self, block: Block) {
		self.body.push(block);
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
	pub label: Label,
	pub instrs: Vec<Instr>,
	pub preds: Vec<Label>, // predecessors
	pub succs: Vec<Label>, // successors
}

impl Block {
	pub fn new(label: Label) -> Self {
		Self { label, instrs: vec![], preds: vec![], succs: vec![] }
	}

	pub fn add_instr(&mut self, instr: Instr) {
		self.instrs.push(instr);
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ir {
	pub fns: Vec<Fn>,
	pub binds: Vec<Bind>, // global variables
}

impl Ir {
	pub fn new() -> Self {
		Self { fns: Vec::new(), binds: Vec::new() }
	}
	pub fn add_fn(&mut self, fn_ir: Fn) {
		self.fns.push(fn_ir);
	}

	pub fn add_bind(&mut self, bind: Bind) {
		self.binds.push(bind);
	}
}

impl Default for Ir {
	fn default() -> Self {
		Self::new()
	}
}

// // display
// impl std::fmt::Display for Ir {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		for fn_ir in self.fns.iter() {
// 			writeln!(f, "{}", fn_ir)?;
// 		}
// 		Ok(())
// 	}
// }

// impl std::fmt::Display for Fn {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		write!(f, "fn {}", self.fn_id)?;
// 		write!(f, "(")?;
// 		for (i, param) in self.params.iter().enumerate() {
// 			if i > 0 {
// 				write!(f, ", ")?;
// 			}
// 			write!(f, "{}", param)?;
// 		}
// 		write!(f, ")")?;
// 		write!(f, " -> ??")?;

// 		writeln!(f)?;
// 		for block in self.body.iter() {
// 			write!(f, "{}", block)?;
// 		}
// 		Ok(())
// 	}
// }

// impl std::fmt::Display for Block {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		for (i, instr) in self.instrs.iter().enumerate() {
// 			if i == 0 {
// 				writeln!(f, "l{}: {}", self.label.0, instr)?;
// 			} else {
// 				writeln!(f, "    {}", instr)?;
// 			}
// 		}
// 		Ok(())
// 	}
// }

// impl std::fmt::Display for Instr {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		match &self.code {
// 			Code::ADD { lhs, rhs, dest } => write!(f, "add {}, {} -> {}", lhs, rhs, dest),
// 			Code::SUB { lhs, rhs, dest } => write!(f, "sub {}, {} -> {}", lhs, rhs, dest),
// 			Code::DIV { lhs, rhs, dest } => write!(f, "div {}, {} -> {}", lhs, rhs, dest),
// 			Code::MUL { lhs, rhs, dest } => write!(f, "mul {}, {} -> {}", lhs, rhs, dest),
// 			Code::MOD { lhs, rhs, dest } => write!(f, "mod {}, {} -> {}", lhs, rhs, dest),
// 			Code::CMPGT { lhs, rhs, dest } => write!(f, "cmp_gt {}, {} -> {}", lhs, rhs, dest),
// 			Code::CMPEQ { lhs, rhs, dest } => write!(f, "cmp_eq {}, {} -> {}", lhs, rhs, dest),
// 			Code::CMPLT { lhs, rhs, dest } => write!(f, "cmp_lt {}, {} -> {}", lhs, rhs, dest),
// 			Code::CMPLE { lhs, rhs, dest } => write!(f, "cmp_le {}, {} -> {}", lhs, rhs, dest),
// 			Code::JMPIF { cond, l0, l1 } => write!(f, "jmp_if {}, {}, {}", cond, l0, l1),
// 			Code::GOTO { to } => write!(f, "goto {}", to),
// 			Code::OWN { value, dest } => write!(f, "own {} -> {}", value, dest),
// 			Code::BORROW { value, dest } => write!(f, "borrow {} -> {}", value, dest),
// 			Code::BORROW_MUT { value, dest } => write!(f, "borrow_mut {} -> {}", value, dest),
// 			Code::FREE { value } => write!(f, "free {}", value),
// 			Code::RET { value } => write!(f, "ret {}", value),
// 			Code::CALL { fn_id, args, dest } => {
// 				write!(
// 					f,
// 					"call {} {} -> {}",
// 					fn_id,
// 					args.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(", "),
// 					dest
// 				)
// 			}
// 			Code::HEAP { value, dest } => write!(f, "heap {} -> {}", value, dest),
// 			Code::LOAD { value, dest } => write!(f, "load {} -> {}", value, dest),
// 			_ => todo!(),
// 		}
// 	}
// }
// impl std::fmt::Display for Bind {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		write!(f, "{}: ??", self.register)
// 	}
// }

// impl std::fmt::Display for Register {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		write!(f, "r{}", self.0)
// 	}
// }
// impl std::fmt::Display for FnId {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		write!(f, "{}", self.0)
// 	}
// }
// impl std::fmt::Display for Label {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		write!(f, "l{}", self.0)
// 	}
// }

// impl std::fmt::Display for HeapValue {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		match self {
// 			HeapValue::Char(c) => write!(f, "{}", c),
// 			HeapValue::String(s) => write!(f, "{}", s),
// 			HeapValue::Int(i) => write!(f, "{}", i),
// 			HeapValue::Float(fl) => write!(f, "{}", fl),
// 			HeapValue::Bool(b) => write!(f, "{}", b),
// 		}
// 	}
// }
