#![allow(dead_code)]

use std::collections::HashMap;
#[allow(clippy::upper_case_acronyms)]
pub type TYPE = String;
#[allow(clippy::upper_case_acronyms)]
pub type REG = String;
#[allow(clippy::upper_case_acronyms)]
pub type LABEL = String;

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Instr {
  // add lhs, rhs -> dest
  ADD { lhs: REG, rhs: REG, dest: REG },
  // sub lhs, rhs -> dest
  SUB { lhs: REG, rhs: REG, dest: REG },
  // div lhs, rhs -> dest
  DIV { lhs: REG, rhs: REG, dest: REG },
  // mul lhs, rhs -> dest
  MUL { lhs: REG, rhs: REG, dest: REG },
  // mod lhs, rhs -> dest
  MOD { lhs: REG, rhs: REG, dest: REG },
  // cmp_gt lhs, rhs -> dest
  CMPGT { lhs: REG, rhs: REG, dest: REG },
  // cmp_eq lhs, rhs -> dest
  CMPEQ { lhs: REG, rhs: REG, dest: REG },
  // cmp_lt lhs, rhs -> dest
  CMPLT { lhs: REG, rhs: REG, dest: REG },
  // cmp_le lhs, rhs -> dest
  CMPLE { lhs: REG, rhs: REG, dest: REG },
  // jmp_if cond, l1(true), l0(false)
  JMPIF { cond: REG, l1: REG, l0: REG },
  // goto t
  GOTO { to: REG },
  // own value -> dest
  OWN { value: REG, dest: REG },
  // free value
  FREE { value: REG },
  // ret value
  RET { value: REG },
  // call fn(args) -> dest
  CALL { name: REG, args: Vec<REG>, dest: REG },
}

impl Instr {
  pub fn add(lhs: REG, rhs: REG, dest: REG) -> Self {
    Instr::ADD { lhs, rhs, dest }
  }
  pub fn cmp_gt(lhs: REG, rhs: REG, dest: REG) -> Self {
    Instr::CMPGT { lhs, rhs, dest }
  }
  pub fn jmp_if(cond: REG, l1: String, l0: String) -> Self {
    Instr::JMPIF { cond, l1, l0 }
  }
  pub fn ret(value: REG) -> Self {
    Instr::RET { value }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockIr {
  pub id: usize,
  pub instrs: Vec<Instr>,
}

impl BlockIr {
  pub fn new(id: usize) -> Self {
    Self { id, instrs: vec![] }
  }

  pub fn add_instr(&mut self, instr: Instr) {
    self.instrs.push(instr);
  }

  pub fn label(&self) -> String {
    format!("l{}", self.id)
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bind {
  pub reg: REG,
  pub ty: TYPE,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FnIr {
  pub name: REG,
  pub params: Vec<Bind>,
  pub ret_ty: Option<TYPE>,
  pub body: Vec<BlockIr>,
}

impl FnIr {
  pub fn new(name: REG, params: Vec<Bind>, ret_ty: Option<TYPE>) -> Self {
    Self { name, params, ret_ty, body: vec![] }
  }

  pub fn add_block(&mut self, block: BlockIr) {
    self.body.push(block);
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IR {
  pub fns: HashMap<String, FnIr>,
}

impl IR {
  // pub fn new() -> Self {
  //   Self { fns: HashMap::new() }
  // }
  pub fn add_fn(&mut self, fn_ir: FnIr) {
    self.fns.insert(fn_ir.name.clone(), fn_ir);
  }
}
