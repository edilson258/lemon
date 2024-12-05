#![allow(dead_code)]

pub type TYPE = String;
pub type REG = String;

#[derive(Debug, Clone, PartialEq, Eq)]
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
  OWN { tag: REG, dest: REG },
  // free src
  FREE { tag: REG },
  // ret src
  RET { tag: REG },
  // call fn(args) -> dest
  CALL { name: REG, args: Vec<REG>, dest: REG },
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
  pub body: Vec<Instr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IR {
  pub fns: Vec<FnIr>,
}

// IR
impl std::fmt::Display for IR {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    writeln!(f)?; // TODO: add version
    writeln!(f, "# lemon intermediate representation")?;
    for fn_ir in &self.fns {
      write!(f, "{}", fn_ir)?;
    }
    Ok(())
  }
}

// Bind
impl std::fmt::Display for Bind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", self.reg, self.ty)
  }
}

// FnIr
impl std::fmt::Display for FnIr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "fn {}(", self.name)?;
    for (i, param) in self.params.iter().enumerate() {
      if i > 0 {
        write!(f, ", ")?;
      }
      write!(f, "{}", param)?;
    }
    if let Some(ty) = &self.ret_ty {
      write!(f, ") -> {}", ty)?;
    } else {
      write!(f, ")")?;
    }
    writeln!(f)?;
    for instr in self.body.iter() {
      writeln!(f, ".. {}", instr)?;
    }
    Ok(())
  }
}

//  instr
impl std::fmt::Display for Instr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Instr::ADD { lhs, rhs, dest } => {
        write!(f, "add {}, {} -> {}", lhs, rhs, dest)
      }
      Instr::SUB { lhs, rhs, dest } => {
        write!(f, "sub {}, {} -> {}", lhs, rhs, dest)
      }
      Instr::DIV { lhs, rhs, dest } => {
        write!(f, "div {}, {} -> {}", lhs, rhs, dest)
      }
      Instr::MUL { lhs, rhs, dest } => {
        write!(f, "mul {}, {} -> {}", lhs, rhs, dest)
      }
      Instr::MOD { lhs, rhs, dest } => {
        write!(f, "mod {}, {} -> {}", lhs, rhs, dest)
      }
      Instr::CMPGT { lhs, rhs, dest } => {
        write!(f, "cmp_gt {}, {} -> {}", lhs, rhs, dest)
      }
      Instr::CMPEQ { lhs, rhs, dest } => {
        write!(f, "cmp_eq {}, {} -> {}", lhs, rhs, dest)
      }
      Instr::CMPLT { lhs, rhs, dest } => {
        write!(f, "cmp_lt {}, {} -> {}", lhs, rhs, dest)
      }
      Instr::CMPLE { lhs, rhs, dest } => {
        write!(f, "cmp_le {}, {} -> {}", lhs, rhs, dest)
      }
      Instr::JMPIF { cond, l1, l0 } => {
        write!(f, "jmp_if {}, {} -> {}", cond, l1, l0)
      }
      Instr::OWN { tag, dest } => {
        write!(f, "own {} -> {}", tag, dest)
      }
      Instr::CALL { name, args, dest } => {
        write!(f, "call {} {} -> {}", name, args.join(", "), dest)
      }
      Instr::GOTO { to } => write!(f, "goto {}", to),
      Instr::FREE { tag } => write!(f, "free {}", tag),
      Instr::RET { tag } => write!(f, "ret {}", tag),
    }
  }
}
