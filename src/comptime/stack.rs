use super::frame::Frame;

pub struct Stack {
	frames: Vec<Frame>,
}

impl Stack {
	pub fn new() -> Self {
		let frames = Vec::from_iter(vec![Frame::new()]);
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
			todo!("comptime error: stack is empty");
		}
		self.frames.last_mut().unwrap()
	}
}
