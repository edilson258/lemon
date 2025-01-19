use std::collections::HashMap;

use crate::checker::types::{ExternFnType, FnType, Type};

#[derive(Debug)]
pub struct MonomorphicStore {
	pub current_fn: Option<String>,
	pub fns: HashMap<String, Vec<Type>>,
}

impl MonomorphicStore {
	pub fn new() -> Self {
		Self { fns: HashMap::new(), current_fn: None }
	}

	pub fn add_fn(&mut self, fn_type: FnType) {
		if let Some(name) = &self.current_fn {
			self.fns.entry(name.clone()).or_default().push(fn_type.into());
		} else {
			todo!("not found fn name")
		}
	}

	pub fn add_extern_fn(&mut self, fn_type: ExternFnType) {
		if let Some(name) = &self.current_fn {
			self.fns.entry(name.clone()).or_default().push(fn_type.into());
		} else {
			todo!("not found fn name")
		}
	}
	pub fn create_fn(&mut self, name: String) {
		self.current_fn = Some(name);
	}

	pub fn end_fn(&mut self) {
		self.current_fn = None;
	}

	pub fn get_fns(&self, name: &str) -> Option<Vec<FnType>> {
		let fns = self.fns.get(name)?;
		let mut fns_type = Vec::with_capacity(fns.len());
		for fn_type in fns {
			match fn_type {
				Type::Fn(fn_type) => fns_type.push(fn_type.clone()),
				_ => todo!("not found: {:?}", fn_type),
			}
		}
		Some(fns_type)
	}
}

impl Default for MonomorphicStore {
	fn default() -> Self {
		Self::new()
	}
}
