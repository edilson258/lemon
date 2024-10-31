pub struct Source {
  pub raw: String,
  pub name: String,
}

impl Source {
  pub fn new(raw: &str, name: &str) -> Self {
    Self { raw: raw.to_string(), name: name.to_string() }
  }

  pub fn len(&self) -> usize {
    self.raw.len()
  }

  pub fn get_raw(&self) -> String {
    self.raw.clone()
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }
}
