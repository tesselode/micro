pub(crate) mod graphics;
pub(crate) mod text;

mod push;

pub use push::*;

use std::{
	collections::HashMap,
	fmt::Debug,
	ops::{Deref, DerefMut},
	path::Path,
	time::{Duration, Instant},
};

use egui::{Align, Layout, TopBottomPanel};
use glam::{Mat4, UVec2, Vec2, Vec3, vec2};
use palette::{LinSrgb, WithAlpha};
use sdl3::{
	EventPump, GamepadSubsystem, IntegerOrSdlError,
	video::{FullscreenType, Window, WindowPos},
};
use wgpu::{Features, PresentMode, TextureFormat};

use crate::{
	App, Event, FrameTimeTracker, WindowMode, build_window,
	color::ColorConstants,
	context::graphics::GraphicsContext,
	egui_integration::{draw_egui_output, egui_raw_input, egui_took_sdl3_event},
	graphics::{Canvas, CanvasSettings, IntoScale2d, IntoScale3d, RenderToCanvasSettings},
	input::{Gamepad, MouseButton, Scancode},
	text::TextContext,
};

/// Starts a Micro application. The app constructor should return a value of a type
/// that implements [`App`].
pub fn run<A, F>(settings: ContextSettings, mut app_constructor: F) -> anyhow::Result<()>
where
	A: App,
	F: FnMut(&mut Context) -> anyhow::Result<A>,
{
	let sdl = sdl3::init().expect("error initializing SDL");
	let video = sdl.video().expect("error initializing video subsystem");
	let controller = sdl
		.gamepad()
		.expect("error initializing controller subsystem");
	let window = build_window(&video, &settings);
	video.text_input().start(&window);
	let event_pump = sdl.event_pump().expect("error creating event pump");
	let graphics = GraphicsContext::new(&window, &settings);
	let text = TextContext::new(&graphics);
	let main_canvas = settings.main_canvas.map(|settings| {
		Canvas::new_from_graphics_ctx(&graphics, settings.size, CanvasSettings::default())
	});

	let mut ctx = Context {
		window,
		gamepad: controller,
		event_pump,
		mouse_wheel_delta: Vec2::ZERO,
		egui_wants_keyboard_input: false,
		egui_wants_mouse_input: false,
		clear_color: LinSrgb::BLACK,
		main_canvas_size: settings.main_canvas.map(|settings| settings.size),
		integer_scaling_enabled: settings
			.main_canvas
			.map(|settings| settings.integer_scaling_enabled)
			.unwrap_or_default(),
		frame_time_tracker: FrameTimeTracker::new(),
		graphics,
		text,
		dev_tools_state: settings.dev_tools_mode.initial_state(),
		should_quit: false,
	};
	let egui_ctx = egui::Context::default();
	let mut egui_textures = HashMap::new();
	let mut app = app_constructor(&mut ctx)?;

	let mut last_update_time = Instant::now();

	loop {
		// get main canvas transform, if applicable
		// (used for events and drawing)
		let main_canvas_transform = main_canvas.as_ref().map(|canvas| {
			main_canvas_transform(
				canvas.size(),
				ctx.window_size(),
				ctx.integer_scaling_enabled,
			)
		});

		// measure and record delta time
		let now = Instant::now();
		let delta_time = now - last_update_time;
		last_update_time = now;
		ctx.frame_time_tracker.record(delta_time);

		// poll for events
		let span = tracy_client::span!("poll events");
		let mut events = ctx.event_pump.poll_iter().collect::<Vec<_>>();
		ctx.mouse_wheel_delta = events.iter().fold(Vec2::ZERO, |delta, event| {
			if let sdl3::event::Event::MouseWheel { x, y, .. } = event {
				delta + Vec2::new(*x, *y)
			} else {
				delta
			}
		});
		drop(span);

		// create egui UI
		let span = tracy_client::span!("create egui UI");
		let egui_input = egui_raw_input(&ctx, &events, delta_time);
		egui_ctx.begin_pass(egui_input);
		if let DevToolsState::Enabled { visible } = ctx.dev_tools_state {
			TopBottomPanel::top("main_menu")
				.show_animated(&egui_ctx, visible, |ui| -> anyhow::Result<()> {
					egui::MenuBar::new()
						.ui(ui, |ui| -> anyhow::Result<()> {
							app.debug_menu(&mut ctx, ui)?;
							ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
								if let Some(stats) = app.debug_stats(&mut ctx) {
									for (i, stat) in stats.iter().enumerate() {
										if i > 0 {
											ui.separator();
										}
										ui.label(stat);
									}
								}
							});
							Ok(())
						})
						.inner?;
					Ok(())
				})
				.map(|response| response.inner)
				.transpose()?;
			if visible {
				app.debug_ui(&mut ctx, &egui_ctx)?;
			}
		}
		let egui_output = egui_ctx.end_pass();
		drop(span);
		ctx.egui_wants_keyboard_input = egui_ctx.wants_keyboard_input();
		ctx.egui_wants_mouse_input = egui_ctx.wants_pointer_input();

		// dispatch events to state
		let span = tracy_client::span!("dispatch events");
		let mouse_event_transform = main_canvas_transform
			.map(|transform| transform.inverse())
			.unwrap_or_default();
		for event in events
			.drain(..)
			.filter(|event| !egui_took_sdl3_event(&egui_ctx, event))
			.filter_map(Event::from_sdl3_event)
		{
			match event {
				Event::WindowSizeChanged(size) => ctx.graphics.resize(size),
				Event::Exited => ctx.should_quit = true,
				Event::KeyPressed {
					key: Scancode::F1, ..
				} => {
					if let DevToolsState::Enabled { visible } = &mut ctx.dev_tools_state {
						*visible = !*visible;
					}
				}
				_ => {}
			}
			app.event(
				&mut ctx,
				event.transform_mouse_events(mouse_event_transform),
			)?;
		}
		drop(span);

		// update state
		let span = tracy_client::span!("update");
		app.update(&mut ctx, delta_time)?;
		drop(span);

		// draw state and egui UI
		let span = tracy_client::span!("draw");
		if let Some(main_canvas) = &main_canvas {
			{
				let clear_color = Some(ctx.clear_color.with_alpha(1.0));
				let ctx = &mut main_canvas.render_to(
					&mut ctx,
					RenderToCanvasSettings {
						clear_color,
						..Default::default()
					},
				);
				app.draw(ctx)?;
			}
			main_canvas
				.transformed(main_canvas_transform.unwrap())
				.draw(&mut ctx);
		} else {
			app.draw(&mut ctx)?;
		}
		drop(span);
		let span = tracy_client::span!("draw egui UI");
		draw_egui_output(&mut ctx, &egui_ctx, egui_output, &mut egui_textures);
		drop(span);
		ctx.graphics.present();

		app.post_draw(&mut ctx)?;

		tracy_client::frame_mark();

		if ctx.should_quit {
			break;
		}
	}

	Ok(())
}

