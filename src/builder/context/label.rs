#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Label {
	pub value: usize,
}

impl Label {
	pub fn new(value: usize) -> Self {
		Self { value }
	}

	pub fn increment(&self) -> Self {
		Label::new(self.value + 1)
	}
}

impl From<Label> for String {
	fn from(value: Label) -> Self {
		format!("l{}", value.value)
	}
}

impl From<usize> for Label {
	fn from(value: usize) -> Self {
		Self::new(value)
	}
}

impl From<Label> for usize {
	fn from(value: Label) -> Self {
		value.value
	}
}

impl Default for Label {
	fn default() -> Self {
		Self::new(1)
	}
}
