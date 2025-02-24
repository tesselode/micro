use std::time::Instant;

use glam::{IVec2, UVec2, vec2};
use sdl2::{
	EventPump, GameControllerSubsystem, IntegerOrSdlError, Sdl, VideoSubsystem,
	video::{FullscreenType, Window, WindowPos},
};

use crate::{
	App, Event, SdlError,
	input::{Gamepad, MouseButton, Scancode},
	window::{WindowMode, build_window},
};

pub fn run<S, F, E>(settings: ContextSettings, mut app_constructor: F) -> Result<(), E>
where
	S: App<Error = E>,
	F: FnMut(&mut Context) -> Result<S, E>,
{
	let mut ctx = Context::new(settings);
	let mut app = app_constructor(&mut ctx)?;

	let mut last_update_time = Instant::now();

	loop {
		// measure and record delta time
		let now = Instant::now();
		let delta_time = now - last_update_time;
		last_update_time = now;
		// ctx.frame_time_tracker.record(delta_time);

		// poll for events
		// let span = tracy_client::span!("poll events");
		let mut events = ctx.event_pump.poll_iter().collect::<Vec<_>>();
		// drop(span);

		// create egui UI
		/* let span = tracy_client::span!("create egui UI");
		let egui_input = egui_raw_input(&ctx, &events, delta_time);
		egui_ctx.begin_pass(egui_input);
		app.debug_ui(&mut ctx, &egui_ctx)?;
		let egui_output = egui_ctx.end_pass();
		drop(span); */

		// dispatch events to state
		// let span = tracy_client::span!("dispatch events");
		for event in events
			.drain(..)
			// .filter(|event| !egui_took_sdl2_event(&egui_ctx, event))
			.filter_map(Event::from_sdl2_event)
		{
			match event {
				// Event::WindowSizeChanged(size) => ctx.graphics.resize(size),
				Event::Exited => ctx.should_quit = true,
				_ => {}
			}
			// let transform = ctx.scaling_mode.transform_affine2(&ctx).inverse();
			// let dpi_scaling = ctx.window_size().y as f32 / ctx.logical_window_size().y as f32;
			app.event(
				&mut ctx, event, /* .transform_mouse_events(transform, dpi_scaling) */
			)?;
		}
		// drop(span);
		// ctx.egui_wants_keyboard_input = egui_ctx.wants_keyboard_input();
		// ctx.egui_wants_mouse_input = egui_ctx.wants_pointer_input();

		// update state
		// let span = tracy_client::span!("update");
		app.update(&mut ctx, delta_time)?;
		// drop(span);

		// draw state and egui UI
		// let span = tracy_client::span!("draw");

		// drop(span);
		// let span = tracy_client::span!("draw egui UI");
		// draw_egui_output(&mut ctx, &egui_ctx, egui_output, &mut egui_textures);
		// drop(span);
		ctx.window.gl_swap_window();

		// tracy_client::frame_mark();
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
	pub(crate) window: Window,
	pub(crate) controller: GameControllerSubsystem,
	pub(crate) event_pump: EventPump,
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
		let dpi_scaling = self.window_size().y as f32 / self.logical_window_size().y as f32;
		let untransformed = vec2(
			mouse_state.x() as f32 * dpi_scaling,
			mouse_state.y() as f32 * dpi_scaling,
		)
		.as_ivec2();
		untransformed
		/* self.scaling_mode
		.transform_affine2(self)
		.inverse()
		.transform_point2(untransformed.as_vec2())
		.as_ivec2() */
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

	/// Quits the game.
	pub fn quit(&mut self) {
		self.should_quit = true;
	}

	fn new(settings: ContextSettings) -> Self {
		let sdl = sdl2::init().expect("error initializing SDL");
		let video = sdl.video().expect("error initializing video subsystem");
		let controller = sdl
			.game_controller()
			.expect("error initializing controller subsystem");
		let window = build_window(&video, &settings);
		let event_pump = sdl.event_pump().expect("error creating event pump");
		Self {
			_sdl: sdl,
			video,
			window,
			controller,
			event_pump,
			should_quit: false,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextSettings {
	pub window_title: String,
	pub window_mode: WindowMode,
	pub resizable: bool,
	// pub swap_interval: SwapInterval,
	// pub scaling_mode: ScalingMode,
}

impl Default for ContextSettings {
	fn default() -> Self {
		Self {
			window_title: "Game".into(),
			window_mode: WindowMode::default(),
			resizable: false,
			// swap_interval: SwapInterval::VSync,
			// scaling_mode: ScalingMode::default(),
		}
	}
}
