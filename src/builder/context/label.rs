#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Label {
	pub id: usize,
}

impl Label {
	pub fn new(id: usize) -> Self {
		Self { id }
	}

	pub fn next(self) -> Self {
		Self::new(self.id + 1)
	}
}

impl Default for Label {
	fn default() -> Self {
		Self::new(1) // always start at 1
	}
}

impl From<Label> for String {
	fn from(label: Label) -> Self {
		format!("l{}", label.id)
	}
}

impl From<usize> for Label {
	fn from(id: usize) -> Self {
		Self::new(id)
	}
}

impl From<Label> for usize {
	fn from(label: Label) -> Self {
		label.id
	}
}
