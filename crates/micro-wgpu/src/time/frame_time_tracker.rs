use std::{collections::VecDeque, time::Duration};

const NUM_FRAME_TIMES_TO_TRACK: usize = 30;

pub struct FrameTimeTracker {
	frame_times: VecDeque<Duration>,
}

impl FrameTimeTracker {
	pub fn new() -> Self {
		Self {
			frame_times: VecDeque::new(),
		}
	}

	pub fn record(&mut self, frame_time: Duration) {
		if self.frame_times.len() >= NUM_FRAME_TIMES_TO_TRACK {
			self.frame_times.pop_front();
		}
		self.frame_times.push_back(frame_time);
	}

	pub fn average(&self) -> Duration {
		self.frame_times.iter().sum::<Duration>() / self.frame_times.len() as u32
	}
}

impl Default for FrameTimeTracker {
	fn default() -> Self {
		Self::new()
	}
}
