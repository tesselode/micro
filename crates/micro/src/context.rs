pub(crate) mod graphics;

use std::{
	collections::HashMap,
	time::{Duration, Instant},
};

use glam::{IVec2, UVec2};
use sdl2::{
	video::{FullscreenType, Window, WindowPos},
	EventPump, GameControllerSubsystem, IntegerOrSdlError, Sdl, VideoSubsystem,
};
use wgpu::PresentMode;

use crate::{
	egui_integration::{draw_egui_output, egui_raw_input, egui_took_sdl2_event},
	graphics::color::Rgba,
	input::{Gamepad, MouseButton, Scancode},
	window::WindowMode,
	Event, State,
};

use self::graphics::GraphicsContext;

pub fn run<S, F>(settings: ContextSettings, mut state_constructor: F)
where
	S: State,
	F: FnMut(&mut Context) -> S,
{
	let mut ctx = Context::new(settings);
	let egui_ctx = egui::Context::default();
	let mut egui_textures = HashMap::new();
	let mut state = state_constructor(&mut ctx);
	let mut last_update_time = Instant::now();
	loop {
		let now = Instant::now();
		let delta_time = now - last_update_time;
		last_update_time = now;
		let mut events = ctx.event_pump.poll_iter().collect::<Vec<_>>();
		let egui_input = egui_raw_input(&ctx, &events);
		egui_ctx.begin_frame(egui_input);
		state.ui(&mut ctx, &egui_ctx);
		let egui_output = egui_ctx.end_frame();
		for event in events
			.drain(..)
			.filter(|event| !egui_took_sdl2_event(&egui_ctx, event))
			.filter_map(Event::from_sdl2_event)
		{
			match event {
				Event::WindowSizeChanged(size) => ctx.resize(size),
				Event::Exited => {
					ctx.should_quit = true;
				}
				_ => {}
			}
			state.event(&mut ctx, event);
		}
		state.update(&mut ctx, delta_time);
		state.draw(&mut ctx);
		draw_egui_output(&mut ctx, &egui_ctx, egui_output, &mut egui_textures);
		ctx.graphics_ctx.render();
		if ctx.should_quit {
			break;
		}
		std::thread::sleep(Duration::from_millis(2));
	}
}

pub struct Context {
	_sdl: Sdl,
	video: VideoSubsystem,
	window: Window,
	controller: GameControllerSubsystem,
	event_pump: EventPump,
	pub(crate) graphics_ctx: GraphicsContext,
	should_quit: bool,
}

impl Context {
	pub fn window_size(&self) -> UVec2 {
		let (width, height) = self.window.size();
		UVec2::new(width, height)
	}

	pub fn window_mode(&self) -> WindowMode {
		match self.window.fullscreen_state() {
			FullscreenType::Off => WindowMode::Windowed {
				size: self.window_size(),
			},
			FullscreenType::True => WindowMode::Fullscreen,
			FullscreenType::Desktop => WindowMode::Fullscreen,
		}
	}

	pub fn present_mode(&self) -> PresentMode {
		self.graphics_ctx.config.present_mode
	}

	pub fn monitor_resolution(&self) -> UVec2 {
		let display_index = self
			.window
			.display_index()
			.expect("could not get display index of window");
		let display_mode = self
			.video
			.desktop_display_mode(display_index)
			.expect("could not get display mode");
		UVec2::new(
			display_mode
				.w
				.try_into()
				.expect("could not convert i32 into u32"),
			display_mode
				.h
				.try_into()
				.expect("could not convert i32 into u32"),
		)
	}

	pub fn set_window_mode(&mut self, window_mode: WindowMode) {
		match window_mode {
			WindowMode::Fullscreen => {
				self.window
					.set_fullscreen(FullscreenType::Desktop)
					.expect("error setting fullscreen mode");
			}
			WindowMode::Windowed { size } => {
				self.window
					.set_fullscreen(FullscreenType::Off)
					.expect("error setting fullscreen mode");
				self.window
					.set_size(size.x, size.y)
					.expect("error setting window size");
				self.window
					.set_position(WindowPos::Centered, WindowPos::Centered);
			}
		}
	}

	pub fn set_present_mode(&mut self, present_mode: PresentMode) {
		self.graphics_ctx.set_present_mode(present_mode);
	}

	pub fn set_background_color(&mut self, background_color: Rgba) {
		self.graphics_ctx.set_background_color(background_color);
	}

	pub(crate) fn resize(&mut self, size: UVec2) {
		self.graphics_ctx.resize(size);
	}

	pub fn is_key_down(&self, scancode: Scancode) -> bool {
		self.event_pump
			.keyboard_state()
			.is_scancode_pressed(scancode.into())
	}

	pub fn is_mouse_button_down(&self, mouse_button: MouseButton) -> bool {
		self.event_pump
			.mouse_state()
			.is_mouse_button_pressed(mouse_button.into())
	}

	pub fn mouse_position(&self) -> IVec2 {
		let mouse_state = self.event_pump.mouse_state();
		IVec2::new(mouse_state.x(), mouse_state.y())
	}

	pub fn game_controller(&self, index: u32) -> Option<Gamepad> {
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

	pub fn quit(&mut self) {
		self.should_quit = true;
	}

	fn new(settings: ContextSettings) -> Self {
		let sdl = sdl2::init().expect("error initializing SDL");
		let video = sdl.video().expect("error initializing video subsystem");
		let controller = sdl.game_controller().unwrap();
		let window = build_window(&video, &settings);
		let event_pump = sdl.event_pump().expect("error creating event pump");
		let graphics_ctx = GraphicsContext::new(&window, settings);
		Self {
			_sdl: sdl,
			video,
			window,
			controller,
			event_pump,
			graphics_ctx,
			should_quit: false,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextSettings {
	pub window_title: String,
	pub window_mode: WindowMode,
	pub resizable: bool,
	pub present_mode: PresentMode,
}

impl Default for ContextSettings {
	fn default() -> Self {
		Self {
			window_title: "Game".into(),
			window_mode: WindowMode::default(),
			resizable: false,
			present_mode: PresentMode::default(),
		}
	}
}

fn build_window(video: &VideoSubsystem, settings: &ContextSettings) -> Window {
	let window_size = match settings.window_mode {
		// doesn't matter because we're going to set the window to fullscreen
		WindowMode::Fullscreen => UVec2::new(800, 600),
		WindowMode::Windowed { size } => size,
	};
	let mut window_builder = video.window(&settings.window_title, window_size.x, window_size.y);
	if settings.window_mode == WindowMode::Fullscreen {
		window_builder.fullscreen_desktop();
	}
	window_builder.opengl();
	if settings.resizable {
		window_builder.resizable();
	}
	window_builder.build().expect("error building window")
}
