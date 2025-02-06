use crate::checker::types::TypeId;

pub struct IrBind {
	pub name: String,
	pub kind: TypeId,
}

impl IrBind {
	pub fn new(name: String, kind: TypeId) -> Self {
		Self { name, kind }
	}
}

pub struct IrValue {
	pub value: String,
	pub kind: TypeId,
}

impl IrValue {
	pub fn new(value: String, kind: TypeId) -> Self {
		Self { value, kind }
	}

	pub fn with_new_type(&self, kind: TypeId) -> Self {
		let value = self.value.clone();
		Self { value, kind }
	}
}

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

	Ret(IrValue),    // ret r1
	Call(CallInstr), // r0 = call fn_name, r1, r2, ...

	// memory
	Load(BinInstr), // r0 = load r1
	Mov(UnInstr),   // mov r0, r1 # only stack
	Drop(IrValue),  // drop r1 # only heap alloc
	Set(UnInstr),   // set r0, r1 # only heap alloc

	Salloc(SallocInstr), // r0 = salloc r1 # only stack
	Halloc(UnInstr),     // r0 = halloc r1 # only heap
}

pub struct SallocInstr {
	pub dest: IrValue,
	pub size: TypeId,
}

impl SallocInstr {
	pub fn new(dest: IrValue, size: TypeId) -> Self {
		Self { dest, size }
	}
}

impl From<SallocInstr> for Instr {
	fn from(value: SallocInstr) -> Self {
		Instr::Salloc(value)
	}
}

pub struct BinInstr {
	pub dest: IrValue,
	pub left: IrValue,
	pub right: IrValue,
}

impl BinInstr {
	pub fn new(dest: IrValue, left: IrValue, right: IrValue) -> Self {
		Self { dest, left, right }
	}
}

pub struct UnInstr {
	pub dest: IrValue,
	pub src: IrValue,
}

impl UnInstr {
	pub fn new(dest: IrValue, src: IrValue) -> Self {
		Self { dest, src }
	}
}

pub struct JmpInstr {
	pub label: String,
}

impl JmpInstr {
	pub fn new(label: String) -> Self {
		Self { label }
	}
}

pub struct JmpIfInstr {
	pub cond: IrValue,
	pub true_label: String,
	pub false_label: String,
}

impl JmpIfInstr {
	pub fn new(cond: IrValue, true_label: String, false_label: String) -> Self {
		Self { cond, true_label, false_label }
	}
}

pub struct CallInstr {
	pub dest: IrValue,
	pub callee: String,
	pub args: Vec<IrValue>,
}

impl CallInstr {
	pub fn new(dest: IrValue, callee: String, args: Vec<IrValue>) -> Self {
		Self { dest, callee, args }
	}
}

pub struct IrBlock {
	pub label: usize,
	pub instrs: Vec<Instr>,
}

impl Default for IrBlock {
	fn default() -> Self {
		Self::new(0)
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
}

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

	pub fn add_block(&mut self, block: IrBlock) {
		self.blocks.push(block);
	}

	pub fn extend_blocks(&mut self, blocks: Vec<IrBlock>) {
		self.blocks.extend(blocks);
	}
}

pub struct IR {
	pub funcs: Vec<Function>,
}

impl Default for IR {
	fn default() -> Self {
		Self::new()
	}
}

impl IR {
	pub fn new() -> Self {
		Self { funcs: Vec::new() }
	}

	pub fn add_func(&mut self, func: Function) {
		self.funcs.push(func);
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
