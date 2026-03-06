pub(crate) mod graphics;
pub(crate) mod text;

mod push;

use egui::{Align, Layout, TopBottomPanel};
use palette::{LinSrgb, WithAlpha};
pub use push::*;

use winit::{
	application::ApplicationHandler,
	dpi::{PhysicalPosition, PhysicalSize, Position, Size},
	event::{DeviceEvent, DeviceId, KeyEvent, MouseButton, WindowEvent},
	event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
	keyboard::{KeyCode, PhysicalKey},
	window::{Fullscreen, Window, WindowId},
};

use std::{
	collections::{HashMap, HashSet},
	fmt::Debug,
	ops::{Deref, DerefMut},
	path::Path,
	sync::Arc,
	time::{Duration, Instant},
};

use glam::{Mat4, UVec2, Vec2, Vec3, uvec2, vec2};
use wgpu::{Features, PresentMode, TextureFormat};

use crate::{
	App, Event, FrameTimeTracker, WindowMode, build_window,
	color::ColorConstants,
	context::graphics::GraphicsContext,
	egui_integration::{draw_egui_output, egui_raw_input, egui_took_event},
	graphics::{
		Canvas, CanvasSettings, IntoScale2d, IntoScale3d, RenderToCanvasSettings, texture::Texture,
	},
	text::TextContext,
};

type AppConstructor<A> = dyn FnMut(&mut Context) -> A + 'static;

/// Starts a Micro application. The app constructor should return a value of a type
/// that implements [`App`].
pub fn run<A, F>(settings: ContextSettings, app_constructor: F)
where
	A: App,
	F: FnMut(&mut Context) -> A + 'static,
{
	let event_loop = EventLoop::new().expect("error creating event loop");
	event_loop.set_control_flow(ControlFlow::Poll);
	let mut app_handler = MicroAppHandler::new(settings, Box::new(app_constructor));
	event_loop
		.run_app(&mut app_handler)
		.expect("error starting event loop");
}

/// Allows you to interact with Micro to check for keyboard inputs,
/// draw graphics, change window settings, etc.
pub struct Context {
	window: Arc<Window>,
	pub(crate) graphics: GraphicsContext,
	pub(crate) text: TextContext,
	held_keys: HashSet<KeyCode>,
	held_mouse_buttons: HashSet<MouseButton>,
	mouse_position: Option<Vec2>,
	egui_wants_keyboard_input: bool,
	egui_wants_mouse_input: bool,
	clear_color: LinSrgb,
	frame_time_tracker: FrameTimeTracker,
	dev_tools_state: DevToolsState,
	main_canvas_size: Option<UVec2>,
	integer_scaling_enabled: bool,
	should_quit: bool,
}

impl Context {
	/// Gets the drawable size of the window (in pixels).
	pub fn window_size(&self) -> UVec2 {
		let PhysicalSize { width, height } = self.window.inner_size();
		UVec2::new(width, height)
	}

	/// Returns the number of pixels per logical point on screen.
	pub fn window_scale(&self) -> f32 {
		self.window
			.current_monitor()
			.expect("could not get monitor for window")
			.scale_factor() as f32
	}

	/// Returns the current window mode (windowed or fullscreen).
	pub fn window_mode(&self) -> WindowMode {
		match self.window.fullscreen() {
			Some(Fullscreen::Borderless(_)) => WindowMode::Fullscreen,
			None => WindowMode::Windowed {
				size: uvec2(
					self.window.inner_size().width,
					self.window.inner_size().height,
				),
			},
			_ => unreachable!("no other fullscreen modes are supported"),
		}
	}

	/// Returns the resolution of the monitor the window is on.
	pub fn monitor_resolution(&self) -> UVec2 {
		let size = self
			.window
			.current_monitor()
			.expect("could not get monitor for window")
			.size();
		UVec2::new(size.width, size.height)
	}

	/* /// Returns `true` if integer scaling is enabled. Only relevant if the
	/// context was set up to use a main canvas.
	pub fn integer_scaling_enabled(&self) -> bool {
		self.integer_scaling_enabled
	} */

