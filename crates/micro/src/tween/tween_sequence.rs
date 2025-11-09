use std::{
	fmt::Debug,
	ops::{Add, AddAssign, RangeInclusive, SubAssign},
	time::Duration,
};

use crate::math::{InverseLerp, Lerp};

use super::Easing;

/// Smoothly animates a value through multiple target values using a series
/// of tweens.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct TweenSequence<V, T = Duration> {
	keyframes: Vec<Keyframe<V, T>>,
	current_time: T,
	looping: bool,
}

impl<Value, Time> TweenSequence<Value, Time> {
	/// Creates a new [`TweenSequence`] starting with an initial value.
	pub fn new(initial_value: Value) -> Self
	where
		Time: Default,
	{
		Self {
			keyframes: vec![Keyframe {
				time: Time::default(),
				value: initial_value,
				easing: Easing::Linear,
			}],
			current_time: Time::default(),
			looping: false,
		}
	}

	/// Creates a new [`TweenSequence`] where the first keyframe has the
	/// specified time and value.
	pub fn starting_at(time: Time, initial_value: Value) -> Self
	where
		Time: Copy,
	{
		Self {
			keyframes: vec![Keyframe {
				time,
				value: initial_value,
				easing: Easing::Linear,
			}],
			current_time: time,
			looping: false,
		}
	}

	/// Creates a [`TweenSequence`] with one tween between the given values.
	pub fn simple(duration: Time, values: RangeInclusive<Value>, easing: Easing) -> Self
	where
		Time: Default + Copy,
	{
		let (start, end) = values.into_inner();
		Self {
			keyframes: vec![
				Keyframe {
					time: Time::default(),
					value: start,
					easing: Easing::Linear,
				},
				Keyframe {
					time: duration,
					value: end,
					easing,
				},
			],
			current_time: Time::default(),
			looping: false,
		}
	}

	/// Adds a keyframe that causes the value to stay the same for the
	/// given `duration`.
	pub fn wait(mut self, duration: Time) -> Self
	where
		Value: Copy,
		Time: Copy + Add<Time, Output = Time>,
	{
		let last_keyframe = self.keyframes.last().unwrap();
		self.keyframes.push(Keyframe {
			time: last_keyframe.time + duration,
			value: last_keyframe.value,
			easing: Easing::Linear,
		});
		self
	}

	/// Adds a keyframe that causes the value to stay the same until
	/// the given `time`.
	pub fn wait_until(mut self, time: Time) -> Self
	where
		Value: Copy,
		Time: PartialOrd,
	{
		let last_keyframe = self.keyframes.last().unwrap();
		if time <= last_keyframe.time {
			panic!("time must be greater than last keyframe time");
		}
		self.keyframes.push(Keyframe {
			time,
			value: last_keyframe.value,
			easing: Easing::Linear,
		});
		self
	}

	/// Adds a keyframe that causes the value to change to the `target`
	/// value over the course of the given `duration`.
	pub fn tween(mut self, duration: Time, target: Value, easing: Easing) -> Self
	where
		Time: Copy + Add<Time, Output = Time>,
	{
		let last_keyframe = self.keyframes.last().unwrap();
		self.keyframes.push(Keyframe {
			time: last_keyframe.time + duration,
			value: target,
			easing,
		});
		self
	}

	/// Adds a keyframe that causes the value to change to the `target`
	/// value until the given `time`.
	pub fn tween_until(mut self, time: Time, value: Value, easing: Easing) -> Self
	where
		Time: PartialOrd,
	{
		if time <= self.keyframes.last().unwrap().time {
			panic!("time must be greater than last keyframe time");
		}
		self.keyframes.push(Keyframe {
			time,
			value,
			easing,
		});
		self
	}

	/// Makes the [`TweenSequence`] a looping animation.
	pub fn looping(self) -> Self {
		Self {
			looping: true,
			..self
		}
	}

	/// Returns the time of the last keyframe.
	pub fn duration(&self) -> Time
	where
		Time: Copy,
	{
		self.keyframes.last().unwrap().time
	}

	/// Progresses the animation by the given amount of time.
	pub fn update(&mut self, delta_time: Time)
	where
		Time: Copy + PartialOrd + AddAssign<Time> + SubAssign<Time>,
	{
		self.current_time += delta_time;
		if self.looping {
			while self.current_time >= self.duration() {
				self.current_time -= self.duration();
			}
		}
	}

	/// Gets the output value at the given `time`.
	pub fn get(&self, mut time: Time) -> Value
	where
		Value: Copy + Lerp,
		Time: Copy + PartialOrd + InverseLerp + SubAssign<Time>,
	{
		if self.looping {
			while time >= self.duration() {
				time -= self.duration();
			}
		}
		let first_keyframe = self.keyframes.first().unwrap();
		if time < first_keyframe.time {
			return first_keyframe.value;
		}
		let (current_keyframe_index, current_keyframe) = self
			.keyframes
			.iter()
			.enumerate()
			.rev()
			.find(|(_, keyframe)| keyframe.time <= time)
			.unwrap();
		let next_keyframe = self.keyframes.get(current_keyframe_index + 1);
		if let Some(next_keyframe) = next_keyframe {
			let f = time.inverse_lerp(current_keyframe.time, next_keyframe.time);
			current_keyframe
				.value
				.lerp(next_keyframe.value, next_keyframe.easing.ease(f))
		} else {
			current_keyframe.value
		}
	}

	/// Gets the output value at the current time.
	pub fn current(&self) -> Value
	where
		Value: Copy + Lerp,
		Time: Copy + PartialOrd + InverseLerp + SubAssign<Time>,
	{
		self.get(self.current_time)
	}

	/// Returns `true` if the animation has finished.
	pub fn finished(&self) -> bool
	where
		Time: PartialOrd + Copy,
	{
		self.current_time >= self.duration()
	}

	/// Creates a new [`TweenSequence`] with the keyframe values mapped
	/// to new values by the given callback.
	pub fn map<NewValue>(
		&self,
		mut f: impl FnMut(Value) -> NewValue,
	) -> TweenSequence<NewValue, Time>
	where
		Value: Copy,
		Time: Copy,
	{
		TweenSequence {
			keyframes: self
				.keyframes
				.iter()
				.map(|old| Keyframe {
					time: old.time,
					value: f(old.value),
					easing: old.easing.clone(),
				})
				.collect(),
			current_time: self.current_time,
			looping: self.looping,
		}
	}
}

/// A keyframe in a [`TweenSequence`]. These define the animation.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct Keyframe<V, T = Duration> {
	/// The time the keyframe occurs at.
	pub time: T,
	/// The target value of the keyframe.
	pub value: V,
	/// The curve of the animation between the previous keyframe and this one.
	///
	/// For the first keyframe, this has no effect.
	pub easing: Easing,
}
