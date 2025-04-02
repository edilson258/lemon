use core::fmt;

use super::{pointer::PtrKind, tracker::PtrId};

#[derive(Debug)]
pub enum OwnershipError {
	PtrNotFound(PtrId),
	UseAfterFree,
	InvalidBorrowState { address: usize },
	CannotBorrowMutWhileShared,
	CannotBorrowSharedWhileMut,
	InvalidDropState { expected: PtrKind, actual: PtrKind },
}
// 1. cannot borrow `i` as mutable more than once at a time
// 2. cannot borrow `*a` as mutable because `a` is also borrowed
// 3. cannot assign to `fancy_num` because it is borrowed
// 4. `y` does not live long enough
// 5. cannot borrow mutably
impl fmt::Display for OwnershipError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::PtrNotFound(id) => {
				write!(f, "pointer value not found, shared borrow occurs here")
			}
			Self::UseAfterFree => write!(f, "use of value after it was freed"),
			Self::InvalidBorrowState { address } => {
				write!(f, "value state does not match current borrow")
			}
			Self::CannotBorrowMutWhileShared => {
				write!(f, "cannot borrow as mutable while already borrowed as shared")
			}
			Self::CannotBorrowSharedWhileMut => {
				write!(f, "cannot borrow as shared while already borrowed as mutable")
			}
			Self::InvalidDropState { expected, actual } => {
				write!(f, "cannot drop value in state `{}`, expected `{}`", actual, expected)
			}
		}
	}
}
