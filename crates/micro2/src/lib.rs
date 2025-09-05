#![allow(clippy::large_enum_variant)]

mod app;
pub mod color;
mod context;
mod event;
pub mod graphics;
pub mod input;
mod log;
pub mod math;
mod time;
pub mod tween;
mod window;

pub use app::*;
pub use context::{ContextSettings, Push, run};
pub use event::*;
pub use time::*;
pub use window::*;

pub use egui as debug_ui;
pub use image;

use std::time::Duration;

use glam::{IVec2, Mat4, UVec2, Vec2, Vec3, vec2};
use palette::LinSrgb;
use sdl3::video::{FullscreenType, WindowPos};
use wgpu::{PresentMode, TextureFormat};

use crate::{
	context::{Context, OnDrop},
	input::{Gamepad, MouseButton, Scancode},
};

/// Gets the drawable size of the window (in pixels).
pub fn window_size() -> UVec2 {
	let (width, height) = Context::with(|ctx| ctx.window.size());
	UVec2::new(width, height)
}

pub fn window_scale() -> f32 {
	Context::with(|ctx| ctx.window.display_scale())
}

/// Returns the current window mode (windowed or fullscreen).
pub fn window_mode() -> WindowMode {
	Context::with(|ctx| match ctx.window.fullscreen_state() {
		FullscreenType::Off => {
			let (width, height) = Context::with(|ctx| ctx.window.size());
			WindowMode::Windowed {
				size: UVec2::new(width, height),
			}
		}
		FullscreenType::True => WindowMode::Fullscreen,
		FullscreenType::Desktop => WindowMode::Fullscreen,
	})
}

/// Returns the resolution of the monitor the window is on.
pub fn monitor_resolution() -> UVec2 {
	let display_mode = Context::with(|ctx| ctx.window.get_display().unwrap().get_mode().unwrap());
	UVec2::new(display_mode.w as u32, display_mode.h as u32)
}

/// Sets the window mode (windowed or fullscreen).
pub fn set_window_mode(window_mode: WindowMode) {
	Context::with_mut(|ctx| match window_mode {
		WindowMode::Fullscreen => {
			ctx.window.set_fullscreen(true).unwrap();
		}
		WindowMode::Windowed { size } => {
			ctx.window.set_fullscreen(false).unwrap();
			ctx.window.set_size(size.x, size.y).unwrap();
			ctx.window
				.set_position(WindowPos::Centered, WindowPos::Centered);
		}
	})
}

pub fn present_mode() -> PresentMode {
	Context::with(|ctx| ctx.graphics.present_mode())
}

pub fn max_queued_frames() -> u32 {
	Context::with(|ctx| ctx.graphics.max_queued_frames())
}

pub fn surface_format() -> TextureFormat {
	Context::with(|ctx| ctx.graphics.surface_format())
}

pub fn current_render_target_size() -> UVec2 {
	Context::with(|ctx| ctx.graphics.current_render_target_size())
}

pub fn set_present_mode(present_mode: PresentMode) {
	Context::with_mut(|ctx| ctx.graphics.set_present_mode(present_mode));
}

pub fn set_max_queued_frames(frames: u32) {
	Context::with_mut(|ctx| ctx.graphics.set_max_queued_frames(frames));
}

pub fn supported_sample_counts() -> Vec<u32> {
	Context::with(|ctx| ctx.graphics.supported_sample_counts.clone())
}

pub fn set_clear_color(color: impl Into<LinSrgb>) {
	let color = color.into();
	Context::with_mut(|ctx| ctx.graphics.clear_color = color);
}

pub fn push(push: impl Into<Push>) -> OnDrop {
	let push = push.into();
	Context::with_mut(|ctx| ctx.graphics.push_graphics_state(push.clone()));
	OnDrop
}

pub fn push_translation_2d(translation: impl Into<Vec2>) -> OnDrop {
	push(Mat4::from_translation(translation.into().extend(0.0)))
}

pub fn push_translation_3d(translation: impl Into<Vec3>) -> OnDrop {
	push(Mat4::from_translation(translation.into()))
}

pub fn push_translation_x(translation: f32) -> OnDrop {
	push(Mat4::from_translation(Vec3::new(translation, 0.0, 0.0)))
}

pub fn push_translation_y(translation: f32) -> OnDrop {
	push(Mat4::from_translation(Vec3::new(0.0, translation, 0.0)))
}

pub fn push_translation_z(translation: f32) -> OnDrop {
	push(Mat4::from_translation(Vec3::new(0.0, 0.0, translation)))
}

pub fn push_scale_2d(scale: impl Into<Vec2>) -> OnDrop {
	push(Mat4::from_scale(scale.into().extend(0.0)))
}

pub fn push_scale_3d(scale: impl Into<Vec3>) -> OnDrop {
	push(Mat4::from_scale(scale.into()))
}

pub fn push_scale_x(scale: f32) -> OnDrop {
	push(Mat4::from_scale(Vec3::new(scale, 1.0, 1.0)))
}

pub fn push_scale_y(scale: f32) -> OnDrop {
	push(Mat4::from_scale(Vec3::new(1.0, scale, 1.0)))
}

pub fn push_scale_z(scale: f32) -> OnDrop {
	push(Mat4::from_scale(Vec3::new(1.0, 1.0, scale)))
}

pub fn push_rotation_x(rotation: f32) -> OnDrop {
	push(Mat4::from_rotation_x(rotation))
}

pub fn push_rotation_y(rotation: f32) -> OnDrop {
	push(Mat4::from_rotation_y(rotation))
}

pub fn push_rotation_z(rotation: f32) -> OnDrop {
	push(Mat4::from_rotation_z(rotation))
}

/// Returns `true` if the given keyboard key is currently held down.
pub fn is_key_down(scancode: Scancode) -> bool {
	Context::with(|ctx| {
		ctx.event_pump
			.keyboard_state()
			.is_scancode_pressed(scancode.into())
	})
	/* && !ctx.egui_wants_keyboard_input */
}

/// Returns `true` if the given mouse button is currently held down.
pub fn is_mouse_button_down(mouse_button: MouseButton) -> bool {
	Context::with(|ctx| {
		ctx.event_pump
			.mouse_state()
			.is_mouse_button_pressed(mouse_button.into())
	})
	/* && !ctx.egui_wants_mouse_input */
}

/// Returns the current mouse position (in pixels, relative to the top-left
/// corner of the window).
pub fn mouse_position() -> IVec2 {
	Context::with(|ctx| {
		let mouse_state = ctx.event_pump.mouse_state();
		vec2(mouse_state.x(), mouse_state.y()).as_ivec2()
	})
}

/// Gets the gamepad with the given index if it's connected.
pub fn gamepad(index: u32) -> Option<Gamepad> {
	Context::with(|ctx| ctx.gamepad.open(index).map(Gamepad).ok())
}

/// Returns the average duration of a frame over the past 30 frames.
pub fn average_frame_time() -> Duration {
	Context::with(|ctx| ctx.frame_time_tracker.average())
}

/// Returns the current frames per second the game is running at.
pub fn fps() -> f32 {
	1.0 / average_frame_time().as_secs_f32()
}

/// Quits the game.
pub fn quit() {
	Context::with_mut(|ctx| ctx.should_quit = true);
}
