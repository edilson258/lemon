// ast -> ir
//

use crate::checker::{context::store::Store, types::TypeStore};

pub struct Compiler<'cp> {
	store: &'cp mut Store,
	type_store: &'cp mut TypeStore,
}

impl<'cp> Compiler<'cp> {
	pub fn new(store: &'cp mut Store, type_store: &'cp mut TypeStore) -> Self {
		Self { store, type_store }
	}
}
