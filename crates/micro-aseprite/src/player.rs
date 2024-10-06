use std::{
	sync::{Arc, Mutex},
	time::Duration,
};

use micro::{
	color::{ColorConstants, LinSrgba},
	graphics::{shader::Shader, texture::Texture, BlendMode},
	math::Mat4,
	standard_draw_param_methods, Context,
};

use super::{AnimationData, Frame, Repeats};

#[derive(Debug, Clone)]
pub struct AnimationPlayer {
	inner: Arc<Mutex<AnimationPlayerInner>>,
	pub shader: Option<Shader>,
	pub transform: Mat4,
	pub color: LinSrgba,
	pub blend_mode: BlendMode,
}

impl AnimationPlayer {
	pub fn new(animation_data: AnimationData, initial_animation_name: impl Into<String>) -> Self {
		let initial_animation_name = initial_animation_name.into();
		let initial_animation = &animation_data.animations[&initial_animation_name];
		let initial_animation_repeats = initial_animation.repeats;
		let initial_frame_index = initial_animation.start_frame;
		Self {
			inner: Arc::new(Mutex::new(AnimationPlayerInner {
				animation_data,
				current_animation_name: initial_animation_name,
				current_animation_remaining_repeats: initial_animation_repeats,
				current_frame_index: initial_frame_index,
				current_frame_elapsed_time: Duration::ZERO,
				current_animation_finished: false,
				paused: false,
			})),
			shader: None,
			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
			blend_mode: BlendMode::default(),
		}
	}

	standard_draw_param_methods!();

	pub fn current_animation_name(&self) -> String {
		self.inner
			.try_lock()
			.unwrap()
			.current_animation_name
			.clone()
	}

	pub fn current_frame(&self) -> Frame {
		let inner = self.inner.try_lock().unwrap();
		inner.animation_data.frames[inner.current_frame_index]
	}

	pub fn paused(&self) -> bool {
		self.inner.try_lock().unwrap().paused
	}

	pub fn set_paused(&mut self, paused: bool) {
		self.inner.try_lock().unwrap().paused = paused;
	}

	pub fn finished(&self) -> bool {
		self.inner.try_lock().unwrap().current_animation_finished
	}

	pub fn switch(&mut self, animation_name: impl Into<String>) {
		let mut inner = self.inner.try_lock().unwrap();
		inner.current_animation_name = animation_name.into();
		let animation = &inner.animation_data.animations[&inner.current_animation_name];
		let start_frame = animation.start_frame;
		let repeats = animation.repeats;
		inner.current_frame_index = start_frame;
		inner.current_frame_elapsed_time = Duration::ZERO;
		inner.current_animation_remaining_repeats = repeats;
		inner.current_animation_finished = false;
	}

	pub fn update(&mut self, delta_time: Duration) {
		let mut inner = self.inner.try_lock().unwrap();
		if inner.paused || inner.current_animation_finished {
			return;
		}
		inner.current_frame_elapsed_time += delta_time;
		loop {
			let current_frame = inner.animation_data.frames[inner.current_frame_index];
			if inner.current_frame_elapsed_time >= current_frame.duration {
				let advanced_frame = inner.advance_one_frame();
				if advanced_frame {
					inner.current_frame_elapsed_time -= current_frame.duration;
				} else {
					inner.current_animation_finished = true;
					break;
				}
			} else {
				break;
			}
		}
	}

	pub fn draw(&self, ctx: &mut Context, texture: &Texture) {
		texture
			.region(
				self.inner.try_lock().unwrap().animation_data.frames
					[self.inner.try_lock().unwrap().current_frame_index]
					.texture_region,
			)
			.shader(&self.shader)
			.transformed(self.transform)
			.color(self.color)
			.blend_mode(self.blend_mode)
			.draw(ctx);
	}
}

#[derive(Debug)]
struct AnimationPlayerInner {
	animation_data: AnimationData,
	current_animation_name: String,
	current_animation_remaining_repeats: Repeats,
	current_frame_index: usize,
	current_frame_elapsed_time: Duration,
	current_animation_finished: bool,
	paused: bool,
}

impl AnimationPlayerInner {
	fn advance_one_frame(&mut self) -> AdvancedFrame {
		let current_animation = &self.animation_data.animations[&self.current_animation_name];
		let is_last_frame = self.current_frame_index >= current_animation.end_frame;
		if is_last_frame {
			match &mut self.current_animation_remaining_repeats {
				Repeats::Infinite => {
					self.current_frame_index = current_animation.start_frame;
					true
				}
				Repeats::Finite(1) => {
					let current_animation =
						&self.animation_data.animations[&self.current_animation_name];
					if let Some(next) = &current_animation.next {
						let next_animation = &self.animation_data.animations[next];
						self.current_animation_name = next.clone();
						self.current_frame_index = next_animation.start_frame;
						self.current_animation_remaining_repeats = next_animation.repeats;
						true
					} else {
						false
					}
				}
				Repeats::Finite(0) => unreachable!(),
				Repeats::Finite(repeats) => {
					self.current_frame_index = current_animation.start_frame;
					*repeats -= 1;
					true
				}
			}
		} else {
			self.current_frame_index += 1;
			true
		}
	}
}

type AdvancedFrame = bool;