/// Allows you to interact with Micro to check for keyboard inputs,
/// draw graphics, change window settings, etc.
pub struct Context {
	gamepad: GamepadSubsystem,
	event_pump: EventPump,
	mouse_wheel_delta: Vec2,
	egui_wants_keyboard_input: bool,
	egui_wants_mouse_input: bool,
	// `graphics` needs to be before `window`, since it holds
	// a `Surface` that must be dropped before the `Window`
	pub(crate) graphics: GraphicsContext,
	pub(crate) text: TextContext,
	window: Window,
	clear_color: LinSrgb,
	main_canvas_size: Option<UVec2>,
	integer_scaling_enabled: bool,
	frame_time_tracker: FrameTimeTracker,
	should_quit: bool,
	dev_tools_state: DevToolsState,
}

impl Context {
	/// Gets the drawable size of the window (in pixels).
	pub fn window_size(&self) -> UVec2 {
		let (width, height) = self.window.size();
		UVec2::new(width, height)
	}

	/// Returns the number of pixels per logical point on screen.
	pub fn window_scale(&self) -> f32 {
		self.window.display_scale()
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
	pub fn monitor_resolution(&self) -> Result<UVec2, sdl3::Error> {
		let display_mode = self.window.get_display()?.get_mode()?;
		Ok(UVec2::new(display_mode.w as u32, display_mode.h as u32))
	}

	/// Returns `true` if integer scaling is enabled. Only relevant if the
	/// context was set up to use a main canvas.
	pub fn integer_scaling_enabled(&self) -> bool {
		self.integer_scaling_enabled
	}

	/// Sets the window mode (windowed or fullscreen).
	pub fn set_window_mode(&mut self, window_mode: WindowMode) -> Result<(), sdl3::Error> {
		match window_mode {
			WindowMode::Fullscreen => {
				self.window.set_fullscreen(true)?;
			}
			WindowMode::Windowed { size } => {
				self.window.set_fullscreen(false)?;
				self.window
					.set_size(size.x, size.y)
					.map_err(|err| match err {
						IntegerOrSdlError::IntegerOverflows(_, _) => panic!("integer overflow"),
						IntegerOrSdlError::SdlError(err) => err,
					})?;
				self.window
					.set_position(WindowPos::Centered, WindowPos::Centered);
			}
		}
		Ok(())
	}

	/// Sets whether integer scaling is enabled. Only relevant if the
	/// context was set up to use a main canvas.
	pub fn set_integer_scaling_enabled(&mut self, enabled: bool) {
		self.integer_scaling_enabled = enabled;
	}

	/// Returns the current [`PresentMode`].
	pub fn present_mode(&self) -> PresentMode {
		self.graphics.present_mode()
	}

	/// Returns the desired maximum number of frames that can be queued up
	/// ahead of time.
	pub fn desired_maximum_frame_latency(&self) -> u32 {
		self.graphics.desired_maximum_frame_latency()
	}

	/// Returns the texture format of the window surface.
	pub fn surface_format(&self) -> TextureFormat {
		self.graphics.surface_format()
	}

	/**
	Returns the size in pixels of the current render target, which is
	either:
	- The current canvas being rendered to if there is one, or
	- The window
	*/
	pub fn current_render_target_size(&self) -> UVec2 {
		self.graphics.current_render_target_size()
	}

	/// Sets the [`PresentMode`].
	pub fn set_present_mode(&mut self, present_mode: PresentMode) {
		self.graphics.set_present_mode(present_mode);
	}

	/// Sets the desired maximum number of frames that can be queued up
	/// ahead of time. Higher values can stabilize framerates, but increase
	/// input lag.
	pub fn set_desired_maximum_frame_latency(&mut self, frames: u32) {
		self.graphics.set_desired_maximum_frame_latency(frames);
	}

	/// Returns the sample counts for MSAA that the graphics card supports.
	pub fn supported_sample_counts(&self) -> &[u32] {
		// TODO: figure out if this function needs a TextureFormat argument to be accurate
		&self.graphics.supported_sample_counts
	}

	/// Sets the color the window surface will be cleared to at the start
	/// of each frame.
	pub fn set_clear_color(&mut self, color: impl Into<LinSrgb>) {
		let color = color.into();
		self.clear_color = color;
		if self.main_canvas_size.is_none() {
			self.graphics.clear_color = color;
		}
	}

	/// Pushes a set of graphics settings that will be used for upcoming
	/// drawing operations. Returns an object which, when dropped, will
	/// restore the previous set of graphics settings.
	pub fn push(&mut self, push: impl Into<Push>) -> OnDrop<'_> {
		self.graphics.push_graphics_state(push.into());
		OnDrop { ctx: self }
	}

