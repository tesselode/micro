pub(crate) mod graphics;

use std::{
	collections::HashMap,
	ops::{Deref, DerefMut},
	time::{Duration, Instant},
};

use glam::{IVec2, Mat4, UVec2, Vec2, Vec3, vec2};
use graphics::GraphicsContext;
use palette::LinSrgb;
use sdl2::{
	EventPump, GameControllerSubsystem, IntegerOrSdlError, Sdl, VideoSubsystem,
	video::{FullscreenType, Window, WindowPos},
};
use wgpu::{Features, PresentMode, TextureFormat};

use crate::{
	App, Event, FrameTimeTracker, SdlError,
	egui_integration::{draw_egui_output, egui_raw_input, egui_took_sdl2_event},
	graphics::{GraphicsPipeline, Vertex2d, drawable::Drawable},
	input::{Gamepad, MouseButton, Scancode},
	window::{WindowMode, build_window},
};

pub fn run<S, F, E>(settings: ContextSettings, mut app_constructor: F) -> Result<(), E>
where
	S: App<Error = E>,
	F: FnMut(&mut Context) -> Result<S, E>,
{
	let sdl = sdl2::init().expect("error initializing SDL");
	let video = sdl.video().expect("error initializing video subsystem");
	let controller = sdl
		.game_controller()
		.expect("error initializing controller subsystem");
	let window = build_window(&video, &settings);
	let event_pump = sdl.event_pump().expect("error creating event pump");
	let graphics = GraphicsContext::new(
		&window,
		settings.present_mode,
		settings.required_graphics_features,
	);

	let mut ctx = Context {
		_sdl: sdl,
		video,
		window,
		controller,
		event_pump,
		egui_wants_keyboard_input: false,
		egui_wants_mouse_input: false,
		frame_time_tracker: FrameTimeTracker::new(),
		graphics,
		should_quit: false,
	};
	let egui_ctx = egui::Context::default();
	let mut egui_textures = HashMap::new();
	let mut app = app_constructor(&mut ctx)?;

	let mut last_update_time = Instant::now();

	loop {
		// measure and record delta time
		let now = Instant::now();
		let delta_time = now - last_update_time;
		last_update_time = now;
		ctx.frame_time_tracker.record(delta_time);

		// poll for events
		let span = tracy_client::span!("poll events");
		let mut events = ctx.event_pump.poll_iter().collect::<Vec<_>>();
		drop(span);

		// create egui UI
		let span = tracy_client::span!("create egui UI");
		let egui_input = egui_raw_input(&ctx, &events, delta_time);
		egui_ctx.begin_pass(egui_input);
		app.debug_ui(&mut ctx, &egui_ctx)?;
		let egui_output = egui_ctx.end_pass();
		drop(span);

		// dispatch events to state
		let span = tracy_client::span!("dispatch events");
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
			let dpi_scaling = ctx.window_size().y as f32 / ctx.logical_window_size().y as f32;
			app.event(&mut ctx, event.transform_mouse_events(dpi_scaling))?;
		}
		drop(span);
		ctx.egui_wants_keyboard_input = egui_ctx.wants_keyboard_input();
		ctx.egui_wants_mouse_input = egui_ctx.wants_pointer_input();

		// update state
		let span = tracy_client::span!("update");
		app.update(&mut ctx, delta_time)?;
		drop(span);

		// draw state and egui UI
		let span = tracy_client::span!("draw");

		drop(span);
		app.draw(&mut ctx)?;
		let span = tracy_client::span!("draw egui UI");
		draw_egui_output(&mut ctx, &egui_ctx, egui_output, &mut egui_textures);
		drop(span);
		ctx.graphics.present();

		tracy_client::frame_mark();
		// ctx.graphics.record_queries();

		if ctx.should_quit {
			break;
		}
	}

	Ok(())
}

pub struct Context {
	_sdl: Sdl,
	pub(crate) video: VideoSubsystem,
	pub(crate) controller: GameControllerSubsystem,
	pub(crate) event_pump: EventPump,
	pub(crate) egui_wants_keyboard_input: bool,
	pub(crate) egui_wants_mouse_input: bool,
	// `graphics` needs to be before `window`, since it holds
	// a `Surface` that must be dropped before the `Window`
	pub(crate) graphics: GraphicsContext,
	pub(crate) window: Window,
	pub(crate) frame_time_tracker: FrameTimeTracker,
	pub(crate) should_quit: bool,
}

impl Context {
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

	pub fn present_mode(&self) -> PresentMode {
		self.graphics.present_mode()
	}

	pub fn surface_format(&self) -> TextureFormat {
		self.graphics.surface_format()
	}

	pub fn set_present_mode(&mut self, present_mode: PresentMode) {
		self.graphics.set_present_mode(present_mode);
	}

	pub fn supported_sample_counts(&self) -> &[u32] {
		&self.graphics.supported_sample_counts
	}

	pub fn current_render_target_size(&self) -> UVec2 {
		self.graphics.current_render_target_size()
	}

	pub fn set_clear_color(&mut self, color: impl Into<LinSrgb>) {
		self.graphics.clear_color = color.into();
	}

	pub fn default_graphics_pipeline(&self) -> GraphicsPipeline {
		self.graphics.default_graphics_pipeline.clone()
	}

	/// Creates a scope where all drawing operations have the given transform
	/// applied.
	///
	/// Calls to `push_transform` can be nested.
	pub fn push_transform(&mut self, transform: impl Into<Mat4>) -> OnDrop<'_> {
		let transform = transform.into();
		self.graphics.transform_stack.push(transform);
		OnDrop {
			ctx: self,
			pop: Pop::Transform,
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

	pub fn push_stencil_reference(&mut self, stencil_reference: u8) -> OnDrop<'_> {
		self.graphics
			.stencil_reference_stack
			.push(stencil_reference);
		OnDrop {
			ctx: self,
			pop: Pop::StencilReference,
		}
	}

	pub fn draw(&mut self, drawable: impl Drawable<Vertex = Vertex2d>) {
		drawable.draw(self, self.graphics.default_graphics_pipeline.raw());
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
		vec2(
			mouse_state.x() as f32 * dpi_scaling,
			mouse_state.y() as f32 * dpi_scaling,
		)
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextSettings {
	pub window_title: String,
	pub window_mode: WindowMode,
	pub resizable: bool,
	pub present_mode: PresentMode,
	pub required_graphics_features: Features,
}

impl Default for ContextSettings {
	fn default() -> Self {
		Self {
			window_title: "Game".into(),
			window_mode: WindowMode::default(),
			resizable: false,
			present_mode: PresentMode::AutoVsync,
			required_graphics_features: Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
		}
	}
}

#[must_use]
pub struct OnDrop<'a> {
	ctx: &'a mut Context,
	pop: Pop,
}

impl Drop for OnDrop<'_> {
	fn drop(&mut self) {
		match self.pop {
			Pop::Transform => {
				self.ctx.graphics.transform_stack.pop();
			}
			Pop::StencilReference => {
				self.ctx.graphics.stencil_reference_stack.pop();
			}
		}
	}
}

impl Deref for OnDrop<'_> {
	type Target = Context;

	fn deref(&self) -> &Self::Target {
		self.ctx
	}
}

impl DerefMut for OnDrop<'_> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.ctx
	}
}

enum Pop {
	Transform,
	StencilReference,
}
