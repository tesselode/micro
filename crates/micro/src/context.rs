pub(crate) mod graphics;

use std::time::{Duration, Instant};

use glam::{IVec2, UVec2};
use sdl2::{
	video::{SwapInterval, Window, WindowBuildError},
	EventPump, Sdl, VideoSubsystem,
};
use thiserror::Error;
use wgpu::{CreateSurfaceError, RequestDeviceError, SurfaceError};

use crate::{
	input::{MouseButton, Scancode},
	window::WindowMode,
	Event, State,
};

use self::graphics::GraphicsContext;

pub fn run<S, F, E>(settings: ContextSettings, mut state_constructor: F) -> Result<(), E>
where
	S: State<E>,
	F: FnMut(&mut Context) -> Result<S, E>,
	E: From<InitError>,
	E: From<SurfaceError>,
{
	let mut ctx = Context::new(settings)?;
	/* let egui_ctx = egui::Context::default();
	let mut egui_textures = HashMap::new(); */
	let mut state = state_constructor(&mut ctx)?;
	let mut last_update_time = Instant::now();
	loop {
		let now = Instant::now();
		let delta_time = now - last_update_time;
		last_update_time = now;
		let mut events = ctx.event_pump.poll_iter().collect::<Vec<_>>();
		/* let egui_input = egui_raw_input(&ctx, &events);
		egui_ctx.begin_frame(egui_input);
		state.ui(&mut ctx, &egui_ctx)?;
		let egui_output = egui_ctx.end_frame(); */
		for event in events
			.drain(..)
			/* .filter(|event| !egui_took_sdl2_event(&egui_ctx, event)) */
			.filter_map(Event::from_sdl2_event)
		{
			match event {
				Event::WindowSizeChanged(size) => ctx.resize(size),
				Event::Exited => {
					ctx.should_quit = true;
				}
				_ => {}
			}
			state.event(&mut ctx, event)?;
		}
		state.update(&mut ctx, delta_time)?;
		state.draw(&mut ctx)?;
		ctx.graphics_ctx.render()?;
		/* draw_egui_output(&mut ctx, &egui_ctx, egui_output, &mut egui_textures); */
		if ctx.should_quit {
			break;
		}
		std::thread::sleep(Duration::from_millis(2));
	}
	Ok(())
}

pub struct Context {
	_sdl: Sdl,
	video: VideoSubsystem,
	window: Window,
	event_pump: EventPump,
	graphics_ctx: GraphicsContext,
	should_quit: bool,
}

impl Context {
	pub fn new(settings: ContextSettings) -> Result<Self, InitError> {
		let sdl = sdl2::init().map_err(InitError::Sdl2Error)?;
		let video = sdl.video().map_err(InitError::Sdl2Error)?;
		let window = build_window(&video, &settings)?;
		let event_pump = sdl.event_pump().map_err(InitError::Sdl2Error)?;
		let graphics_ctx = GraphicsContext::new(&window)?;
		Ok(Self {
			_sdl: sdl,
			video,
			window,
			event_pump,
			graphics_ctx,
			should_quit: false,
		})
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextSettings {
	pub window_title: String,
	pub window_mode: WindowMode,
	pub resizable: bool,
	pub swap_interval: SwapInterval,
}

impl Default for ContextSettings {
	fn default() -> Self {
		Self {
			window_title: "Game".into(),
			window_mode: WindowMode::default(),
			resizable: false,
			swap_interval: SwapInterval::VSync,
		}
	}
}

#[derive(Debug, Clone, Error)]
pub enum InitError {
	#[error("{0}")]
	Sdl2Error(String),
	#[error("{0}")]
	WindowBuildError(#[from] WindowBuildError),
	#[error("{0}")]
	InitGraphicsError(#[from] InitGraphicsError),
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum InitGraphicsError {
	#[error("{0}")]
	CreateSurfaceError(#[from] CreateSurfaceError),
	#[error("{0}")]
	RequestDeviceError(#[from] RequestDeviceError),
	#[error("Could not find a graphics adapter")]
	NoAdapterFound,
}

fn build_window(
	video: &VideoSubsystem,
	settings: &ContextSettings,
) -> Result<Window, WindowBuildError> {
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
	window_builder.build()
}
