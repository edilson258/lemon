use rustc_hash::FxHashMap;

use super::pointer::{Addresses, Ptr, PtrKind};
use crate::{
	error_ownership,
	message::{Message, MessageResult},
};

pub type Address = usize;
pub type PtrId = usize;

pub const PTR_ID_NONE: PtrId = usize::MAX;

#[derive(Debug, Default)]
pub struct OwnershipTracker {
	borrow_stacks: Vec<Vec<PtrKind>>, // address -> stack
	ptrs: Vec<Ptr>,                   // ptr_id -> Ptr
	usage_count: FxHashMap<PtrId, usize>,
}

impl OwnershipTracker {
	pub fn new() -> Self {
		Self::default()
	}
	// fn get_stack_for(&mut self, ptr_id: PtrId) -> MessageResult<(&Ptr, &mut Vec<PtrKind>)> {
	// 	let ptr = self.ptrs.get(ptr_id).ok_or(Self::internal_error("pointer not found"))?;
	// 	let stack = self
	// 		.borrow_stacks
	// 		.get_mut(ptr.address)
	// 		.ok_or(Self::internal_error("use of value after it was freed"))?;

	// 	Ok((ptr, stack))
	// }

	pub fn lookup_stack(&mut self, address: usize) -> MessageResult<&mut Vec<PtrKind>> {
		match self.borrow_stacks.get_mut(address) {
			Some(stack) => Ok(stack),
			None => Err(Self::internal_error("use of value after it was freed")),
		}
	}

	pub fn lookup_ptr(&self, address: usize) -> MessageResult<&Ptr> {
		match self.ptrs.get(address) {
			Some(ptr) => Ok(ptr),
			None => Err(Self::internal_error("use of value after it was freed")),
		}
	}

	pub fn next_borrow_ids(&self) -> (PtrId, PtrId) {
		let borrow_id = self.ptrs.len();
		let succ_id = borrow_id + 1;
		(borrow_id, succ_id)
	}

	pub fn owned_pointer(&mut self) -> PtrId {
		self.alloc_pointer(PtrKind::Owned)
	}

	pub fn copied_pointer(&mut self) -> PtrId {
		self.alloc_pointer(PtrKind::Copied)
	}

	pub fn alloc_pointer(&mut self, kind: PtrKind) -> PtrId {
		let ptr_id = self.ptrs.len();
		let address = self.borrow_stacks.len();
		self.borrow_stacks.push(vec![kind]);
		let ptr = Ptr::new(ptr_id, address, kind);
		self.ptrs.push(ptr);
		ptr_id
	}

	pub fn alloc_pointer_with_addresses(&mut self, addresses: Addresses, kind: PtrKind) -> PtrId {
		let ptr_id = self.ptrs.len();
		let address = self.borrow_stacks.len();
		self.borrow_stacks.push(vec![kind]);
		let ptr = Ptr::new_addresses(ptr_id, addresses, kind);
		self.ptrs.push(ptr);
		ptr_id
	}

	pub fn mutable_borrow(&mut self, from: PtrId) -> MessageResult<(Ptr, Ptr)> {
		self.mark_and_drop_if_needed(from)?;
		self.create_borrow(from, PtrKind::MutableBorrow)
	}

