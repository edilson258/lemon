use rustc_hash::{FxHashMap, FxHashSet};

use crate::checker::types::TypeId;

use super::EventId;

pub struct Event {
	types: FxHashMap<EventId, TypeId>,
	drops: FxHashMap<EventId, FxHashSet<String>>,
}

impl Event {
	pub fn new() -> Self {
		let types = FxHashMap::default();
		let drops = FxHashMap::default();
		Event { types, drops }
	}

	pub fn add_type(&mut self, event_id: EventId, type_id: TypeId) {
		self.types.insert(event_id, type_id);
	}

	pub fn add_drop(&mut self, event_id: EventId, drop: String) {
		self.drops.entry(event_id).or_default().insert(drop);
	}

	pub fn get_type(&self, event_id: EventId) -> Option<TypeId> {
		self.types.get(&event_id).cloned()
	}

	pub fn get_drops(&self, event_id: EventId) -> Option<&FxHashSet<String>> {
		self.drops.get(&event_id)
	}
}
