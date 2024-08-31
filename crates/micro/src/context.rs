pub(crate) mod graphics;

use std::{
	collections::HashMap,
	fmt::Debug,
	ops::{Deref, DerefMut},
	time::{Duration, Instant},
};

use glam::{vec2, Affine2, IVec2, Mat4, UVec2, Vec2, Vec3};
use glow::HasContext;
use palette::LinSrgba;
use sdl2::{
	video::{FullscreenType, GLProfile, SwapInterval, Window, WindowPos},
	EventPump, GameControllerSubsystem, IntegerOrSdlError, Sdl, VideoSubsystem,
};

use crate::{
	build_window,
	color::ColorConstants,
	egui_integration::{draw_egui_output, egui_raw_input, egui_took_sdl2_event},
	graphics::{Camera3d, Canvas, CanvasSettings, Msaa, StencilAction, StencilTest},
	input::{Gamepad, MouseButton, Scancode},
	log_if_err,
	time::FrameTimeTracker,
	window::WindowMode,
	App, Event, SdlError,
};

use self::graphics::GraphicsContext;

/// Runs the game. Call this in your `main` function.
pub fn run<S, F, E>(settings: ContextSettings, app_constructor: F)
where
	S: App<E>,
	F: FnMut(&mut Context) -> Result<S, E>,
	E: Debug,
{
	log_if_err!(run_inner(settings, app_constructor));
}

fn run_inner<S, F, E>(settings: ContextSettings, mut app_constructor: F) -> Result<(), E>
where
	S: App<E>,
	F: FnMut(&mut Context) -> Result<S, E>,
{
	// create contexts and resources
	let mut ctx = Context::new(&settings);
	let egui_ctx = egui::Context::default();
	let mut egui_textures = HashMap::new();
	let mut app = app_constructor(&mut ctx)?;
	let main_canvas = if let ScalingMode::Pixelated {
		base_size: size, ..
	} = settings.scaling_mode
	{
		Some(Canvas::new(&mut ctx, size, CanvasSettings::default()))
	} else {
		None
	};

	let mut last_update_time = Instant::now();

	loop {
		// measure and record delta time
		let now = Instant::now();
		let delta_time = now - last_update_time;
		last_update_time = now;
		ctx.frame_time_tracker.record(delta_time);

		// poll for events
		let mut events = ctx.event_pump.poll_iter().collect::<Vec<_>>();

		// create egui UI
		let egui_input = egui_raw_input(&ctx, &events, delta_time);
		egui_ctx.begin_frame(egui_input);
		app.debug_ui(&mut ctx, &egui_ctx)?;
		let egui_output = egui_ctx.end_frame();

		// dispatch events to state
		for event in events
			.drain(..)
			.filter(|event| !egui_took_sdl2_event(&egui_ctx, event))
			.filter_map(Event::from_sdl2_event)
		{
			match event {
				Event::WindowSizeChanged(size) => ctx.graphics.resize(size),
				Event::Exited => ctx.should_quit = true,
				_ => {}
			}
			let transform = ctx.scaling_mode.transform_affine2(&ctx).inverse();
			let dpi_scaling = ctx.window_size().y as f32 / ctx.logical_window_size().y as f32;
			app.event(
				&mut ctx,
				event.transform_mouse_events(transform, dpi_scaling),
			)?;
		}
		ctx.egui_wants_keyboard_input = egui_ctx.wants_keyboard_input();
		ctx.egui_wants_mouse_input = egui_ctx.wants_pointer_input();

		// update state
		app.update(&mut ctx, delta_time)?;

		// draw state and egui UI
		if let Some(main_canvas) = &main_canvas {
			ctx.clear(LinSrgba::BLACK);
			{
				let ctx = &mut main_canvas.render_to(&mut ctx);
				app.draw(ctx)?;
			}
			let transform = ctx.scaling_mode.transform_mat4(&ctx);
			main_canvas.transformed(transform).draw(&mut ctx);
		} else {
			let ctx = &mut ctx.push_transform(ctx.scaling_mode.transform_mat4(&ctx));
			app.draw(ctx)?;
		}
		draw_egui_output(&mut ctx, &egui_ctx, egui_output, &mut egui_textures);
		ctx.window.gl_swap_window();

		ctx.graphics.delete_unused_resources();

		if ctx.should_quit {
			break;
		}
	}
	Ok(())
}

