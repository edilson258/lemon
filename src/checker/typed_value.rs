use super::borrow::ptr::RefId;
use super::types::TypeId;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefSource {
	Single(RefId),
	Union(FxHashSet<RefId>),
}

impl RefSource {
	pub fn iter(&self) -> impl Iterator<Item = RefId> + '_ {
		match self {
			RefSource::Single(id) => std::iter::once(*id).collect::<Vec<_>>().into_iter(),
			RefSource::Union(set) => set.iter().copied().collect::<Vec<_>>().into_iter(),
		}
	}
	pub fn as_string(&self) -> String {
		let ids: Vec<String> = self.iter().map(|id| id.to_string()).collect();
		ids.join(" | ")
	}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypedValue {
	pub type_id: TypeId,
	pub source: RefSource,
	pub module: bool,
	// metadata
	// pub range: Range,
}

impl TypedValue {
	pub fn new(type_id: TypeId, owner: RefId) -> Self {
		Self { type_id, source: RefSource::Single(owner), module: false }
	}

	pub fn new_source(type_id: TypeId, source: RefSource) -> Self {
		Self { type_id, source, module: false }
	}

	pub fn new_module(type_id: TypeId, owner: RefId) -> Self {
		Self { type_id, source: RefSource::Single(owner), module: true }
	}

	pub fn infer_type(&mut self, infered: TypeId) {
		self.type_id = infered;
	}
}
