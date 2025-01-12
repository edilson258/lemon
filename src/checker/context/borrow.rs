use super::value::ValueId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BorrowId(pub(crate) usize);

impl BorrowId {
	pub fn as_usize(&self) -> usize {
		self.0
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Borrow {
	pub id: BorrowId,
	pub is_mut: bool,
	pub value_id: ValueId,
}

impl Borrow {
	pub fn new(id: BorrowId, is_mut: bool, value_id: ValueId) -> Self {
		Self { id, is_mut, value_id }
	}

	pub fn new_mutable(id: BorrowId, value_id: ValueId) -> Self {
		Self { id, is_mut: true, value_id }
	}

	pub fn new_immutable(id: BorrowId, value_id: ValueId) -> Self {
		Self { id, is_mut: false, value_id }
	}

	pub fn is_mutable(&self) -> bool {
		self.is_mut
	}
}
#[derive(Debug, Clone)]
pub struct BorrowStore {
	borrows: Vec<Borrow>,
}

impl BorrowStore {
	pub fn new() -> Self {
		Self { borrows: Vec::new() }
	}
	pub fn add_borrow(&mut self, value_id: ValueId, is_mut: bool) -> BorrowId {
		let borrow_id = BorrowId(self.borrows.len());
		self.borrows.push(Borrow::new(borrow_id, is_mut, value_id));
		borrow_id
	}

	pub fn get_borrow(&self, borrow_id: BorrowId) -> Option<&Borrow> {
		self.borrows.get(borrow_id.as_usize())
	}

	pub fn drop_borrows(&mut self, borrow_id: BorrowId) {
		self.borrows.retain(|borrow| borrow.id != borrow_id);
	}
	pub fn can_borrow_as(&self, value_id: ValueId, is_mut: bool) -> bool {
		if let Some(borrow) = self.borrows.iter().find(|b| b.value_id == value_id) {
			if is_mut && borrow.is_mutable() {
				return false;
			}
		}
		true
	}
}

impl Default for BorrowStore {
	fn default() -> Self {
		Self::new()
	}
}
