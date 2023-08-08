use std::{
	collections::HashMap,
	fmt::Debug,
	rc::Rc,
	time::{Duration, Instant},
};

use glam::{IVec2, Mat3, UVec2, Vec2};
use glow::HasContext;
use sdl2::{
	video::{FullscreenType, GLContext, GLProfile, SwapInterval, Window, WindowPos},
	EventPump, GameControllerSubsystem, IntegerOrSdlError, Sdl, VideoSubsystem,
};

use crate::{
	egui_integration::{draw_egui_output, egui_raw_input, egui_took_sdl2_event},
	error::SdlError,
	graphics::{
		color::Rgba,
		shader::{Shader, DEFAULT_FRAGMENT_SHADER, DEFAULT_VERTEX_SHADER},
		stencil::{StencilAction, StencilTest},
		texture::{Texture, TextureSettings},
	},
	input::{Gamepad, MouseButton, Scancode},
	log_if_err,
	window::WindowMode,
	Event, State,
};

pub fn run<S, F, E>(settings: ContextSettings, state_constructor: F)
where
	S: State<E>,
	F: FnMut(&mut Context) -> Result<S, E>,
	E: Debug,
{
	#[cfg(debug_assertions)]
	setup_logging();
	#[cfg(not(debug_assertions))]
	let _guard = setup_logging(&settings);
	std::panic::set_hook(Box::new(|info| tracing::error!("{}", info)));
	log_if_err!(run_inner(settings, state_constructor));
}

