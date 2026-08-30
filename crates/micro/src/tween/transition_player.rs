use std::time::Duration;

use crate::{
	math::Lerp,
	tween::{Easing, TweenSequence},
};

#[derive(Debug, Clone, PartialEq)]
pub struct TransitionPlayer<T> {
	previous_target: T,
	tween_sequence: TweenSequence<T>,
}

impl<T> TransitionPlayer<T> {
	pub fn new(initial_value: T) -> Self
	where
		T: Copy,
	{
		Self {
			previous_target: initial_value,
			tween_sequence: TweenSequence::new(initial_value),
		}
	}

	pub fn update(&mut self, target: T, duration: Duration, easing: Easing, delta_time: Duration)
	where
		T: Copy + Lerp + PartialEq,
	{
		self.tween_sequence.update(delta_time);
		if target != self.previous_target {
			self.tween_sequence =
				TweenSequence::simple(duration, self.tween_sequence.current()..=target, easing);
			self.previous_target = target;
		}
	}

	pub fn current(&self) -> T
	where
		T: Copy + Lerp,
	{
		self.tween_sequence.current()
	}
}