	pub fn readonly_borrow(&mut self, from: PtrId) -> MessageResult<(Ptr, Ptr)> {
		self.mark_and_drop_if_needed(from)?;
		self.create_borrow(from, PtrKind::ReadOnlyBorrow)
	}
	fn create_borrow(&mut self, from: PtrId, borrow_kind: PtrKind) -> MessageResult<(Ptr, Ptr)> {
		let (borrow_id, succesor_id) = self.next_borrow_ids();

		let from_ptr = self.ptrs.get(from).ok_or(Self::internal_error("pointer not found"))?.clone();

		// Verifica todos os stacks envolvidos
		for &addr in &from_ptr.addresses {
			let stack = self.lookup_stack(addr)?;
			let Some(&active) = stack.last() else {
				return Err(error_ownership!("use of value after it was freed"));
			};

			if active != from_ptr.kind {
				let message = self.borrow_conflict(borrow_kind, active);
				let message = message
					.note_if_some(None, "use the previous borrow before creating a new one")
					.note_if_some(None, "or move the new borrow to a separate scope");
				return Err(message);
			}

			match borrow_kind {
				PtrKind::MutableBorrow if matches!(active, PtrKind::ReadOnlyBorrow | PtrKind::Copied) => {
					let message = self
						.borrow_conflict(borrow_kind, active)
						.note_if_some(None, "or move the 'immutable' borrow before the 'mutable'");
					return Err(message);
				}
				PtrKind::ReadOnlyBorrow if matches!(active, PtrKind::MutableBorrow | PtrKind::Copied) => {
					let message = self
						.borrow_conflict(borrow_kind, active)
						.note_if_some(None, "make sure the previous borrow is no longer used")
						.note_if_some(None, "use a new scope to separate the borrows");
					return Err(message);
				}
				_ => {}
			}
		}

		for &addr in &from_ptr.addresses {
			let stack = self.lookup_stack(addr)?;
			stack.pop();
			stack.push(from_ptr.kind); // successor
			stack.push(borrow_kind); // borrow
		}

		let borrow_ptr = Ptr::new_addresses(borrow_id, from_ptr.addresses.clone(), borrow_kind);
		let successor_ptr = Ptr::new_addresses(succesor_id, from_ptr.addresses.clone(), from_ptr.kind);

		self.ptrs.push(borrow_ptr.clone());
		self.ptrs.push(successor_ptr.clone());

		Ok((borrow_ptr, successor_ptr))
	}

	pub fn drop_pointer(&mut self, ptr_id: PtrId) -> MessageResult<()> {
		let ptr = self.lookup_ptr(ptr_id)?.clone();
		for addr in ptr.addresses.iter() {
			let stack = self.lookup_stack(*addr)?;
			let Some(&active) = stack.last() else {
				return Err(error_ownership!("use of value after it was freed"));
			};

			if active != ptr.kind {
				let text_error = format!("cannot drop, found '{}', expected '{}'", active, ptr.kind);
				let message = Self::internal_error(text_error)
					.note_if_some(None, "only the top of the stack can be dropped");
				return Err(message);
			}

			stack.pop();
		}

		println!("drop: {}", ptr_id);
		Ok(())
	}

	pub fn register_use(&mut self, ptr_id: PtrId) {
		*self.usage_count.entry(ptr_id).or_insert(0) += 1;
	}

	pub fn mark_and_drop_if_needed(&mut self, ptr_id: PtrId) -> MessageResult<()> {
		println!("drop: {}", ptr_id);
		self.register_use(ptr_id);
		if self.mark_used(ptr_id) {
			let ptr = &self.ptrs[ptr_id];
			println!("drop_kind {}", ptr.kind);
			match ptr.kind {
				PtrKind::MutableBorrow | PtrKind::ReadOnlyBorrow => {
					self.drop_pointer(ptr_id)?;
				}
				PtrKind::Owned | PtrKind::Copied => {}
			}
		}
		Ok(())
	}

	/// marks the pointer as used, returns true if we can drop/free it
	pub fn mark_used(&mut self, ptr_id: PtrId) -> bool {
		if let Some(count) = self.usage_count.get_mut(&ptr_id) {
			*count = count.saturating_sub(1);
			if *count == 0 {
				return true;
			}
		}
		false
	}

	pub fn borrow_conflict(&self, requested: PtrKind, active: PtrKind) -> Message {
		error_ownership!("cannot borrow as '{}' while already borrowed as '{}'", requested, active)
	}

	pub fn internal_error(message: impl Into<String>) -> Message {
		let message = Message::error_ownership(message);
		message.note_internal()
	}
}