/// The main interface between your game code and functionality provided
/// by the framework.
pub struct Context {
	_sdl: Sdl,
	pub(crate) video: VideoSubsystem,
	pub(crate) window: Window,
	pub(crate) controller: GameControllerSubsystem,
	pub(crate) event_pump: EventPump,
	pub(crate) egui_wants_keyboard_input: bool,
	pub(crate) egui_wants_mouse_input: bool,
	pub(crate) graphics: GraphicsContext,
	pub(crate) scaling_mode: ScalingMode,
	pub(crate) frame_time_tracker: FrameTimeTracker,
	pub(crate) should_quit: bool,
}

impl Context {
	pub fn max_msaa_level(&self) -> Msaa {
		unsafe { self.graphics.gl.get_parameter_i32(glow::MAX_SAMPLES) }.into()
	}

	/// Gets the drawable size of the window (in pixels).
	pub fn window_size(&self) -> UVec2 {
		let (width, height) = self.window.drawable_size();
		UVec2::new(width, height)
	}

	/// Gets the size of the window (in points).
	pub fn logical_window_size(&self) -> UVec2 {
		let (width, height) = self.window.size();
		UVec2::new(width, height)
	}

	/// Returns the current window mode (windowed or fullscreen).
	pub fn window_mode(&self) -> WindowMode {
		match self.window.fullscreen_state() {
			FullscreenType::Off => WindowMode::Windowed {
				size: self.window_size(),
			},
			FullscreenType::True => WindowMode::Fullscreen,
			FullscreenType::Desktop => WindowMode::Fullscreen,
		}
	}

	/// Returns the current swap interval (vsync on or off).
	pub fn swap_interval(&self) -> SwapInterval {
		self.video.gl_get_swap_interval()
	}

	/// Returns the resolution of the monitor the window is on.
	pub fn monitor_resolution(&self) -> Result<UVec2, SdlError> {
		let display_index = self.window.display_index()?;
		let display_mode = self.video.desktop_display_mode(display_index)?;
		Ok(UVec2::new(display_mode.w as u32, display_mode.h as u32))
	}

	/// Sets the window mode (windowed or fullscreen).
	pub fn set_window_mode(&mut self, window_mode: WindowMode) -> Result<(), SdlError> {
		match window_mode {
			WindowMode::Fullscreen => {
				self.window.set_fullscreen(FullscreenType::Desktop)?;
			}
			WindowMode::Windowed { size } => {
				self.window.set_fullscreen(FullscreenType::Off)?;
				self.window
					.set_size(size.x, size.y)
					.map_err(|err| match err {
						IntegerOrSdlError::IntegerOverflows(_, _) => panic!("integer overflow"),
						IntegerOrSdlError::SdlError(err) => SdlError(err),
					})?;
				self.window
					.set_position(WindowPos::Centered, WindowPos::Centered);
			}
		}
		Ok(())
	}

	/// Sets the swap interval (vsync on or off).
	pub fn set_swap_interval(&self, swap_interval: SwapInterval) -> Result<(), SdlError> {
		self.video.gl_set_swap_interval(swap_interval)?;
		Ok(())
	}

	/// Clears the window surface to the given color. Also clears the stencil buffer and depth buffer.
	pub fn clear(&self, color: impl Into<LinSrgba>) {
		let color = color.into();
		unsafe {
			self.graphics
				.gl
				.clear_color(color.red, color.green, color.blue, color.alpha);
			self.graphics.gl.stencil_mask(0xFF);
			self.graphics.gl.clear_stencil(0);
			self.graphics
				.gl
				.clear(glow::COLOR_BUFFER_BIT | glow::STENCIL_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
			self.graphics.gl.stencil_mask(0x00);
		}
	}

	/// Clears the stencil buffer.
	pub fn clear_stencil(&self) {
		unsafe {
			self.graphics.gl.stencil_mask(0xFF);
			self.graphics.gl.clear_stencil(0);
			self.graphics.gl.clear(glow::STENCIL_BUFFER_BIT);
			self.graphics.gl.stencil_mask(0x00);
		}
	}

	/// Clears the depth buffer.
	pub fn clear_depth_buffer(&self) {
		unsafe {
			self.graphics.gl.clear(glow::DEPTH_BUFFER_BIT);
		}
	}

	/// Creates a scope where all drawing operations have the given transform
	/// applied.
	///
	/// Calls to `push_transform` can be nested.
	pub fn push_transform(&mut self, transform: impl Into<Mat4>) -> OnDrop {
		let transform = transform.into();
		self.graphics.transform_stack.push(transform);
		OnDrop {
			ctx: self,
			action: OnDropAction::PopTransform,
		}
	}

	pub fn push_translation_2d(&mut self, translation: impl Into<Vec2>) -> OnDrop<'_> {
		self.push_transform(Mat4::from_translation(translation.into().extend(0.0)))
	}

