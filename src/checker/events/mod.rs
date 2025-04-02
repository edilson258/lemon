use crate::{loader::ModId, range::Range};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EventId(pub ModId, Range);

impl EventId {
	pub fn new(mod_id: ModId, range: Range) -> Self {
		Self(mod_id, range)
	}
}

use rustc_hash::{FxHashMap, FxHashSet};

use crate::checker::types::TypeId;

#[derive(Debug)]
pub struct Event {
	types: FxHashMap<EventId, TypeId>,
	multi_types: FxHashMap<EventId, Vec<TypeId>>,
	drops: FxHashMap<EventId, FxHashSet<String>>,
}

impl Event {
	pub fn new() -> Self {
		let types = FxHashMap::default();
		let multi_types = FxHashMap::default();
		let drops = FxHashMap::default();
		Event { types, drops, multi_types }
	}

	pub fn add_type(&mut self, event_id: EventId, type_id: TypeId) {
		self.types.insert(event_id, type_id);
	}

	pub fn add_multi_type(&mut self, event_id: EventId, type_ids: Vec<TypeId>) {
		self.multi_types.insert(event_id, type_ids);
	}

	pub fn lookup_multi_types(&self, event_id: EventId) -> Option<&Vec<TypeId>> {
		self.multi_types.get(&event_id)
	}

	pub fn add_drop(&mut self, event_id: EventId, drop: String) {
		self.drops.entry(event_id).or_default().insert(drop);
	}

	pub fn lookup_type(&self, event_id: EventId) -> Option<TypeId> {
		self.types.get(&event_id).cloned()
	}

	pub fn lookup_drops(&self, event_id: EventId) -> Option<&FxHashSet<String>> {
		self.drops.get(&event_id)
	}
}

impl Default for Event {
	fn default() -> Self {
		Self::new()
	}
}
