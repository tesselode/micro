use std::time::Duration;

use glam::Mat4;
use palette::LinSrgba;

use crate::graphics::{shader::Shader, standard_draw_command_methods, texture::Texture, BlendMode};

use super::{AnimationData, Frame, Repeats};

#[derive(Debug, Clone, PartialEq)]
pub struct AnimationPlayer {
	animation_data: AnimationData,
	current_animation_name: String,
	current_animation_remaining_repeats: Repeats,
	current_frame_index: usize,
	current_frame_elapsed_time: Duration,
	current_animation_finished: bool,
	pub paused: bool,
}

impl AnimationPlayer {
	pub fn new(animation_data: AnimationData, initial_animation_name: impl Into<String>) -> Self {
		let initial_animation_name = initial_animation_name.into();
		let initial_animation = &animation_data.animations[&initial_animation_name];
		let initial_animation_repeats = initial_animation.repeats;
		let initial_frame_index = initial_animation.start_frame;
		Self {
			animation_data,
			current_animation_name: initial_animation_name,
			current_animation_remaining_repeats: initial_animation_repeats,
			current_frame_index: initial_frame_index,
			current_frame_elapsed_time: Duration::ZERO,
			current_animation_finished: false,
			paused: false,
		}
	}

	pub fn current_animation_name(&self) -> &str {
		&self.current_animation_name
	}

	pub fn current_frame(&self) -> Frame {
		self.animation_data.frames[self.current_frame_index]
	}

	pub fn finished(&self) -> bool {
		self.current_animation_finished
	}

	pub fn switch(&mut self, animation_name: impl Into<String>) {
		self.current_animation_name = animation_name.into();
		let animation = &self.animation_data.animations[&self.current_animation_name];
		self.current_frame_index = animation.start_frame;
		self.current_frame_elapsed_time = Duration::ZERO;
		self.current_animation_remaining_repeats = animation.repeats;
		self.current_animation_finished = false;
	}

	pub fn update(&mut self, delta_time: Duration) {
		if self.paused || self.current_animation_finished {
			return;
		}
		self.current_frame_elapsed_time += delta_time;
		loop {
			let current_frame = self.animation_data.frames[self.current_frame_index];
			if self.current_frame_elapsed_time >= current_frame.duration {
				let advanced_frame = self.advance_one_frame();
				if advanced_frame {
					self.current_frame_elapsed_time -= current_frame.duration;
				} else {
					self.current_animation_finished = true;
					break;
				}
			} else {
				break;
			}
		}
	}

	fn draw_inner(&self, texture: &Texture, params: &DrawAnimationParams) {
		texture
			.region(self.animation_data.frames[self.current_frame_index].texture_region)
			.shader(params.shader)
			.transformed(params.transform)
			.color(params.color)
			.blend_mode(params.blend_mode)
			.draw();
	}

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

pub struct DrawAnimationParams<'a> {
	pub shader: Option<&'a Shader>,
	pub transform: Mat4,
	pub color: LinSrgba,
	pub blend_mode: BlendMode,
}

pub struct DrawAnimationCommand<'a> {
	animation_player: &'a AnimationPlayer,
	params: DrawAnimationParams<'a>,
	texture: Texture,
}

impl<'a> DrawAnimationCommand<'a> {
	standard_draw_command_methods!();
}

impl<'a> Drop for DrawAnimationCommand<'a> {
	fn drop(&mut self) {
		self.animation_player
			.draw_inner(&self.texture, &self.params);
	}
}
