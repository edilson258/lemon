use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ModId(u64);

impl ModId {
	pub fn new(id: u64) -> Self {
		Self(id)
	}
}

impl Default for ModId {
	fn default() -> Self {
		Self(u64::MAX)
	}
}

impl Display for ModId {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "ModId({})", self.0)
	}
}

impl From<u64> for ModId {
	fn from(value: u64) -> Self {
		Self(value)
	}
}

impl From<usize> for ModId {
	fn from(value: usize) -> Self {
		Self(value as u64)
	}
}

impl From<ModId> for usize {
	fn from(value: ModId) -> Self {
		value.0 as usize
	}
}

impl From<ModId> for u64 {
	fn from(value: ModId) -> Self {
		value.0
	}
}