fn run_inner<S, F, E>(settings: ContextSettings, mut state_constructor: F) -> Result<(), E>
where
	S: State<E>,
	F: FnMut(&mut Context) -> Result<S, E>,
{
	let mut ctx = Context::new(settings);
	let egui_ctx = egui::Context::default();
	let mut egui_textures = HashMap::new();
	let mut state = state_constructor(&mut ctx)?;
	let mut last_update_time = Instant::now();
	loop {
		let now = Instant::now();
		let delta_time = now - last_update_time;
		last_update_time = now;
		let mut events = ctx.event_pump.poll_iter().collect::<Vec<_>>();
		let egui_input = egui_raw_input(&ctx, &events);
		egui_ctx.begin_frame(egui_input);
		state.ui(&mut ctx, &egui_ctx)?;
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
			state.event(&mut ctx, event)?;
		}
		state.update(&mut ctx, delta_time)?;
		state.draw(&mut ctx)?;
		draw_egui_output(&mut ctx, &egui_ctx, egui_output, &mut egui_textures);
		ctx.window.gl_swap_window();
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
	controller: GameControllerSubsystem,
	_sdl_gl_ctx: GLContext,
	event_pump: EventPump,
	should_quit: bool,
	pub(crate) gl: Rc<glow::Context>,
	pub(crate) default_texture: Texture,
	pub(crate) default_shader: Shader,
	pub(crate) transform_stack: Vec<Mat3>,
	pub(crate) render_target: RenderTarget,
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

	pub fn swap_interval(&self) -> SwapInterval {
		self.video.gl_get_swap_interval()
	}

	pub fn monitor_resolution(&self) -> Result<UVec2, SdlError> {
		let display_index = self.window.display_index()?;
		let display_mode = self.video.desktop_display_mode(display_index)?;
		Ok(UVec2::new(display_mode.w as u32, display_mode.h as u32))
	}

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

	pub fn set_swap_interval(&mut self, swap_interval: SwapInterval) -> Result<(), SdlError> {
		self.video.gl_set_swap_interval(swap_interval)?;
		Ok(())
	}

	pub fn clear(&self, color: Rgba) {
		unsafe {
			self.gl
				.clear_color(color.red, color.green, color.blue, color.alpha);
			self.gl.stencil_mask(0xFF);
			self.gl.clear_stencil(0);
			self.gl
				.clear(glow::COLOR_BUFFER_BIT | glow::STENCIL_BUFFER_BIT);
			self.gl.stencil_mask(0x00);
		}
	}

	pub fn clear_stencil(&self) {
		unsafe {
			self.gl.stencil_mask(0xFF);
			self.gl.clear_stencil(0);
			self.gl.clear(glow::STENCIL_BUFFER_BIT);
			self.gl.stencil_mask(0x00);
		}
	}

	pub fn with_transform<T>(&mut self, transform: Mat3, f: impl FnOnce(&mut Context) -> T) -> T {
		self.transform_stack.push(transform);
		let returned_value = f(self);
		self.transform_stack.pop();
		returned_value
	}

	pub fn write_to_stencil<T>(
		&mut self,
		action: StencilAction,
		f: impl FnOnce(&mut Context) -> T,
	) -> T {
		unsafe {
			self.gl.color_mask(false, false, false, false);
			self.gl.enable(glow::STENCIL_TEST);
			let op = action.as_glow_stencil_op();
			self.gl.stencil_op(op, op, op);
			let reference = match action {
				StencilAction::Replace(value) => value,
				_ => 0,
			};
			self.gl.stencil_func(glow::ALWAYS, reference.into(), 0xFF);
			self.gl.stencil_mask(0xFF);
		}
		let returned_value = f(self);
		unsafe {
			self.gl.color_mask(true, true, true, true);
			self.gl.disable(glow::STENCIL_TEST);
		}
		returned_value
	}

	pub fn with_stencil<T>(
		&mut self,
		test: StencilTest,
		reference: u8,
		f: impl FnOnce(&mut Context) -> T,
	) -> T {
		unsafe {
			self.gl.enable(glow::STENCIL_TEST);
			self.gl.stencil_op(glow::KEEP, glow::KEEP, glow::KEEP);
			self.gl
				.stencil_func(test.as_glow_stencil_func(), reference.into(), 0xFF);
			self.gl.stencil_mask(0x00);
		}
		let returned_value = f(self);
		unsafe {
			self.gl.disable(glow::STENCIL_TEST);
		}
		returned_value
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
		let controller = sdl
			.game_controller()
			.expect("error initializing controller subsystem");
		let gl_attr = video.gl_attr();
		gl_attr.set_context_profile(GLProfile::Core);
		gl_attr.set_context_version(3, 3);
		gl_attr.set_stencil_size(8);
		let window = build_window(&video, &settings);
		let (window_width, window_height) = window.size();
		let window_size = UVec2::new(window_width, window_height);
		let _sdl_gl_ctx = window
			.gl_create_context()
			.expect("error creating OpenGL context");
		video
			.gl_set_swap_interval(settings.swap_interval)
			.expect("error setting swap interval");
		let event_pump = sdl.event_pump().expect("error creating event pump");
		let (gl, default_texture, default_shader) = create_gl_context(&video, window_size);
		Self {
			_sdl: sdl,
			video,
			window,
			controller,
			_sdl_gl_ctx,
			event_pump,
			should_quit: false,
			gl,
			default_texture,
			default_shader,
			transform_stack: vec![],
			render_target: RenderTarget::Window,
		}
	}

	pub(crate) fn set_render_target(&mut self, render_target: RenderTarget) {
		self.render_target = render_target;
		let viewport_size: IVec2 = match render_target {
			RenderTarget::Window => self.window_size(),
			RenderTarget::Canvas { size } => size,
		}
		.as_ivec2();
		unsafe {
			self.gl.viewport(0, 0, viewport_size.x, viewport_size.y);
		}
	}

	pub(crate) fn resize(&mut self, size: UVec2) {
		let viewport_size = size.as_ivec2();
		unsafe {
			self.gl.viewport(0, 0, viewport_size.x, viewport_size.y);
		}
	}

	pub(crate) fn global_transform(&self) -> Mat3 {
		let coordinate_system_transform = match self.render_target {
			RenderTarget::Window => {
				let window_size = self.window_size();
				Mat3::from_translation(Vec2::new(-1.0, 1.0))
					* Mat3::from_scale(Vec2::new(
						2.0 / window_size.x as f32,
						-2.0 / window_size.y as f32,
					))
			}
			RenderTarget::Canvas { size } => {
				Mat3::from_translation(Vec2::new(-1.0, -1.0))
					* Mat3::from_scale(Vec2::new(2.0 / size.x as f32, 2.0 / size.y as f32))
			}
		};
		self.transform_stack
			.iter()
			.fold(coordinate_system_transform, |previous, transform| {
				previous * *transform
			})
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum RenderTarget {
	Window,
	Canvas { size: UVec2 },
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

fn create_gl_context(
	video: &VideoSubsystem,
	window_size: UVec2,
) -> (Rc<glow::Context>, Texture, Shader) {
	let gl = Rc::new(unsafe {
		glow::Context::from_loader_function(|name| video.gl_get_proc_address(name) as *const _)
	});
	unsafe {
		gl.enable(glow::BLEND);
		gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
		gl.viewport(0, 0, window_size.x as i32, window_size.y as i32);
	}
	let default_texture = Texture::new_from_gl(
		gl.clone(),
		UVec2::new(1, 1),
		Some(&[255, 255, 255, 255]),
		TextureSettings::default(),
	);
	let default_shader =
		Shader::new_from_gl(gl.clone(), DEFAULT_VERTEX_SHADER, DEFAULT_FRAGMENT_SHADER)
			.expect("Error compiling default shader");
	(gl, default_texture, default_shader)
}

#[cfg(debug_assertions)]
fn setup_logging() {
	use tracing_subscriber::EnvFilter;

	tracing_subscriber::fmt()
		.with_env_filter(
			EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn")),
		)
		.init();
}

#[cfg(not(debug_assertions))]
fn setup_logging(settings: &ContextSettings) -> tracing_appender::non_blocking::WorkerGuard {
	use tracing_subscriber::EnvFilter;

	let file_appender = tracing_appender::rolling::hourly(&settings.logs_dir, "log");
	let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
	tracing_subscriber::fmt()
		.with_env_filter(
			EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn")),
		)
		.with_ansi(false)
		.with_writer(non_blocking)
		.init();
	guard
}
