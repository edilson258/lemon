#[derive(Debug)]
pub struct Flow {
	pub unreachable: bool,  // unreachable code
	pub paths_return: bool, // return in all paths
}

impl Flow {
	pub fn new() -> Self {
		Self { unreachable: false, paths_return: false }
	}
	pub fn set_unreachable(&mut self, state: bool) {
		self.unreachable = state;
	}

	pub fn set_paths_return(&mut self, state: bool) {
		self.paths_return = state;
	}

	pub fn is_unreachable(&self) -> bool {
		self.unreachable
	}

	pub fn is_paths_return(&self) -> bool {
		self.paths_return
	}
}

impl Default for Flow {
	fn default() -> Self {
		Self::new()
	}
}
