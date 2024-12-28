// advanced comptime engine for lemon
// by Yazalde Filimone <yazaldefilimon@gmail.com>
//

use crate::ir;
pub struct Engine<'eng> {
	root: &'eng ir::Root,
}

impl<'eng> Engine<'eng> {
	pub fn new(root: &'eng ir::Root) -> Self {
		Self { root }
	}
	pub fn run(&mut self) {
		todo!("run comptime engine");
	}
}
