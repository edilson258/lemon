use super::ptr::RefId;
use std::ops::{Index, IndexMut};

#[derive(Debug, Default)]
pub struct Arena<T> {
	items: Vec<T>,
}

impl<T> Arena<T> {
	pub fn new() -> Self {
		Self { items: Vec::new() }
	}

	pub fn insert(&mut self, value: T) -> RefId {
		let id = RefId(self.items.len());
		self.items.push(value);
		id
	}

	pub fn get(&self, id: RefId) -> Option<&T> {
		self.items.get(id.0)
	}

	pub fn get_mut(&mut self, id: RefId) -> Option<&mut T> {
		self.items.get_mut(id.0)
	}

	pub fn iter(&self) -> impl Iterator<Item = (RefId, &T)> {
		self.items.iter().enumerate().map(|(i, item)| (RefId(i), item))
	}

	pub fn iter_mut(&mut self) -> impl Iterator<Item = (RefId, &mut T)> {
		self.items.iter_mut().enumerate().map(|(i, item)| (RefId(i), item))
	}

	pub fn len(&self) -> usize {
		self.items.len()
	}

	pub fn is_empty(&self) -> bool {
		self.items.is_empty()
	}
}

impl<T> Index<RefId> for Arena<T> {
	type Output = T;

	fn index(&self, id: RefId) -> &Self::Output {
		&self.items[id.0]
	}
}

impl<T> IndexMut<RefId> for Arena<T> {
	fn index_mut(&mut self, id: RefId) -> &mut Self::Output {
		&mut self.items[id.0]
	}
}
