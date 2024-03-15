pub mod animation;
mod context;
mod egui_integration;
mod error;
mod event;
pub mod graphics;
pub mod input;
#[cfg(feature = "ldtk")]
pub mod ldtk;
mod log;
mod macros;
pub mod math;
mod offset_and_count;
#[cfg(feature = "resource_management")]
pub mod resource;
mod state;
pub mod time;
pub mod tween;
mod window;

pub use context::{run, ContextSettings, OnDrop, ScalingMode};
pub use error::*;
pub use event::*;
pub use offset_and_count::*;
pub use state::*;
pub use window::*;

use std::time::Duration;

use context::Context;
use glam::{IVec2, Mat4, UVec2, Vec2, Vec3};
use glow::HasContext;
use graphics::{Camera3d, StencilAction, StencilTest};
use input::{Gamepad, MouseButton, Scancode};
use palette::LinSrgba;
use sdl2::{
	video::{FullscreenType, SwapInterval, WindowPos},
	IntegerOrSdlError,
};

/// Gets the drawable size of the window (in pixels).
pub fn window_size() -> UVec2 {
	let (width, height) = Context::with(|ctx| ctx.window.size());
	UVec2::new(width, height)
}

/// Returns the current window mode (windowed or fullscreen).
pub fn window_mode() -> WindowMode {
	match Context::with(|ctx| ctx.window.fullscreen_state()) {
		FullscreenType::Off => WindowMode::Windowed {
			size: window_size(),
		},
		FullscreenType::True => WindowMode::Fullscreen,
		FullscreenType::Desktop => WindowMode::Fullscreen,
	}
}

/// Returns the current swap interval (vsync on or off).
pub fn swap_interval() -> SwapInterval {
	Context::with(|ctx| ctx.video.gl_get_swap_interval())
}

/// Returns the resolution of the monitor the window is on.
pub fn monitor_resolution() -> Result<UVec2, SdlError> {
	let display_index = Context::with(|ctx| ctx.window.display_index())?;
	let display_mode = Context::with(|ctx| ctx.video.desktop_display_mode(display_index))?;
	Ok(UVec2::new(display_mode.w as u32, display_mode.h as u32))
}

/// Sets the window mode (windowed or fullscreen).
pub fn set_window_mode(window_mode: WindowMode) -> Result<(), SdlError> {
	match window_mode {
		WindowMode::Fullscreen => {
			Context::with_mut(|ctx| ctx.window.set_fullscreen(FullscreenType::Desktop))?;
		}
		WindowMode::Windowed { size } => {
			Context::with_mut(|ctx| -> Result<(), SdlError> {
				ctx.window.set_fullscreen(FullscreenType::Off)?;
				ctx.window
					.set_size(size.x, size.y)
					.map_err(|err| match err {
						IntegerOrSdlError::IntegerOverflows(_, _) => panic!("integer overflow"),
						IntegerOrSdlError::SdlError(err) => SdlError(err),
					})?;
				ctx.window
					.set_position(WindowPos::Centered, WindowPos::Centered);
				Ok(())
			})?;
		}
	}
	Ok(())
}

/// Sets the swap interval (vsync on or off).
pub fn set_swap_interval(swap_interval: SwapInterval) -> Result<(), SdlError> {
	Context::with(|ctx| ctx.video.gl_set_swap_interval(swap_interval))?;
	Ok(())
}

