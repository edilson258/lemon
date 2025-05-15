use super::arena::Arena;
use super::error;
use super::ptr::{RefAccess, RefData, RefId, RefState};
use crate::checker::context::scope::Scope;
use crate::checker::typed_value::{RefSource, TypedValue};
use crate::error_ownership;
use crate::message::MessageResult;
use rustc_hash::{FxHashMap, FxHashSet};
use std::fmt::Write;

pub type BorrowTracker = FxHashMap<RefId, FxHashSet<RefId>>;

#[derive(Debug)]
pub struct BorrowChecker {
	pub arena: Arena<RefData>,
	pub tracker: BorrowTracker,
}

impl BorrowChecker {
	pub fn new() -> Self {
		Self { arena: Arena::new(), tracker: FxHashMap::default() }
	}

	pub fn create_ref(&mut self, access: RefAccess) -> RefId {
		let id = RefId(self.arena.len());
		let data = RefData::new_local(id, access);
		self.arena.insert(data)
	}

	pub fn create_owner(&mut self) -> RefId {
		let id = RefId(self.arena.len());
		let data = RefData::new(id, RefAccess::Owner);
		self.arena.insert(data)
	}

	pub fn create_local_owner(&mut self) -> RefId {
		let id = RefId(self.arena.len());
		let data = RefData::new_local(id, RefAccess::Owner);
		self.arena.insert(data)
	}

	pub fn borrow_mutable(&mut self, value: &mut TypedValue) -> MessageResult<RefId> {
		self.can_borrow_mutable(value)?;
		let new_id = self.create_ref(RefAccess::Mutable);
		for owner in value.source.iter() {
			self.tracker.entry(owner).or_default().insert(new_id);
		}
		Ok(new_id)
	}

	pub fn create_raw_copy(&mut self) -> RefId {
		let id = RefId(self.arena.len());
		let data = RefData::new(id, RefAccess::RawCopy);
		self.arena.insert(data)
	}

	pub fn borrow_immutable(&mut self, value: &mut TypedValue) -> MessageResult<RefId> {
		self.can_borrow_immutable(value)?;
		let new_id = self.create_ref(RefAccess::Immutable);
		for owner in value.source.iter() {
			self.tracker.entry(owner).or_default().insert(new_id);
		}
		Ok(new_id)
	}
	pub fn borrow_owner(&mut self, value: &mut TypedValue) -> MessageResult<RefId> {
		for base in value.source.iter() {
			let Some(owner_ref) = self.arena.get(base) else {
				return Err(error_ownership!("cannot move '{}': dropped", base));
			};

			if !owner_ref.access.is_owner() || owner_ref.state.is_droped() {
				return Err(error_ownership!("cannot move '{}': dropped", base));
			}
		}
		for base in value.source.iter() {
			self.arena[base].state = RefState::Drop;
			self.tracker.remove(&base);
		}

		let new_owner = self.create_owner();
		value.source = RefSource::Single(new_owner);

		Ok(new_owner)
	}

	// pub fn release(&mut self, source: &RefSource) {
	// 	for ref_id in source.iter() {
	// 		let ref_data = &self.arena[ref_id];
	// 		if !ref_data.access.is_owner() {
	// 			self.arena[ref_id].state = RefState::Drop;
	// 		}
	// 		self.tracker.remove(&ref_id);
	// 	}
	// }
	pub fn release(&mut self, source: &RefSource) {
		for ref_id in source.iter() {
			let ref_data = &self.arena[ref_id];
			let should_drop = match ref_data.access {
				RefAccess::Owner => ref_data.origin.is_local(),
				_ => true,
			};
			if should_drop {
				self.arena[ref_id].state = RefState::Drop;
			}
			self.tracker.remove(&ref_id);
		}
	}

	pub fn release_all_from_scope(&mut self, scope: &Scope) {
		for value in scope.variables.values() {
			self.release(&value.typed_value.source);
		}
	}

	// pub fn check_return_value(&self, value: &TypedValue, range: Range) -> MessageResult<()> {
	// 	if !self.can_return_value(value) {
	// 		return Err(error::cannot_return_local_reference(range));
	// 	}
	// 	Ok(())
	// }

	// helpers
	//

	pub fn can_borrow_mutable(&self, value: &TypedValue) -> MessageResult<()> {
		for base in value.source.iter() {
			let Some(owner_ref) = self.arena.get(base) else {
				return Err(error::mutable_while_droped(base.as_string()));
			};

			if owner_ref.state.is_droped() || !owner_ref.access.is_owner() {
				return Err(error::mutable_while_droped(base.as_string()));
			}

			for alive_borrower in self.lookup_alive_borrowers(&value.source) {
				let ref_data = &self.arena[alive_borrower];
				if ref_data.access.is_immutable() {
					return Err(error::mutable_while_immutable_exists(base.as_string()));
				}
				if ref_data.access.is_mutable() {
					return Err(error::mutable_more_than_once(base.as_string()));
				}
			}
		}
		Ok(())
	}

	pub fn can_borrow_immutable(&mut self, value: &TypedValue) -> MessageResult<()> {
		let debug = self.dump_tracker_state();
		println!("{}", debug);
		if self.lookup_alive_borrowers(&value.source).any(|id| self.arena[id].access.is_mutable()) {
			let owner_name = value.source.as_string();
			let message = error::immutable_while_mutable_exists(owner_name);
			return Err(message);
		}
		Ok(())
	}

	pub fn can_borrow_owner(&self, value: &TypedValue) -> MessageResult<()> {
		for owner in value.source.iter() {
			let Some(data) = self.arena.get(owner) else {
				return Err(error_ownership!("cannot move '{}': dropped", owner.as_string()));
			};

			if data.state.is_droped() || !data.access.is_owner() {
				return Err(error_ownership!("cannot move '{}': dropped", owner.as_string()));
			}
		}
		Ok(())
	}
	pub fn can_return_value(&self, value: &TypedValue) -> bool {
		for ref_id in value.source.iter() {
			let ref_data = &self.arena[ref_id];
			if ref_data.origin.is_external() {
				return false;
			}
		}
		true
	}

	pub fn lookup_alive_borrowers<'a>(
		&'a self,
		source: &'a RefSource,
	) -> impl Iterator<Item = RefId> + 'a {
		source
			.iter()
			.flat_map(|owner| self.tracker.get(&owner).into_iter().flat_map(|refs| refs.iter().copied()))
			.filter(move |id| self.arena[*id].state.is_alive())
	}

	// debug propose
	pub fn dump_tracker_state(&self) -> String {
		let mut out = String::new();
		writeln!(out, "=== BorrowChecker State ===").unwrap();
		writeln!(out, "-- Arena --").unwrap();
		for (id, ref_data) in self.arena.iter() {
			writeln!(out, "{} => {:?} | {:?}", id, ref_data.access, ref_data.state).unwrap();
		}

		writeln!(out, "\n-- Tracker --").unwrap();
		for (owner, borrowers) in &self.tracker {
			writeln!(out, "{} ->", owner).unwrap();
			for borrower in borrowers {
				let ref_data = &self.arena[*borrower];
				writeln!(out, "    {}: {:?} | {:?}", borrower, ref_data.access, ref_data.state).unwrap();
			}
		}

		out
	}
}

impl Default for BorrowChecker {
	fn default() -> Self {
		Self::new()
	}
}
