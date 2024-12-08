use super::*;

impl fmt::Display for FnValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let params = self.params.iter().map(|p| p.to_string()).collect::<Vec<_>>().join(", ");
    if let Some(ret) = &self.ret_type {
      write!(f, "fn({}) -> {}", params, ret)
    } else {
      write!(f, "fn({})", params)
    }
  }
}

impl fmt::Display for NumbValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}{}",
      if self.signed { "i" } else { "u" },
      self.bits.map_or_else(|| "size".to_string(), |b| b.to_string())
    )
  }
}

impl fmt::Display for FloatValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "f{}", self.bits)
  }
}

impl fmt::Display for Type {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Type::Numb(num) => write!(f, "{}", num),
      Type::Float(floa) => write!(f, "{}", floa),
      Type::Bool => write!(f, "bool"),
      Type::Char => write!(f, "char"),
      Type::String => write!(f, "string"),
      Type::Fn(fn_type) => write!(f, "{}", fn_type),
      // _ => write!(f, "<unknown>"),
    }
  }
}
