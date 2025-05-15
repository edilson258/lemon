#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RefAccess {
	Owner,
	Mutable,
	Immutable,
	// `successor` is ative when mutable reference is dropped
	Successor,
	RawCopy,
}

impl RefAccess {
	pub fn is_owner(&self) -> bool {
		matches!(self, RefAccess::Owner)
	}
	pub fn is_mutable(&self) -> bool {
		matches!(self, RefAccess::Mutable | RefAccess::Successor)
	}
	pub fn is_immutable(&self) -> bool {
		matches!(self, RefAccess::Immutable | RefAccess::RawCopy)
	}
}

impl Default for RefAccess {
	fn default() -> Self {
		Self::Owner
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RefState {
	Alive,
	Drop,
}

impl RefState {
	pub fn is_alive(&self) -> bool {
		matches!(self, RefState::Alive)
	}

	pub fn is_droped(&self) -> bool {
		matches!(self, RefState::Drop)
	}
}

impl Default for RefState {
	fn default() -> Self {
		Self::Alive
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RefId(pub usize);

impl RefId {
	pub fn new(id: usize) -> Self {
		Self(id)
	}

	pub fn as_string(&self) -> String {
		format!("r{}", self.0)
	}
}
impl std::fmt::Display for RefId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "r{}", self.0)
	}
}

#[derive(Debug, Clone)]
pub enum RefOrigin {
	Local,
	External,
}

impl Default for RefOrigin {
	fn default() -> Self {
		Self::Local
	}
}

impl RefOrigin {
	pub fn is_local(&self) -> bool {
		matches!(self, RefOrigin::Local)
	}
	pub fn is_external(&self) -> bool {
		matches!(self, RefOrigin::External)
	}
}

#[derive(Debug, Clone)]
pub struct RefData {
	pub id: RefId,
	pub access: RefAccess,
	pub state: RefState,
	pub origin: RefOrigin,
}

impl RefData {
	pub fn new(id: RefId, access: RefAccess) -> Self {
		Self { id, access, state: RefState::default(), origin: RefOrigin::External }
	}

	pub fn new_local(id: RefId, access: RefAccess) -> Self {
		Self { id, access, state: RefState::default(), origin: RefOrigin::Local }
	}

	pub fn new_owned(id: RefId) -> Self {
		RefData::new(id, RefAccess::Owner)
	}

	pub fn new_mut(id: RefId) -> Self {
		RefData::new(id, RefAccess::Mutable)
	}

	pub fn new_immutable(id: RefId) -> Self {
		RefData::new(id, RefAccess::Immutable)
	}

	pub fn new_successor(id: RefId) -> Self {
		RefData::new(id, RefAccess::Successor)
	}

	pub fn new_raw_copy(id: RefId) -> Self {
		RefData::new(id, RefAccess::RawCopy)
	}
}
