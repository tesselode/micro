use std::{
	fmt::Debug,
	ops::{Add, AddAssign, RangeInclusive, SubAssign},
	time::Duration,
};

use derive_more::derive::{Display, Error};

use crate::math::{InverseLerp, Lerp};

use super::Easing;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct TweenSequence<V, T = Duration> {
	keyframes: Vec<Keyframe<V, T>>,
	current_time: T,
	looping: bool,
}

impl<Value, Time> TweenSequence<Value, Time> {
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

	pub fn looping(self) -> Self {
		Self {
			looping: true,
			..self
		}
	}

	pub fn duration(&self) -> Time
	where
		Time: Copy,
	{
		self.keyframes.last().unwrap().time
	}

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

	pub fn current(&self) -> Value
	where
		Value: Copy + Lerp,
		Time: Copy + PartialOrd + InverseLerp + SubAssign<Time>,
	{
		self.get(self.current_time)
	}

	pub fn finished(&self) -> bool
	where
		Time: PartialOrd + Copy,
	{
		self.current_time >= self.duration()
	}

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

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct Keyframe<V, T = Duration> {
	pub time: T,
	pub value: V,
	pub easing: Easing,
}

#[derive(Debug, Clone, PartialEq, Error, Display)]
#[display("Sequence already has a keyframe at time {time}")]
pub struct KeyframeAlreadyAtTime<V, T> {
	pub time: T,
	pub sequence: TweenSequence<V, T>,
}
