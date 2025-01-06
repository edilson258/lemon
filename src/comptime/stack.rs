use crate::report::throw_engine_error;

use super::frame::Frame;

pub struct Stack {
	frames: Vec<Frame>,
}

impl Stack {
	pub fn new(registers_size: usize) -> Self {
		let frames = Vec::from_iter(vec![Frame::new(registers_size)]);
		Self { frames }
	}

	pub fn push(&mut self, frame: Frame) {
		self.frames.push(frame);
	}

	pub fn pop(&mut self) -> Option<Frame> {
		self.frames.pop()
	}

	pub fn current_frame(&mut self) -> &mut Frame {
		if self.frames.is_empty() {
			throw_engine_error("stack is empty");
		}
		self.frames.last_mut().unwrap()
	}
}
