use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedTimestepProducer {
	interval: Duration,
	max_steps_per_frame: usize,
	time_since_last_step: Duration,
}

impl FixedTimestepProducer {
	pub fn new(settings: FixedTimestepProducerSettings) -> Self {
		Self {
			interval: Duration::from_secs_f64(1.0 / settings.ticks_per_second),
			max_steps_per_frame: settings.max_steps_per_frame,
			time_since_last_step: Duration::ZERO,
		}
	}

	pub fn update(&mut self, delta_time: Duration) -> usize {
		let mut num_steps = 0;
		self.time_since_last_step += delta_time;
		while self.time_since_last_step >= self.interval {
			self.time_since_last_step -= self.interval;
			num_steps += 1;
			if num_steps >= self.max_steps_per_frame {
				self.time_since_last_step = Duration::ZERO;
				break;
			}
		}
		num_steps
	}

	pub fn run<E>(
		&mut self,
		delta_time: Duration,
		mut f: impl FnMut(Duration) -> Result<(), E>,
	) -> Result<(), E> {
		let num_steps = self.update(delta_time);
		for _ in 0..num_steps {
			f(self.interval)?;
		}
		Ok(())
	}
}

impl Default for FixedTimestepProducer {
	fn default() -> Self {
		Self::new(FixedTimestepProducerSettings::default())
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FixedTimestepProducerSettings {
	pub ticks_per_second: f64,
	pub max_steps_per_frame: usize,
}

impl Default for FixedTimestepProducerSettings {
	fn default() -> Self {
		Self {
			ticks_per_second: 60.0,
			max_steps_per_frame: 2,
		}
	}
}