	pub fn push_translation_3d(&mut self, translation: impl Into<Vec3>) -> OnDrop<'_> {
		self.push_transform(Mat4::from_translation(translation.into()))
	}

	pub fn push_translation_x(&mut self, translation: f32) -> OnDrop<'_> {
		self.push_transform(Mat4::from_translation(Vec3::new(translation, 0.0, 0.0)))
	}

	pub fn push_translation_y(&mut self, translation: f32) -> OnDrop<'_> {
		self.push_transform(Mat4::from_translation(Vec3::new(0.0, translation, 0.0)))
	}

	pub fn push_translation_z(&mut self, translation: f32) -> OnDrop<'_> {
		self.push_transform(Mat4::from_translation(Vec3::new(0.0, 0.0, translation)))
	}

	pub fn push_scale_2d(&mut self, scale: impl Into<Vec2>) -> OnDrop<'_> {
		self.push_transform(Mat4::from_scale(scale.into().extend(0.0)))
	}

	pub fn push_scale_3d(&mut self, scale: impl Into<Vec3>) -> OnDrop<'_> {
		self.push_transform(Mat4::from_scale(scale.into()))
	}

	pub fn push_scale_x(&mut self, scale: f32) -> OnDrop<'_> {
		self.push_transform(Mat4::from_scale(Vec3::new(scale, 1.0, 1.0)))
	}

	pub fn push_scale_y(&mut self, scale: f32) -> OnDrop<'_> {
		self.push_transform(Mat4::from_scale(Vec3::new(1.0, scale, 1.0)))
	}

	pub fn push_scale_z(&mut self, scale: f32) -> OnDrop<'_> {
		self.push_transform(Mat4::from_scale(Vec3::new(1.0, 1.0, scale)))
	}

	pub fn push_rotation_x(&mut self, rotation: f32) -> OnDrop<'_> {
		self.push_transform(Mat4::from_rotation_x(rotation))
	}

	pub fn push_rotation_y(&mut self, rotation: f32) -> OnDrop<'_> {
		self.push_transform(Mat4::from_rotation_y(rotation))
	}

	pub fn push_rotation_z(&mut self, rotation: f32) -> OnDrop<'_> {
		self.push_transform(Mat4::from_rotation_z(rotation))
	}

	/// Creates a scope where all drawing operations use the given 3D camera.
	///
	/// This also turns on the depth buffer.
	pub fn use_3d_camera(&mut self, camera: Camera3d) -> OnDrop {
		let transform = camera.transform(self);
		self.graphics.transform_stack.push(transform);
		unsafe {
			self.graphics.gl.enable(glow::DEPTH_TEST);
		}
		OnDrop {
			ctx: self,
			action: OnDropAction::StopUsingCamera,
		}
	}

	pub fn write_to_stencil(&mut self, action: StencilAction) -> OnDrop {
		unsafe {
			self.graphics.gl.color_mask(false, false, false, false);
			self.graphics.gl.enable(glow::STENCIL_TEST);
			let op = action.as_glow_stencil_op();
			self.graphics.gl.stencil_op(glow::KEEP, glow::KEEP, op);
			let reference = match action {
				StencilAction::Replace(value) => value,
				_ => 0,
			};
			self.graphics
				.gl
				.stencil_func(glow::ALWAYS, reference.into(), 0xFF);
			self.graphics.gl.stencil_mask(0xFF);
			self.graphics.gl.depth_mask(false);
		}
		OnDrop {
			ctx: self,
			action: OnDropAction::StopWritingToStencil,
		}
	}

	pub fn use_stencil(&mut self, test: StencilTest, reference: u8) -> OnDrop {
		unsafe {
			self.graphics.gl.enable(glow::STENCIL_TEST);
			self.graphics
				.gl
				.stencil_op(glow::KEEP, glow::KEEP, glow::KEEP);
			self.graphics
				.gl
				.stencil_func(test.as_glow_stencil_func(), reference.into(), 0xFF);
			self.graphics.gl.stencil_mask(0x00);
		}
		OnDrop {
			ctx: self,
			action: OnDropAction::StopUsingStencil,
		}
	}

	/// Returns `true` if the given keyboard key is currently held down.
	pub fn is_key_down(&self, scancode: Scancode) -> bool {
		self.event_pump
			.keyboard_state()
			.is_scancode_pressed(scancode.into())
			&& !self.egui_wants_keyboard_input
	}

	/// Returns `true` if the given mouse button is currently held down.
	pub fn is_mouse_button_down(&self, mouse_button: MouseButton) -> bool {
		self.event_pump
			.mouse_state()
			.is_mouse_button_pressed(mouse_button.into())
			&& !self.egui_wants_mouse_input
	}

	/// Returns the current mouse position (in pixels, relative to the top-left
	/// corner of the window).
	pub fn mouse_position(&self) -> IVec2 {
		let mouse_state = self.event_pump.mouse_state();
		let dpi_scaling = self.window_size().y as f32 / self.logical_window_size().y as f32;
		let untransformed = vec2(
			mouse_state.x() as f32 * dpi_scaling,
			mouse_state.y() as f32 * dpi_scaling,
		)
		.as_ivec2();
		self.scaling_mode
			.transform_affine2(self)
			.inverse()
			.transform_point2(untransformed.as_vec2())
			.as_ivec2()
	}

	/// Gets the gamepad with the given index if it's connected.
	pub fn gamepad(&self, index: u32) -> Option<Gamepad> {
		match self.controller.open(index) {
			Ok(controller) => Some(Gamepad(controller)),
			Err(error) => match error {
				IntegerOrSdlError::IntegerOverflows(_, _) => {
					panic!("integer overflow when getting controller")
				}
				IntegerOrSdlError::SdlError(_) => None,
			},
		}
	}

	/// Returns the average duration of a frame over the past 30 frames.
	pub fn average_frame_time(&self) -> Duration {
		self.frame_time_tracker.average()
	}

	/// Returns the current frames per second the game is running at.
	pub fn fps(&self) -> f32 {
		1.0 / self.average_frame_time().as_secs_f32()
	}

	/// Quits the game.
	pub fn quit(&mut self) {
		self.should_quit = true;
	}

	fn new(settings: &ContextSettings) -> Self {
		let sdl = sdl2::init().expect("error initializing SDL");
		let video = sdl.video().expect("error initializing video subsystem");
		let controller = sdl
			.game_controller()
			.expect("error initializing controller subsystem");
		let gl_attr = video.gl_attr();
		gl_attr.set_context_profile(GLProfile::Core);
		gl_attr.set_context_version(3, 3);
		gl_attr.set_stencil_size(8);
		gl_attr.set_framebuffer_srgb_compatible(true);
		let window = build_window(&video, settings);
		let event_pump = sdl.event_pump().expect("error creating event pump");
		let graphics = GraphicsContext::new(&video, &window);
		video
			.gl_set_swap_interval(settings.swap_interval)
			.expect("error setting swap interval");
		Self {
			_sdl: sdl,
			video,
			window,
			controller,
			event_pump,
			egui_wants_keyboard_input: false,
			egui_wants_mouse_input: false,
			graphics,
			scaling_mode: settings.scaling_mode,
			frame_time_tracker: FrameTimeTracker::new(),
			should_quit: false,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextSettings {
	pub window_title: String,
	pub window_mode: WindowMode,
	pub resizable: bool,
	pub swap_interval: SwapInterval,
	pub scaling_mode: ScalingMode,
}

impl Default for ContextSettings {
	fn default() -> Self {
		Self {
			window_title: "Game".into(),
			window_mode: WindowMode::default(),
			resizable: false,
			swap_interval: SwapInterval::VSync,
			scaling_mode: ScalingMode::default(),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ScalingMode {
	#[default]
	None,
	Smooth {
		base_size: UVec2,
	},
	Pixelated {
		base_size: UVec2,
		integer_scale: bool,
	},
}

impl ScalingMode {
	pub(crate) fn transform_affine2(&self, ctx: &Context) -> Affine2 {
		match self {
			ScalingMode::None => Affine2::IDENTITY,
			ScalingMode::Smooth { base_size } => {
				let max_horizontal_scale = ctx.window_size().x as f32 / base_size.x as f32;
				let max_vertical_scale = ctx.window_size().y as f32 / base_size.y as f32;
				let scale = max_horizontal_scale.min(max_vertical_scale);
				Affine2::from_translation(ctx.window_size().as_vec2() / 2.0)
					* Affine2::from_scale(Vec2::splat(scale))
					* Affine2::from_translation(-base_size.as_vec2() / 2.0)
			}
			ScalingMode::Pixelated {
				base_size,
				integer_scale,
			} => {
				let max_horizontal_scale = ctx.window_size().x as f32 / base_size.x as f32;
				let max_vertical_scale = ctx.window_size().y as f32 / base_size.y as f32;
				let mut scale = max_horizontal_scale.min(max_vertical_scale);
				if *integer_scale {
					scale = scale.floor();
				}
				Affine2::from_translation((ctx.window_size().as_vec2() / 2.0).round())
					* Affine2::from_scale(Vec2::splat(scale))
					* Affine2::from_translation((-base_size.as_vec2() / 2.0).round())
			}
		}
	}

	fn transform_mat4(&self, ctx: &Context) -> Mat4 {
		match self {
			ScalingMode::None => Mat4::IDENTITY,
			ScalingMode::Smooth { base_size } => {
				let max_horizontal_scale = ctx.window_size().x as f32 / base_size.x as f32;
				let max_vertical_scale = ctx.window_size().y as f32 / base_size.y as f32;
				let scale = max_horizontal_scale.min(max_vertical_scale);
				Mat4::from_translation((ctx.window_size().as_vec2() / 2.0).extend(0.0))
					* Mat4::from_scale(Vec2::splat(scale).extend(1.0))
					* Mat4::from_translation((-base_size.as_vec2() / 2.0).extend(0.0))
			}
			ScalingMode::Pixelated {
				base_size,
				integer_scale,
			} => {
				let max_horizontal_scale = ctx.window_size().x as f32 / base_size.x as f32;
				let max_vertical_scale = ctx.window_size().y as f32 / base_size.y as f32;
				let mut scale = max_horizontal_scale.min(max_vertical_scale);
				if *integer_scale {
					scale = scale.floor();
				}
				Mat4::from_translation((ctx.window_size().as_vec2() / 2.0).extend(0.0))
					* Mat4::from_scale(Vec2::splat(scale).extend(1.0))
					* Mat4::from_translation((-base_size.as_vec2() / 2.0).extend(0.0))
			}
		}
	}
}

#[must_use]
pub struct OnDrop<'a> {
	ctx: &'a mut Context,
	action: OnDropAction,
}

impl<'a> Drop for OnDrop<'a> {
	fn drop(&mut self) {
		match self.action {
			OnDropAction::PopTransform => {
				self.ctx.graphics.transform_stack.pop();
			}
			OnDropAction::StopUsingCamera => {
				self.ctx.graphics.transform_stack.pop();
				unsafe {
					self.ctx.graphics.gl.disable(glow::DEPTH_TEST);
				}
			}
			OnDropAction::StopWritingToStencil => unsafe {
				self.ctx.graphics.gl.color_mask(true, true, true, true);
				self.ctx.graphics.gl.disable(glow::STENCIL_TEST);
				self.ctx.graphics.gl.depth_mask(true);
			},
			OnDropAction::StopUsingStencil => unsafe {
				self.ctx.graphics.gl.disable(glow::STENCIL_TEST);
			},
		}
	}
}

impl<'a> Deref for OnDrop<'a> {
	type Target = Context;

	fn deref(&self) -> &Self::Target {
		self.ctx
	}
}

impl<'a> DerefMut for OnDrop<'a> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.ctx
	}
}

enum OnDropAction {
	PopTransform,
	StopUsingCamera,
	StopWritingToStencil,
	StopUsingStencil,
}
