use super::{Bind, BlockIr, FnIr, Instr, IR};

// IR
impl std::fmt::Display for IR {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (_name, fn_ir) in self.fns.iter() {
      writeln!(f, "{}", fn_ir)?;
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
      write!(f, " {}", instr)?;
    }
    Ok(())
  }
}

impl std::fmt::Display for BlockIr {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (index, instr) in self.instrs.iter().enumerate() {
      if index == 0 {
        writeln!(f, "l{}: {}", self.id, instr)?;
      } else {
        writeln!(f, "     {}", instr)?;
      }
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
      Instr::OWN { value, dest } => {
        write!(f, "own {} -> {}", value, dest)
      }
      Instr::CALL { name, args, dest } => {
        write!(f, "call {} {} -> {}", name, args.join(", "), dest)
      }
      Instr::GOTO { to } => write!(f, "goto {}", to),
      Instr::FREE { value } => write!(f, "free {}", value),
      Instr::RET { value } => write!(f, "ret {}", value),
    }
  }
}
