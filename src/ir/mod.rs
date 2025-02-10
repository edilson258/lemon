#![allow(dead_code)]
mod value;
use crate::checker::types::TypeId;
pub use value::*;

#[derive(Debug)]
pub struct IrBind {
	pub name: String,
	pub kind: TypeId,
}

impl IrBind {
	pub fn new(name: String, kind: TypeId) -> Self {
		Self { name, kind }
	}
}

#[derive(Debug)]
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

	Ret(IrBasicValue), // ret r1
	Call(CallInstr),   // r0 = call fn_name, r1, r2, ...

	// memory
	Load(UnInstr),       // r0 = load r1
	Mov(UnInstr),        // mov r0, r1 # only stack
	Drop(IrBasicValue),  // drop r1 # only heap alloc
	Set(UnInstr),        // set r0, r1 # only heap alloc
	Salloc(SallocInstr), // r0 = salloc r1 # only stack
	Halloc(UnInstr),     // r0 = halloc r1 # only heap
}

#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
pub struct UnInstr {
	pub dest: IrBasicValue,
	pub src: IrBasicValue,
}

impl UnInstr {
	pub fn new(dest: IrBasicValue, src: IrBasicValue) -> Self {
		Self { dest, src }
	}
}
#[derive(Debug)]
pub struct JmpInstr {
	pub label: String,
}

impl JmpInstr {
	pub fn new(label: String) -> Self {
		Self { label }
	}
}
#[derive(Debug)]
pub struct JmpIfInstr {
	pub cond: IrBasicValue,
	pub true_label: String,
	pub false_label: String,
}

impl JmpIfInstr {
	pub fn new(cond: IrBasicValue, true_label: String, false_label: String) -> Self {
		Self { cond, true_label, false_label }
	}
}
#[derive(Debug)]
pub struct CallInstr {
	pub dest: IrBasicValue,
	pub callee: String,
	pub args: Vec<IrBasicValue>,
}

impl CallInstr {
	pub fn new(dest: IrBasicValue, callee: String, args: Vec<IrBasicValue>) -> Self {
		Self { dest, callee, args }
	}
}

#[derive(Debug)]
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

	pub fn add_instr(&mut self, instr: Instr) {
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
		format!("l{}", self.label)
	}
}
#[derive(Debug)]
pub struct Function {
	pub name: String,
	pub comptime: bool,
	pub ret: TypeId,
	pub args: Vec<IrBind>,
	pub blocks: Vec<IrBlock>,
}

impl Function {
	pub fn new(name: String, comptime: bool, args: Vec<IrBind>, ret: TypeId) -> Self {
		Self { name, comptime, args, blocks: Vec::new(), ret }
	}

	pub fn is_main(&self) -> bool {
		self.name == "main"
	}

	pub fn is_packed(&self) -> bool {
		false
	}

	pub fn add_block(&mut self, block: IrBlock) {
		self.blocks.push(block);
	}

	pub fn extend_blocks(&mut self, blocks: Vec<IrBlock>) {
		self.blocks.extend(blocks);
	}
}

pub struct IR {
	pub functions: Vec<Function>,
}

impl Default for IR {
	fn default() -> Self {
		Self::new()
	}
}

impl IR {
	pub fn new() -> Self {
		Self { functions: Vec::new() }
	}

	pub fn add_function(&mut self, function: Function) {
		self.functions.push(function);
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
