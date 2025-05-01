use crate::range::Range;

use super::{events::EventId, types::TypeId, Checker};

impl Checker<'_> {
	pub fn register_type(&mut self, type_id: TypeId, range: Range) {
		let event_id = EventId::new(self.ctx.mod_id, range);
		self.ctx.event.add_type(event_id, type_id);
	}

	pub fn register_multi_type(&mut self, type_ids: Vec<TypeId>, range: Range) {
		let event_id = EventId::new(self.ctx.mod_id, range);
		self.ctx.event.add_multi_type(event_id, type_ids);
	}

	pub fn register_drop(&mut self, event_id: EventId, drop: String) {
		self.ctx.event.add_drop(event_id, drop);
	}
}
