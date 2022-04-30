pub mod error;

use vek::{Mat4, Vec2, Vec3};

use std::{
	rc::Rc,
	time::{Duration, Instant},
};

use glow::HasContext;
use sdl2::{
	event::{Event, WindowEvent},
	keyboard::Scancode,
	mouse::MouseButton,
	video::{GLContext, GLProfile, SwapInterval, Window},
	EventPump, GameControllerSubsystem, IntegerOrSdlError, Sdl, VideoSubsystem,
};

use crate::{
	graphics::{
		color::Rgba,
		shader::Shader,
		texture::{Texture, TextureSettings},
	},
	input::GameController,
	State,
};

use self::error::InitError;

pub struct Context {
	_sdl: Sdl,
	_video: VideoSubsystem,
	window: Window,
	controller: GameControllerSubsystem,
	_sdl_gl_ctx: GLContext,
	event_pump: EventPump,
	should_quit: bool,
	pub(crate) gl: Rc<glow::Context>,
	pub(crate) default_texture: Texture,
	pub(crate) default_shader: Shader,
	pub(crate) transform_stack: Vec<Mat4<f32>>,
	pub(crate) render_target: RenderTarget,
}

impl Context {
	pub fn new(settings: ContextSettings) -> Result<Self, InitError> {
		let sdl = sdl2::init()?;
		let video = sdl.video()?;
		let controller = sdl.game_controller()?;
		let gl_attr = video.gl_attr();
		gl_attr.set_context_profile(GLProfile::Core);
		gl_attr.set_context_version(3, 3);
		let window = build_window(&video, &settings)?;
		let (window_width, window_height) = window.size();
		let window_size = Vec2::new(
			window_width.try_into().expect("window is too wide"),
			window_height.try_into().expect("window is too tall"),
		);
		let _sdl_gl_ctx = window.gl_create_context()?;
		video
			.gl_set_swap_interval(if settings.vsync {
				SwapInterval::VSync
			} else {
				SwapInterval::Immediate
			})
			.expect("Could not set vsync");
		let event_pump = sdl.event_pump()?;
		let (gl, default_texture, default_shader) = create_gl_context(&video, window_size);
		Ok(Self {
			_sdl: sdl,
			_video: video,
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
		})
	}

	pub fn run<E, S, F>(&mut self, mut state_constructor: F) -> Result<(), E>
	where
		S: State<E>,
		F: FnMut(&mut Context) -> Result<S, E>,
	{
		let mut state = state_constructor(self)?;
		let mut last_update_time = Instant::now();
		loop {
			let now = Instant::now();
			let delta_time = now - last_update_time;
			last_update_time = now;
			state.update(self, delta_time)?;
			state.draw(self)?;
			self.window.gl_swap_window();
			while let Some(event) = self.event_pump.poll_event() {
				match event {
					Event::Window {
						win_event: WindowEvent::Resized(width, height),
						..
					} => {
						self.resize(Vec2::new(width as u32, height as u32));
					}
					Event::Quit { .. } => {
						self.should_quit = true;
					}
					_ => {}
				}
				state.event(self, event)?;
			}
			if self.should_quit {
				break;
			}
			std::thread::sleep(Duration::from_millis(2));
		}
		Ok(())
	}

	pub(crate) fn set_render_target(&mut self, render_target: RenderTarget) {
		self.render_target = render_target;
		let viewport_size: Vec2<i32> = match render_target {
			RenderTarget::Window => self.window_size(),
			RenderTarget::Canvas { size } => size,
		}
		.numcast()
		.expect("viewport is too large");
		unsafe {
			self.gl.viewport(0, 0, viewport_size.x, viewport_size.y);
		}
	}

	pub(crate) fn resize(&mut self, size: Vec2<u32>) {
		let viewport_size = size.numcast().expect("window is too large");
		unsafe {
			self.gl.viewport(0, 0, viewport_size.x, viewport_size.y);
		}
	}

	pub(crate) fn global_transform(&self) -> Mat4<f32> {
		let coordinate_system_transform = match self.render_target {
			RenderTarget::Window => {
				let window_size = self.window_size();
				Mat4::scaling_3d(Vec3::new(
					2.0 / window_size.x as f32,
					-2.0 / window_size.y as f32,
					1.0,
				))
				.translated_2d(Vec2::new(-1.0, 1.0))
			}
			RenderTarget::Canvas { size } => {
				Mat4::scaling_3d(Vec3::new(2.0 / size.x as f32, 2.0 / size.y as f32, 1.0))
					.translated_2d(Vec2::new(-1.0, -1.0))
			}
		};
		self.transform_stack
			.iter()
			.fold(coordinate_system_transform, |previous, transform| {
				previous * *transform
			})
	}

	pub fn window_size(&self) -> Vec2<u32> {
		let (width, height) = self.window.size();
		Vec2::new(width, height)
	}

	pub fn clear(&self, color: Rgba) {
		unsafe {
			self.gl
				.clear_color(color.red, color.green, color.blue, color.alpha);
			self.gl.clear(glow::COLOR_BUFFER_BIT);
		}
	}

	pub fn with_transform<T>(
		&mut self,
		transform: Mat4<f32>,
		f: impl FnOnce(&mut Context) -> T,
	) -> T {
		self.transform_stack.push(transform);
		let returned_value = f(self);
		self.transform_stack.pop();
		returned_value
	}

	pub fn is_key_down(&self, scancode: Scancode) -> bool {
		self.event_pump
			.keyboard_state()
			.is_scancode_pressed(scancode)
	}

	pub fn is_mouse_button_down(&self, mouse_button: MouseButton) -> bool {
		self.event_pump
			.mouse_state()
			.is_mouse_button_pressed(mouse_button)
	}

	pub fn game_controller(&self, index: u32) -> Option<GameController> {
		match self.controller.open(index) {
			Ok(controller) => Some(GameController(controller)),
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextSettings {
	pub window_title: String,
	pub window_size: Vec2<u32>,
	pub resizable: bool,
	pub vsync: bool,
}

impl Default for ContextSettings {
	fn default() -> Self {
		Self {
			window_title: "Game".into(),
			window_size: Vec2::new(800, 600),
			resizable: false,
			vsync: true,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum RenderTarget {
	Window,
	Canvas { size: Vec2<u32> },
}

fn build_window(video: &VideoSubsystem, settings: &ContextSettings) -> Result<Window, InitError> {
	let mut window_builder = video.window(
		&settings.window_title,
		settings.window_size.x,
		settings.window_size.y,
	);
	window_builder.opengl();
	if settings.resizable {
		window_builder.resizable();
	}
	Ok(window_builder.build()?)
}

fn create_gl_context(
	video: &VideoSubsystem,
	window_size: Vec2<i32>,
) -> (Rc<glow::Context>, Texture, Shader) {
	let gl = Rc::new(unsafe {
		glow::Context::from_loader_function(|name| video.gl_get_proc_address(name) as *const _)
	});
	unsafe {
		gl.enable(glow::BLEND);
		gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
		gl.viewport(0, 0, window_size.x, window_size.y);
	}
	let default_texture = Texture::new_from_gl(
		gl.clone(),
		Vec2::new(1, 1),
		Some(&[255, 255, 255, 255]),
		TextureSettings::default(),
	)
	.expect("Error creating default texture");
	let default_shader = Shader::new_from_gl(
		gl.clone(),
		include_str!("vertex.glsl"),
		include_str!("fragment.glsl"),
	)
	.expect("Error compiling default shader");
	(gl, default_texture, default_shader)
}
