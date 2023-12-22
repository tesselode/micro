use std::{
	ops::{Range, RangeInclusive},
	time::Duration,
};

use super::{Easing, Tweenable};

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct TweenSequence<T: Tweenable + Copy> {
	starting_value: T,
	tweens: Vec<Tween<T>>,
	pub elapsed: Duration,
}

impl<T: Tweenable + Copy> TweenSequence<T> {
	pub fn new(starting_value: T) -> Self {
		Self {
			starting_value,
			tweens: vec![],
			elapsed: Duration::ZERO,
		}
	}

	pub fn simple(duration: Duration, values: RangeInclusive<T>, easing: Easing) -> Self {
		let mut tween_sequence = TweenSequence::new(*values.start());
		tween_sequence = tween_sequence.tween(duration, *values.end(), easing);
		tween_sequence
	}

	pub fn tween(mut self, duration: Duration, target: T, easing: Easing) -> Self {
		let (last_time, last_value) = self
			.tweens
			.last()
			.map(|Tween { times, values, .. }| (times.end, values.end))
			.unwrap_or((Duration::ZERO, self.starting_value));
		self.tweens.push(Tween {
			times: last_time..last_time + duration,
			values: last_value..target,
			easing,
		});
		self
	}

	pub fn wait(self, duration: Duration) -> Self {
		let last_value = self
			.tweens
			.last()
			.map(|Tween { values, .. }| values.end)
			.unwrap_or(self.starting_value);
		self.tween(duration, last_value, Easing::Linear)
	}

	pub fn duration(&self) -> Duration {
		self.tweens
			.last()
			.map(|Tween { times, .. }| times.end)
			.unwrap_or(Duration::ZERO)
	}

	pub fn get(&self, time: Duration) -> T {
		let current_tween = self
			.tweens
			.iter()
			.rev()
			.find(|Tween { times, .. }| times.start <= time);
		if let Some(Tween {
			times,
			values,
			easing,
		}) = current_tween
		{
			let f = (time - times.start).as_secs_f32() / (times.end - times.start).as_secs_f32();
			let f = f.clamp(0.0, 1.0);
			values.start.lerp(values.end, easing.ease(f))
		} else {
			self.starting_value
		}
	}

	pub fn current(&self) -> T {
		self.get(self.elapsed)
	}

	pub fn update(&mut self, delta_time: Duration) {
		self.elapsed += delta_time;
	}

	pub fn finished(&self) -> bool {
		self.elapsed >= self.duration()
	}
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
struct Tween<T: Tweenable> {
	times: Range<Duration>,
	values: Range<T>,
	easing: Easing,
}