	/// Pushes a transformation that translates all drawing operations by the
	/// specified amount along the X and Y axes.
	pub fn push_translation_2d(&mut self, translation: impl Into<Vec2>) -> OnDrop<'_> {
		self.push(Mat4::from_translation(translation.into().extend(0.0)))
	}

	/// Pushes a transformation that translates all drawing operations by the
	/// specified amount along the X, Y, and Z axes.
	pub fn push_translation_3d(&mut self, translation: impl Into<Vec3>) -> OnDrop<'_> {
		self.push(Mat4::from_translation(translation.into()))
	}

	/// Pushes a transformation that translates all drawing operations by the
	/// specified amount along the X axis.
	pub fn push_translation_x(&mut self, translation: f32) -> OnDrop<'_> {
		self.push(Mat4::from_translation(Vec3::new(translation, 0.0, 0.0)))
	}

	/// Pushes a transformation that translates all drawing operations by the
	/// specified amount along the Y axis.
	pub fn push_translation_y(&mut self, translation: f32) -> OnDrop<'_> {
		self.push(Mat4::from_translation(Vec3::new(0.0, translation, 0.0)))
	}

	/// Pushes a transformation that translates all drawing operations by the
	/// specified amount along the Z axis.
	pub fn push_translation_z(&mut self, translation: f32) -> OnDrop<'_> {
		self.push(Mat4::from_translation(Vec3::new(0.0, 0.0, translation)))
	}

	/// Pushes a transformation that scales all drawing operations by the
	/// specified amount along the X and Y axes.
	pub fn push_scale_2d(&mut self, scale: impl IntoScale2d) -> OnDrop<'_> {
		self.push(Mat4::from_scale(scale.into_scale_2d().extend(0.0)))
	}

	/// Pushes a transformation that scales all drawing operations by the
	/// specified amount along the X, Y, and Z axes.
	pub fn push_scale_3d(&mut self, scale: impl IntoScale3d) -> OnDrop<'_> {
		self.push(Mat4::from_scale(scale.into_scale_3d()))
	}

	/// Pushes a transformation that scales all drawing operations by the
	/// specified amount along the X axis.
	pub fn push_scale_x(&mut self, scale: f32) -> OnDrop<'_> {
		self.push(Mat4::from_scale(Vec3::new(scale, 1.0, 1.0)))
	}

	/// Pushes a transformation that scales all drawing operations by the
	/// specified amount along the Y axis.
	pub fn push_scale_y(&mut self, scale: f32) -> OnDrop<'_> {
		self.push(Mat4::from_scale(Vec3::new(1.0, scale, 1.0)))
	}

	/// Pushes a transformation that scales all drawing operations by the
	/// specified amount along the Z axis.
	pub fn push_scale_z(&mut self, scale: f32) -> OnDrop<'_> {
		self.push(Mat4::from_scale(Vec3::new(1.0, 1.0, scale)))
	}

	/// Pushes a transformation that rotates all drawing operations by the
	/// specified amount around the X axis.
	pub fn push_rotation_x(&mut self, rotation: f32) -> OnDrop<'_> {
		self.push(Mat4::from_rotation_x(rotation))
	}

	/// Pushes a transformation that rotates all drawing operations by the
	/// specified amount around the Y axis.
	pub fn push_rotation_y(&mut self, rotation: f32) -> OnDrop<'_> {
		self.push(Mat4::from_rotation_y(rotation))
	}

	/// Pushes a transformation that rotates all drawing operations by the
	/// specified amount around the Z axis.
	pub fn push_rotation_z(&mut self, rotation: f32) -> OnDrop<'_> {
		self.push(Mat4::from_rotation_z(rotation))
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
	pub fn mouse_position(&self) -> Vec2 {
		let mouse_state = self.event_pump.mouse_state();
		let transform = self
			.main_canvas_size
			.map(|size| {
				main_canvas_transform(size, self.window_size(), self.integer_scaling_enabled)
					.inverse()
			})
			.unwrap_or_default();
		let untransformed = vec2(mouse_state.x(), mouse_state.y());
		transform
			.transform_point3(untransformed.extend(0.0))
			.truncate()
	}

	pub fn mouse_wheel_delta(&self) -> Vec2 {
		self.mouse_wheel_delta
	}

	/// Gets the currently connected gamepads.
	pub fn gamepads(&self) -> anyhow::Result<Vec<Gamepad>> {
		let mut gamepads = vec![];
		for id in self.gamepad.gamepads()? {
			gamepads.push(Gamepad(self.gamepad.get(id)?));
		}
		Ok(gamepads)
	}

	/// Returns the average duration of a frame over the past 30 frames.
	pub fn average_frame_time(&self) -> Duration {
		self.frame_time_tracker.average()
	}

	/// Returns the current frames per second the game is running at.
	pub fn fps(&self) -> f32 {
		1.0 / self.average_frame_time().as_secs_f32()
	}

	/// Returns the current activation state of the dev tools.
	pub fn dev_tools_state(&self) -> DevToolsState {
		self.dev_tools_state
	}

	pub fn load_font_file(&mut self, path: impl AsRef<Path>) -> std::io::Result<()> {
		self.text.load_font_file(path)
	}

	pub fn load_fonts_dir(&mut self, path: impl AsRef<Path>) {
		self.text.load_fonts_dir(path);
	}

	/// Quits the game.
	pub fn quit(&mut self) {
		self.should_quit = true;
	}
}

