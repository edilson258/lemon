// use std::collections::HashMap;

// use super::types::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceId(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exported {
	pub source_id: SourceId,
	// pub named: HashMap<String, TypeId>,
}
