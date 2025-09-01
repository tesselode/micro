pub(crate) mod graphics;

use std::time::{Duration, Instant};

use glam::{IVec2, UVec2, vec2};
use palette::LinSrgb;
use sdl3::{
	EventPump, GamepadSubsystem, IntegerOrSdlError,
	video::{FullscreenType, Window, WindowPos},
};
use wgpu::{Features, PresentMode};

use crate::{
	App, Event, FrameTimeTracker, WindowMode, build_window,
	context::graphics::GraphicsContext,
	input::{Gamepad, MouseButton, Scancode},
};

pub fn run<A, F>(settings: ContextSettings, mut app_constructor: F)
where
	A: App,
	F: FnMut(&mut Context) -> A,
{
	let sdl = sdl3::init().expect("error initializing SDL");
	let video = sdl.video().expect("error initializing video subsystem");
	let controller = sdl
		.gamepad()
		.expect("error initializing controller subsystem");
	let window = build_window(&video, &settings);
	let event_pump = sdl.event_pump().expect("error creating event pump");
	let graphics = GraphicsContext::new(&window, &settings);

	let mut ctx = Context {
		window,
		gamepad: controller,
		event_pump,
		// egui_wants_keyboard_input: false,
		// egui_wants_mouse_input: false,
		frame_time_tracker: FrameTimeTracker::new(),
		graphics,
		should_quit: false,
	};
	// let egui_ctx = egui::Context::default();
	// let mut egui_textures = HashMap::new();
	let mut app = app_constructor(&mut ctx);

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
		/* let span = tracy_client::span!("create egui UI");
		let egui_input = egui_raw_input(&ctx, &events, delta_time);
		egui_ctx.begin_pass(egui_input);
		app.debug_ui(&mut ctx, &egui_ctx)?;
		let egui_output = egui_ctx.end_pass();
		drop(span); */

		// dispatch events to state
		let span = tracy_client::span!("dispatch events");
		for event in events
			.drain(..)
			// .filter(|event| !egui_took_sdl3_event(&egui_ctx, event))
			.filter_map(Event::from_sdl3_event)
		{
			match event {
				Event::WindowSizeChanged(size) => ctx.graphics.resize(size),
				Event::Exited => ctx.should_quit = true,
				_ => {}
			}
			app.event(&mut ctx, event);
		}
		drop(span);
		// ctx.egui_wants_keyboard_input = egui_ctx.wants_keyboard_input();
		// ctx.egui_wants_mouse_input = egui_ctx.wants_pointer_input();

		// update state
		let span = tracy_client::span!("update");
		app.update(&mut ctx, delta_time);
		drop(span);

		// draw state and egui UI
		let span = tracy_client::span!("draw");

		drop(span);
		app.draw(&mut ctx);
		/* let span = tracy_client::span!("draw egui UI");
		draw_egui_output(&mut ctx, &egui_ctx, egui_output, &mut egui_textures);
		drop(span); */
		ctx.graphics.present();

		tracy_client::frame_mark();

		if ctx.should_quit {
			break;
		}
	}
}

pub struct Context {
	pub(crate) gamepad: GamepadSubsystem,
	pub(crate) event_pump: EventPump,
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
		let (width, height) = self.window.size();
		UVec2::new(width, height)
	}

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

	pub fn set_clear_color(&mut self, color: impl Into<LinSrgb>) {
		self.graphics.clear_color = color.into();
	}

	/// Returns `true` if the given keyboard key is currently held down.
	pub fn is_key_down(&self, scancode: Scancode) -> bool {
		self.event_pump
			.keyboard_state()
			.is_scancode_pressed(scancode.into())
		/* && !self.egui_wants_keyboard_input */
	}

	/// Returns `true` if the given mouse button is currently held down.
	pub fn is_mouse_button_down(&self, mouse_button: MouseButton) -> bool {
		self.event_pump
			.mouse_state()
			.is_mouse_button_pressed(mouse_button.into())
		/* && !self.egui_wants_mouse_input */
	}

	/// Returns the current mouse position (in pixels, relative to the top-left
	/// corner of the window).
	pub fn mouse_position(&self) -> IVec2 {
		let mouse_state = self.event_pump.mouse_state();
		vec2(mouse_state.x(), mouse_state.y()).as_ivec2()
	}

	/// Gets the gamepad with the given index if it's connected.
	pub fn gamepad(&self, index: u32) -> Option<Gamepad> {
		self.gamepad.open(index).map(Gamepad).ok()
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
	pub max_queued_frames: u32,
	pub required_graphics_features: Features,
}

impl Default for ContextSettings {
	fn default() -> Self {
		Self {
			window_title: "Game".into(),
			window_mode: WindowMode::default(),
			resizable: false,
			present_mode: PresentMode::AutoVsync,
			max_queued_frames: 1,
			required_graphics_features: Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
		}
	}
}
