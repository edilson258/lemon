use crate::{error_ownership, message::Message};

#[inline]
pub fn mutable_more_than_once(name: impl Into<String>) -> Message {
	error_ownership!("cannot borrow '{}' as mutable more than once", name.into())
}
#[inline]
pub fn mutable_while_droped(name: impl Into<String>) -> Message {
	error_ownership!("cannot borrow '{}' as mutable while droped", name.into())
}

#[inline]
pub fn immutable_while_mutable_exists(name: impl Into<String>) -> Message {
	error_ownership!("cannot borrow '{}' as immutable while mutable exists", name.into())
}

#[inline]
pub fn mutable_while_immutable_exists(name: impl Into<String>) -> Message {
	error_ownership!("cannot borrow '{}' as mutable while immutable exists", name.into())
}

#[inline]
pub fn immutable_while_droped(name: impl Into<String>) -> Message {
	error_ownership!("cannot borrow '{}' as immutable while droped", name.into())
}