/// Clears the window surface to the given color. Also clears the stencil buffer and depth buffer.
pub fn clear(color: impl Into<LinSrgba>) {
	let color = color.into();
	unsafe {
		Context::with(|ctx| {
			ctx.graphics
				.gl
				.clear_color(color.red, color.green, color.blue, color.alpha);
			ctx.graphics.gl.stencil_mask(0xFF);
			ctx.graphics.gl.clear_stencil(0);
			ctx.graphics
				.gl
				.clear(glow::COLOR_BUFFER_BIT | glow::STENCIL_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
			ctx.graphics.gl.stencil_mask(0x00);
		});
	}
}

/// Clears the stencil buffer.
pub fn clear_stencil() {
	unsafe {
		Context::with(|ctx| {
			ctx.graphics.gl.stencil_mask(0xFF);
			ctx.graphics.gl.clear_stencil(0);
			ctx.graphics.gl.clear(glow::STENCIL_BUFFER_BIT);
			ctx.graphics.gl.stencil_mask(0x00);
		});
	}
}

/// Clears the depth buffer.
pub fn clear_depth_buffer() {
	unsafe {
		Context::with(|ctx| ctx.graphics.gl.clear(glow::DEPTH_BUFFER_BIT));
	}
}

/// Creates a scope where all drawing operations have the given transform
/// applied.
///
/// Calls to `transform` can be nested.
///
/// ```rust
/// use glam::{Mat4, Vec3};
///
/// # fn fake(ctx: &mut micro::Context) {
/// {
///     let _scope = ctx.transform(Mat4::from_scale(Vec3::new(2.0, 2.0, 1.0)));
///     // the next drawing operations will have a transform applied
///     // ...
/// }
/// // the following drawing operations will be back to normal
/// # }
/// ```
pub fn push_transform(transform: impl Into<Mat4>) -> OnDrop {
	let transform = transform.into();
	Context::with_mut(|ctx| ctx.graphics.transform_stack.push(transform));
	OnDrop {
		on_drop: || {
			Context::with_mut(|ctx| ctx.graphics.transform_stack.pop());
		},
	}
}

pub fn push_translation_2d(translation: impl Into<Vec2>) -> OnDrop {
	push_transform(Mat4::from_translation(translation.into().extend(0.0)))
}

pub fn push_translation_3d(translation: impl Into<Vec3>) -> OnDrop {
	push_transform(Mat4::from_translation(translation.into()))
}

pub fn push_translation_x(translation: f32) -> OnDrop {
	push_transform(Mat4::from_translation(Vec3::new(translation, 0.0, 0.0)))
}

pub fn push_translation_y(translation: f32) -> OnDrop {
	push_transform(Mat4::from_translation(Vec3::new(0.0, translation, 0.0)))
}

pub fn push_translation_z(translation: f32) -> OnDrop {
	push_transform(Mat4::from_translation(Vec3::new(0.0, 0.0, translation)))
}

pub fn push_scale_2d(scale: impl Into<Vec2>) -> OnDrop {
	push_transform(Mat4::from_scale(scale.into().extend(0.0)))
}

pub fn push_scale_3d(scale: impl Into<Vec3>) -> OnDrop {
	push_transform(Mat4::from_scale(scale.into()))
}

pub fn push_rotation_x(rotation: f32) -> OnDrop {
	push_transform(Mat4::from_rotation_x(rotation))
}

pub fn push_rotation_y(rotation: f32) -> OnDrop {
	push_transform(Mat4::from_rotation_y(rotation))
}

pub fn push_rotation_z(rotation: f32) -> OnDrop {
	push_transform(Mat4::from_rotation_z(rotation))
}

/// Creates a scope where all drawing operations use the given 3D camera.
///
/// This also turns on the depth buffer.
///
/// ```rust
/// use micro::graphics::Camera3d;
/// use glam::Vec3;
///
/// # fn fake(ctx: &mut micro::Context) {
/// {
///     let _scope = ctx.use_3d_camera(
///         Camera3d::perspective(90.0, 16.0 / 9.0, 0.01..=1000.0, Vec3::ZERO, Vec3::new(0.0, 0.0, 1.0))
///     );
///     // the next drawing operations will use the 3d camera
///     // ...
/// }
/// // the following drawing operations will be back to normal
/// # }
/// ```
pub fn use_3d_camera(camera: Camera3d) -> OnDrop {
	let transform = camera.transform();
	Context::with_mut(|ctx| {
		ctx.graphics.transform_stack.push(transform);
		unsafe {
			ctx.graphics.gl.enable(glow::DEPTH_TEST);
		}
	});
	OnDrop {
		on_drop: || {
			Context::with_mut(|ctx| {
				ctx.graphics.transform_stack.pop();
				unsafe {
					ctx.graphics.gl.disable(glow::DEPTH_TEST);
				}
			});
		},
	}
}

pub fn write_to_stencil(action: StencilAction) -> OnDrop {
	unsafe {
		Context::with(|ctx| {
			ctx.graphics.gl.color_mask(false, false, false, false);
			ctx.graphics.gl.enable(glow::STENCIL_TEST);
			let op = action.as_glow_stencil_op();
			ctx.graphics.gl.stencil_op(glow::KEEP, glow::KEEP, op);
			let reference = match action {
				StencilAction::Replace(value) => value,
				_ => 0,
			};
			ctx.graphics
				.gl
				.stencil_func(glow::ALWAYS, reference.into(), 0xFF);
			ctx.graphics.gl.stencil_mask(0xFF);
			ctx.graphics.gl.depth_mask(false);
		});
	}
	OnDrop {
		on_drop: || unsafe {
			Context::with(|ctx| {
				ctx.graphics.gl.color_mask(true, true, true, true);
				ctx.graphics.gl.disable(glow::STENCIL_TEST);
				ctx.graphics.gl.depth_mask(true);
			});
		},
	}
}

pub fn use_stencil(test: StencilTest, reference: u8) -> OnDrop {
	unsafe {
		Context::with(|ctx| {
			ctx.graphics.gl.enable(glow::STENCIL_TEST);
			ctx.graphics
				.gl
				.stencil_op(glow::KEEP, glow::KEEP, glow::KEEP);
			ctx.graphics
				.gl
				.stencil_func(test.as_glow_stencil_func(), reference.into(), 0xFF);
			ctx.graphics.gl.stencil_mask(0x00);
		});
	}
	OnDrop {
		on_drop: || unsafe {
			Context::with(|ctx| ctx.graphics.gl.disable(glow::STENCIL_TEST));
		},
	}
}

/// Returns `true` if the given keyboard key is currently held down.
pub fn is_key_down(scancode: Scancode) -> bool {
	Context::with(|ctx| {
		ctx.event_pump
			.keyboard_state()
			.is_scancode_pressed(scancode.into())
			&& !ctx.egui_wants_keyboard_input
	})
}

/// Returns `true` if the given mouse button is currently held down.
pub fn is_mouse_button_down(mouse_button: MouseButton) -> bool {
	Context::with(|ctx| {
		ctx.event_pump
			.mouse_state()
			.is_mouse_button_pressed(mouse_button.into())
			&& !ctx.egui_wants_mouse_input
	})
}

/// Returns the current mouse position (in pixels, relative to the top-left
/// corner of the window).
pub fn mouse_position() -> IVec2 {
	Context::with(|ctx| {
		let mouse_state = ctx.event_pump.mouse_state();
		let untransformed = IVec2::new(mouse_state.x(), mouse_state.y());
		ctx.scaling_mode
			.transform_affine2()
			.inverse()
			.transform_point2(untransformed.as_vec2())
			.as_ivec2()
	})
}

/// Gets the game controller with the given index if it's connected.
pub fn game_controller(index: u32) -> Option<Gamepad> {
	Context::with(|ctx| match ctx.controller.open(index) {
		Ok(controller) => Some(Gamepad(controller)),
		Err(error) => match error {
			IntegerOrSdlError::IntegerOverflows(_, _) => {
				panic!("integer overflow when getting controller")
			}
			IntegerOrSdlError::SdlError(_) => None,
		},
	})
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