/// Settings for starting an application.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextSettings {
	/// The title of the application window.
	pub window_title: String,
	/// The size and fullscreen state of the window.
	pub window_mode: WindowMode,
	/// Whether the window is resizable.
	pub resizable: bool,
	pub main_canvas: Option<MainCanvasSettings>,
	/// The [`PresentMode`] used by the application.
	pub present_mode: PresentMode,
	/// The desired maximum number of frames that can be queued up
	/// ahead of time. Higher values can stabilize framerates, but increase
	/// input lag.
	pub desired_maximum_frame_latency: u32,
	/// A bitset of graphics features the application will use.
	pub required_graphics_features: Features,
	/// Whether dev tools should be enabled or not.
	pub dev_tools_mode: DevToolsMode,
}

impl Default for ContextSettings {
	fn default() -> Self {
		Self {
			window_title: "Game".into(),
			window_mode: WindowMode::default(),
			resizable: false,
			main_canvas: None,
			present_mode: PresentMode::AutoVsync,
			desired_maximum_frame_latency: 1,
			required_graphics_features: Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
			dev_tools_mode: DevToolsMode::default(),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MainCanvasSettings {
	pub size: UVec2,
	pub integer_scaling_enabled: bool,
}

/// Configures whether dev tools will be available on this run of the
/// application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DevToolsMode {
	/// Dev tools are disabled.
	Disabled,
	/// Dev tools are enabled.
	Enabled {
		/// Whether the dev tools UI should be shown by default.
		/// This can be toggled at runtime by pressing F1.
		show_by_default: bool,
	},
}

impl DevToolsMode {
	fn initial_state(self) -> DevToolsState {
		match self {
			DevToolsMode::Disabled => DevToolsState::Disabled,
			DevToolsMode::Enabled { show_by_default } => DevToolsState::Enabled {
				visible: show_by_default,
			},
		}
	}
}

impl Default for DevToolsMode {
	fn default() -> Self {
		Self::Enabled {
			show_by_default: true,
		}
	}
}

/// Whether the dev tools are currently enabled, and if so, whether the UI
/// is currently visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DevToolsState {
	/// Dev tools are disabled.
	Disabled,
	/// Dev tools are enabled.
	Enabled {
		/// Whether the dev tools UI is visible or not.
		visible: bool,
	},
}

/// Restores the previous graphics settings when dropped. Returned by
/// the `Context::push_*` functions.
#[must_use]
pub struct OnDrop<'a> {
	ctx: &'a mut Context,
}

impl Drop for OnDrop<'_> {
	fn drop(&mut self) {
		self.ctx.graphics.pop_graphics_state();
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

fn main_canvas_transform(canvas_size: UVec2, window_size: UVec2, integer_scale: bool) -> Mat4 {
	let max_horizontal_scale = window_size.x as f32 / canvas_size.x as f32;
	let max_vertical_scale = window_size.y as f32 / canvas_size.y as f32;
	let mut scale = max_horizontal_scale.min(max_vertical_scale);
	if integer_scale {
		scale = scale.floor();
	}
	Mat4::from_translation((window_size.as_vec2() / 2.0).extend(0.0))
		* Mat4::from_scale(Vec3::splat(scale))
		* Mat4::from_translation((-canvas_size.as_vec2() / 2.0).extend(0.0))
}
