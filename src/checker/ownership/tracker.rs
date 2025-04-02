use crate::{error_ownership, message::MessageResult};

use super::pointer::{Ptr, PtrKind};

pub type Address = usize;
pub type PtrId = usize;

// pub type MessageResult<T> = core::result::Result<T, OwnershipError>;

#[derive(Debug, Default)]
pub struct OwnershipTracker {
	borrow_stacks: Vec<Vec<PtrKind>>, // [Address]
	ptrs: Vec<Ptr>,                   // [PtrId]
}

impl OwnershipTracker {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn get_borrow_stack(&self, address: Address) -> Option<&Vec<PtrKind>> {
		self.borrow_stacks.get(address)
	}

	pub fn owned_pointer(&mut self) -> Ptr {
		let ptr_id = self.ptrs.len() as PtrId;
		let address = self.borrow_stacks.len() as Address;
		self.borrow_stacks.push(vec![PtrKind::Owned]);
		let ptr = Ptr::new_owned(ptr_id, address);
		self.ptrs.push(ptr);
		ptr
	}
	pub fn copied_pointer(&mut self) -> Ptr {
		let ptr_id = self.ptrs.len() as PtrId;
		let address = self.borrow_stacks.len() as Address;
		self.borrow_stacks.push(vec![PtrKind::Copied]);
		let ptr = Ptr::new_copied(ptr_id, address);
		self.ptrs.push(ptr);
		ptr
	}

	pub fn mutable_borrow(&mut self, from: PtrId) -> MessageResult<(Ptr, Ptr)> {
		let ptr = self.ptrs.get(from).ok_or(error_ownership!("pointer value not found"))?;
		let stack = match self.borrow_stacks.get_mut(ptr.address) {
			Some(stack) => stack,
			None => return Err(error_ownership!("use of value after it was freed")),
		};
		match (stack.last(), ptr.kind) {
			(Some(&kind), _) if kind != ptr.kind => {
				let message = error_ownership!("cannot borrow as mutable while already {}", kind);
				return Err(message);
			}
			(_, PtrKind::ReadOnlyBorrow | PtrKind::Copied) => {
				let message = error_ownership!("cannot borrow as mutable while already shared");
				return Err(message);
			}
			_ => {}
		}

		let id = self.ptrs.len();
		let borrow = Ptr::new_mutable_borrow(id, ptr.address);
		let succ = Ptr::new(id + 1, ptr.address, ptr.kind);

		stack.pop();
		stack.push(succ.kind);
		stack.push(borrow.kind);

		self.ptrs.push(borrow);
		self.ptrs.push(succ);
		Ok((borrow, succ))
	}

	pub fn readonly_borrow(&mut self, from: PtrId) -> MessageResult<(Ptr, Ptr)> {
		let ptr = self.ptrs.get(from).ok_or(error_ownership!("pointer value not found"))?;
		let stack = match self.borrow_stacks.get_mut(ptr.address) {
			Some(stack) => stack,
			None => return Err(error_ownership!("use of value after it was freed")),
		};

		match (stack.last(), ptr.kind) {
			(Some(&kind), _) if kind != ptr.kind => {
				return Err(error_ownership!("value state does not match current borrow"));
			}
			(_, PtrKind::MutableBorrow | PtrKind::Copied) => {
				return Err(error_ownership!("cannot borrow as shared while already borrowed as mutable"));
			}
			_ => {}
		}

		let id = self.ptrs.len();
		let borrow = Ptr::new_readonly_borrow(id, ptr.address);
		let succ = Ptr::new(id + 1, ptr.address, ptr.kind);

		stack.pop();
		stack.push(succ.kind);
		stack.push(borrow.kind);

		self.ptrs.push(borrow);
		self.ptrs.push(succ);
		Ok((borrow, succ))
	}
	pub fn drop_pointer(&mut self, ptr_id: PtrId) -> MessageResult<()> {
		let ptr = self.ptrs.get(ptr_id).ok_or(error_ownership!("pointer value not found"))?;

		let stack = match self.borrow_stacks.get_mut(ptr.address) {
			Some(stack) => stack,
			None => return Err(error_ownership!("use of value after it was freed")),
		};

		match stack.last() {
			Some(&top) if top == ptr.kind => {
				stack.pop();
				Ok(())
			}
			Some(&top) => {
				Err(error_ownership!("cannot drop value in state `{}`, expected `{}`", top, ptr.kind))
			}
			None => Err(error_ownership!("use of value after it was freed")),
		}
	}
}