	/// Sets the window mode (windowed or fullscreen).
	pub fn set_window_mode(&mut self, window_mode: WindowMode) {
		match window_mode {
			WindowMode::Fullscreen => {
				self.window
					.set_fullscreen(Some(Fullscreen::Borderless(None)));
			}
			WindowMode::Windowed { size } => {
				self.window.set_fullscreen(None);
				let new_size = self.window.request_inner_size(Size::Physical(PhysicalSize {
					width: size.x,
					height: size.y,
				}));
				if let Some(new_size) = new_size {
					self.resize(uvec2(new_size.width, new_size.height));
					if let Some(primary_monitor) = self.window.primary_monitor() {
						self.window
							.set_outer_position(Position::Physical(PhysicalPosition {
								x: (primary_monitor.size().width / 2 - new_size.width / 2) as i32,
								y: (primary_monitor.size().height / 2 - new_size.height / 2) as i32,
							}));
					}
				}
			}
		}
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
	pub fn is_key_down(&self, key: KeyCode) -> bool {
		self.held_keys.contains(&key) && !self.egui_wants_keyboard_input
	}

	/// Returns `true` if the given mouse button is currently held down.
	pub fn is_mouse_button_down(&self, mouse_button: MouseButton) -> bool {
		self.held_mouse_buttons.contains(&mouse_button) && !self.egui_wants_mouse_input
	}

	/// Returns the current mouse position (in pixels, relative to the top-left
	/// corner of the window).
	pub fn mouse_position(&self) -> Option<Vec2> {
		let transform = self
			.main_canvas_size
			.map(|size| {
				main_canvas_transform(size, self.window_size(), self.integer_scaling_enabled)
					.inverse()
			})
			.unwrap_or_default();
		self.mouse_position
			.map(|position| transform.transform_point3(position.extend(0.0)).truncate())
	}

	/// Returns the average duration of a frame over the past 30 frames.
	pub fn average_frame_time(&self) -> Duration {
		self.frame_time_tracker.average()
	}

	/// Returns the current frames per second the game is running at.
	pub fn fps(&self) -> f32 {
		1.0 / self.average_frame_time().as_secs_f32()
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

	async fn new(settings: &ContextSettings, window: Arc<Window>) -> Self {
		let graphics = GraphicsContext::new(window.clone(), settings);
		let text = TextContext::new(&graphics);
		Self {
			window,
			graphics,
			text,
			held_keys: HashSet::new(),
			held_mouse_buttons: HashSet::new(),
			mouse_position: None,
			egui_wants_keyboard_input: false,
			egui_wants_mouse_input: false,
			clear_color: LinSrgb::BLACK,
			main_canvas_size: settings.main_canvas.map(|settings| settings.size),
			integer_scaling_enabled: settings
				.main_canvas
				.map(|settings| settings.integer_scaling_enabled)
				.unwrap_or_default(),
			frame_time_tracker: FrameTimeTracker::new(),
			dev_tools_state: settings.dev_tools_mode.initial_state(),
			should_quit: false,
		}
	}

	fn resize(&mut self, size: UVec2) {
		self.graphics.resize(size);
		if let Some(primary_monitor) = self.window.primary_monitor() {
			self.window
				.set_outer_position(Position::Physical(PhysicalPosition {
					x: (primary_monitor.size().width / 2 - size.x / 2) as i32,
					y: (primary_monitor.size().height / 2 - size.y / 2) as i32,
				}));
		}
	}

	fn render(&mut self) {
		self.graphics.present();
		self.window.request_redraw();
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

struct MicroAppHandler<A: App> {
	settings: ContextSettings,
	status: Status<A>,
}

impl<A: App> MicroAppHandler<A> {
	pub fn new(settings: ContextSettings, app_constructor: Box<AppConstructor<A>>) -> Self {
		Self {
			status: Status::Uninitialized { app_constructor },
			settings,
		}
	}
}

impl<A: App> ApplicationHandler for MicroAppHandler<A> {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		let Status::Uninitialized { app_constructor } = &mut self.status else {
			return;
		};
		let window = build_window(event_loop, &self.settings);
		let mut ctx = pollster::block_on(Context::new(&self.settings, window));
		let app = app_constructor(&mut ctx);
		let main_canvas = self
			.settings
			.main_canvas
			.map(|settings| Canvas::new(&ctx, settings.size, CanvasSettings::default()));
		self.status = Status::Initialized {
			app,
			ctx,
			event_queue: vec![],
			last_update_time: Instant::now(),
			egui_ctx: egui::Context::default(),
			egui_textures: HashMap::new(),
			main_canvas,
		};
	}

	fn device_event(
		&mut self,
		_event_loop: &ActiveEventLoop,
		_device_id: DeviceId,
		event: DeviceEvent,
	) {
		let Status::Initialized { event_queue, .. } = &mut self.status else {
			return;
		};

		if let Some(event) = Event::from_device_event(&event) {
			event_queue.push(event);
		}
	}

	fn window_event(
		&mut self,
		event_loop: &ActiveEventLoop,
		_window_id: WindowId,
		event: WindowEvent,
	) {
		let Status::Initialized {
			app,
			ctx,
			event_queue,
			last_update_time,
			egui_ctx,
			egui_textures,
			main_canvas,
		} = &mut self.status
		else {
			return;
		};

		event_queue.append(&mut Event::from_window_event(&event, ctx.mouse_position));

		match event {
			WindowEvent::CloseRequested => event_loop.exit(),
			WindowEvent::CursorMoved { position, .. } => {
				ctx.mouse_position = Some(vec2(position.x as f32, position.y as f32))
			}
			WindowEvent::CursorLeft { .. } => ctx.mouse_position = None,
			WindowEvent::KeyboardInput {
				event:
					KeyEvent {
						physical_key: PhysicalKey::Code(code),
						state,
						..
					},
				..
			} => match state {
				winit::event::ElementState::Pressed => {
					ctx.held_keys.insert(code);
				}
				winit::event::ElementState::Released => {
					ctx.held_keys.remove(&code);
				}
			},
			WindowEvent::MouseInput { state, button, .. } => match state {
				winit::event::ElementState::Pressed => {
					ctx.held_mouse_buttons.insert(button);
				}
				winit::event::ElementState::Released => {
					ctx.held_mouse_buttons.remove(&button);
				}
			},
			WindowEvent::Resized(size) => ctx.resize(uvec2(size.width, size.height)),
			WindowEvent::RedrawRequested => {
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
				let delta_time = now - *last_update_time;
				*last_update_time = now;
				ctx.frame_time_tracker.record(delta_time);

				// create egui UI
				let span = tracy_client::span!("create egui UI");
				let egui_input = egui_raw_input(ctx, event_queue, delta_time);
				egui_ctx.begin_pass(egui_input);
				if let DevToolsState::Enabled { visible } = ctx.dev_tools_state {
					TopBottomPanel::top("main_menu").show_animated(egui_ctx, visible, |ui| {
						egui::MenuBar::new().ui(ui, |ui| {
							app.debug_menu(ctx, ui);
							ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
								if let Some(stats) = app.debug_stats(ctx) {
									for (i, stat) in stats.iter().enumerate() {
										if i > 0 {
											ui.separator();
										}
										ui.label(stat);
									}
								}
							});
						});
					});
					if visible {
						app.debug_ui(ctx, egui_ctx);
					}
				}
				let egui_output = egui_ctx.end_pass();
				drop(span);
				ctx.egui_wants_keyboard_input = egui_ctx.wants_keyboard_input();
				ctx.egui_wants_mouse_input = egui_ctx.wants_pointer_input();

				// dispatch events
				let span = tracy_client::span!("dispatch events");
				let mouse_event_transform = main_canvas_transform
					.map(|transform| transform.inverse())
					.unwrap_or_default();
				for event in event_queue
					.drain(..)
					.filter(|event| !egui_took_event(egui_ctx, event))
				{
					if let Event::KeyPressed {
						key: KeyCode::F1,
						is_repeat: false,
					} = &event && let DevToolsState::Enabled { visible } =
						&mut ctx.dev_tools_state
					{
						*visible = !*visible;
					}
					app.event(ctx, event.transform_mouse_events(mouse_event_transform));
				}
				drop(span);

				// update
				let span = tracy_client::span!("update");
				app.update(ctx, delta_time);
				drop(span);

				// draw
				let span = tracy_client::span!("draw");
				if let Some(main_canvas) = &main_canvas {
					{
						let clear_color = Some(ctx.clear_color.with_alpha(1.0));
						let ctx = &mut main_canvas.render_to(
							ctx,
							RenderToCanvasSettings {
								clear_color,
								..Default::default()
							},
						);
						app.draw(ctx);
					}
					main_canvas
						.transformed(main_canvas_transform.unwrap())
						.draw(ctx);
				} else {
					app.draw(ctx);
				}
				drop(span);
				let span = tracy_client::span!("draw egui UI");
				draw_egui_output(ctx, egui_ctx, egui_output, egui_textures);
				drop(span);
				ctx.render();

				app.post_draw(ctx);

				tracy_client::frame_mark();

				if ctx.should_quit {
					event_loop.exit();
				}
			}
			_ => {}
		}
	}

	fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
		let Status::Initialized { app, ctx, .. } = &mut self.status else {
			return;
		};

		app.event(ctx, Event::Exited);
	}
}

enum Status<A: App> {
	Uninitialized {
		app_constructor: Box<AppConstructor<A>>,
	},
	Initialized {
		app: A,
		ctx: Context,
		event_queue: Vec<Event>,
		last_update_time: Instant,
		egui_ctx: egui::Context,
		egui_textures: HashMap<egui::TextureId, Texture>,
		main_canvas: Option<Canvas>,
	},
}
