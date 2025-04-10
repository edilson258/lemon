use core::fmt;

use rustc_hash::FxHashSet;

use super::tracker::PtrId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PtrKind {
	Owned,
	MutableBorrow,
	ReadOnlyBorrow,
	Copied,
}

impl PtrKind {
	pub fn is_mutable_borrow(&self) -> bool {
		matches!(self, PtrKind::MutableBorrow)
	}

	pub fn is_read_only_borrow(&self) -> bool {
		matches!(self, PtrKind::ReadOnlyBorrow)
	}
}

pub const MAX_ADDRESS_BY_PTR: usize = 4;
pub type Addresses = FxHashSet<usize>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ptr {
	pub id: PtrId,
	pub addresses: FxHashSet<usize>,
	pub kind: PtrKind,
}

impl Ptr {
	pub fn new(id: PtrId, address: usize, kind: PtrKind) -> Self {
		let addresses = FxHashSet::default();
		Self { id, addresses, kind }
	}

	pub fn new_owned(id: PtrId, address: usize) -> Self {
		let addresses = FxHashSet::default();
		Self { id, addresses, kind: PtrKind::Owned }
	}
	pub fn new_mutable_borrow(id: PtrId, address: usize) -> Self {
		let addresses = FxHashSet::default();
		Self { id, addresses, kind: PtrKind::MutableBorrow }
	}
	pub fn new_readonly_borrow(id: PtrId, address: usize) -> Self {
		let addresses = FxHashSet::default();
		Self { id, addresses, kind: PtrKind::ReadOnlyBorrow }
	}
	pub fn new_copied(id: PtrId, address: usize) -> Self {
		let addresses = FxHashSet::default();
		Self { id, addresses, kind: PtrKind::Copied }
	}
	pub fn new_addresses(id: PtrId, addresses: Addresses, kind: PtrKind) -> Self {
		Self { id, addresses, kind }
	}

	pub fn is_owned(&self) -> bool {
		matches!(self.kind, PtrKind::Owned)
	}
	pub fn is_mutable_borrow(&self) -> bool {
		matches!(self.kind, PtrKind::MutableBorrow)
	}
	pub fn is_read_only_borrow(&self) -> bool {
		matches!(self.kind, PtrKind::ReadOnlyBorrow)
	}
	pub fn is_copied(&self) -> bool {
		matches!(self.kind, PtrKind::Copied)
	}
}

impl fmt::Display for Ptr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "pointer '{}'  kind '{}'", self.id, self.kind)
	}
}

impl fmt::Display for PtrKind {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			PtrKind::Owned => write!(f, "owned"),
			PtrKind::MutableBorrow => write!(f, "mutable"),
			PtrKind::ReadOnlyBorrow => write!(f, "immutable"),
			PtrKind::Copied => write!(f, "copied"),
		}
	}
}
