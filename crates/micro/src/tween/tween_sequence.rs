use std::{
	collections::VecDeque,
	fmt::Debug,
	ops::{Add, AddAssign, RangeInclusive, SubAssign},
	time::Duration,
};

use thiserror::Error;

use crate::math::{InverseLerp, Lerp};

use super::Easing;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct TweenSequence<Value, Time = Duration, Event = ()> {
	keyframes: Vec<Keyframe<Value, Time>>,
	events: Vec<(Time, Event)>,
	current_time: Time,
	previous_time: Time,
	looping: bool,
	emitted_events: VecDeque<Event>,
}

impl<Value, Time, Event> TweenSequence<Value, Time, Event> {
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
			events: vec![],
			current_time: Time::default(),
			previous_time: Time::default(),
			looping: false,
			emitted_events: VecDeque::new(),
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
			events: vec![],
			current_time: time,
			previous_time: time,
			looping: false,
			emitted_events: VecDeque::new(),
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
			events: vec![],
			current_time: Time::default(),
			previous_time: Time::default(),
			looping: false,
			emitted_events: VecDeque::new(),
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

	pub fn emit(mut self, event: Event) -> Self
	where
		Time: Copy,
	{
		self.events.push((self.duration(), event));
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
		Event: Clone,
	{
		self.previous_time = self.current_time;
		self.current_time += delta_time;
		if self.looping {
			while self.current_time >= self.duration() {
				self.current_time -= self.duration();
			}
		}
		/*
		it's possible that if the delta time is high enough, the animation will loop
		multiple times in one update. in that case, events won't be emitted
		enough times. i'm not bothering to solve that problem for now because:

		- seems complicated
		- micro should probably be clamping delta times anyway to avoid issues
		like tunneling
		*/
		for (time, event) in &self.events {
			if was_time_just_passed(*time, self.previous_time, self.current_time) {
				self.emitted_events.push_back(event.clone());
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

	pub fn pop_event(&mut self) -> Option<Event> {
		self.emitted_events.pop_front()
	}

	pub fn map<NewValue>(
		&self,
		mut f: impl FnMut(Value) -> NewValue,
	) -> TweenSequence<NewValue, Time, Event>
	where
		Value: Copy,
		Time: Copy,
		Event: Clone,
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
			events: self.events.clone(),
			current_time: self.current_time,
			previous_time: self.previous_time,
			looping: self.looping,
			emitted_events: VecDeque::new(),
		}
	}

	pub fn map_events<NewEvent>(
		&self,
		mut f: impl FnMut(&Event) -> NewEvent,
	) -> TweenSequence<Value, Time, NewEvent>
	where
		Value: Copy,
		Time: Copy,
		Event: Clone,
	{
		TweenSequence {
			keyframes: self.keyframes.clone(),
			events: self
				.events
				.iter()
				.map(|(time, event)| (*time, f(event)))
				.collect(),
			current_time: self.current_time,
			previous_time: self.previous_time,
			looping: self.looping,
			emitted_events: VecDeque::new(),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct Keyframe<Value, Time = Duration> {
	pub time: Time,
	pub value: Value,
	pub easing: Easing,
}

#[derive(Debug, Clone, PartialEq, Error)]
#[error("Sequence already has a keyframe at time {time}")]
pub struct KeyframeAlreadyAtTime<V, T> {
	pub time: T,
	pub sequence: TweenSequence<V, T>,
}

fn was_time_just_passed<T: PartialOrd>(time: T, previous: T, current: T) -> bool {
	if previous > current {
		time > previous || time <= current
	} else {
		time > previous && time <= current
	}
}
