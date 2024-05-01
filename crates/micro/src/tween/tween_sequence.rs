use std::{
	fmt::Debug,
	ops::{Add, AddAssign, RangeInclusive},
	time::Duration,
};

use thiserror::Error;

use crate::math::{InverseLerp, Lerp};

use super::Easing;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct TweenSequence<V, T = Duration> {
	keyframes: Vec<Keyframe<V, T>>,
	current_time: T,
}

impl<V, T> TweenSequence<V, T> {
	pub fn new(initial_value: V) -> Self
	where
		T: Default,
	{
		Self {
			keyframes: vec![Keyframe {
				time: T::default(),
				value: initial_value,
				easing: Easing::Linear,
			}],
			current_time: T::default(),
		}
	}

	pub fn starting_at(time: T, initial_value: V) -> Self
	where
		T: Copy,
	{
		Self {
			keyframes: vec![Keyframe {
				time,
				value: initial_value,
				easing: Easing::Linear,
			}],
			current_time: time,
		}
	}

	pub fn simple(duration: T, values: RangeInclusive<V>, easing: Easing) -> Self
	where
		T: Default + Copy,
	{
		let (start, end) = values.into_inner();
		Self {
			keyframes: vec![
				Keyframe {
					time: T::default(),
					value: start,
					easing: Easing::Linear,
				},
				Keyframe {
					time: duration,
					value: end,
					easing,
				},
			],
			current_time: T::default(),
		}
	}

	pub fn wait(mut self, duration: T) -> Self
	where
		V: Copy,
		T: Copy + Add<T, Output = T>,
	{
		let last_keyframe = self.keyframes.last().unwrap();
		self.keyframes.push(Keyframe {
			time: last_keyframe.time + duration,
			value: last_keyframe.value,
			easing: Easing::Linear,
		});
		self
	}

	pub fn wait_until(mut self, time: T) -> Self
	where
		V: Copy,
		T: PartialOrd,
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

	pub fn tween(mut self, duration: T, target: V, easing: Easing) -> Self
	where
		T: Copy + Add<T, Output = T>,
	{
		let last_keyframe = self.keyframes.last().unwrap();
		self.keyframes.push(Keyframe {
			time: last_keyframe.time + duration,
			value: target,
			easing,
		});
		self
	}

	pub fn tween_until(mut self, time: T, value: V, easing: Easing) -> Self
	where
		T: PartialOrd,
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

	pub fn duration(&self) -> T
	where
		T: Copy,
	{
		self.keyframes.last().unwrap().time
	}

	pub fn update(&mut self, delta_time: T)
	where
		T: AddAssign<T>,
	{
		self.current_time += delta_time;
	}

	pub fn get(&self, time: T) -> V
	where
		V: Copy + Lerp,
		T: Copy + PartialOrd + InverseLerp,
	{
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

	pub fn current(&self) -> V
	where
		V: Copy + Lerp,
		T: Copy + PartialOrd + InverseLerp,
	{
		self.get(self.current_time)
	}

	pub fn finished(&self) -> bool
	where
		T: PartialOrd + Copy,
	{
		self.current_time >= self.duration()
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct Keyframe<V, T = Duration> {
	pub time: T,
	pub value: V,
	pub easing: Easing,
}

#[derive(Debug, Clone, PartialEq, Error)]
#[error("Sequence already has a keyframe at time {time}")]
pub struct KeyframeAlreadyAtTime<V, T> {
	pub time: T,
	pub sequence: TweenSequence<V, T>,
}
