mod error;

pub use error::*;
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
	EventPump, Sdl, VideoSubsystem,
};

use crate::{
	graphics::{
		color::Rgba,
		shader::Shader,
		texture::{Texture, TextureSettings},
	},
	State,
};

const MAX_TRANSFORM_STACK_DEPTH: usize = 256;

pub struct Context {
	_sdl: Sdl,
	_video: VideoSubsystem,
	window: Window,
	_sdl_gl_ctx: GLContext,
	event_pump: EventPump,
	should_quit: bool,
	pub(crate) gl: Rc<glow::Context>,
	pub(crate) default_texture: Texture,
	pub(crate) default_shader: Shader,
	pub(crate) transform_stack: Vec<Mat4<f32>>,
}

impl Context {
	pub fn new(settings: ContextSettings) -> Result<Self, InitError> {
		let sdl = sdl2::init()?;
		let video = sdl.video()?;
		let gl_attr = video.gl_attr();
		gl_attr.set_context_profile(GLProfile::Core);
		gl_attr.set_context_version(3, 3);
		let window = build_window(&video, &settings)?;
		let (window_width, window_height) = window.size();
		let _sdl_gl_ctx = window.gl_create_context()?;
		video
			.gl_set_swap_interval(if settings.vsync {
				SwapInterval::VSync
			} else {
				SwapInterval::Immediate
			})
			.expect("Could not set vsync");
		let event_pump = sdl.event_pump()?;
		let (gl, default_texture, default_shader) =
			create_gl_context(&video, window_width, window_height);
		Ok(Self {
			_sdl: sdl,
			_video: video,
			window,
			_sdl_gl_ctx,
			event_pump,
			should_quit: false,
			gl,
			default_texture,
			default_shader,
			transform_stack: vec![],
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
						self.resize(width as u32, height as u32);
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

	pub(crate) fn resize(&mut self, window_width: u32, window_height: u32) {
		unsafe {
			self.gl
				.viewport(0, 0, window_width as i32, window_height as i32);
		}
	}

	pub(crate) fn global_transform(&self) -> Mat4<f32> {
		let (window_width, window_height) = self.window.size();
		self.transform_stack.iter().fold(
			coordinate_system_transform(window_width, window_height),
			|previous, transform| previous * *transform,
		)
	}

	pub fn window_size(&self) -> (u32, u32) {
		self.window.size()
	}

	pub fn clear(&self, color: Rgba) {
		unsafe {
			self.gl
				.clear_color(color.red, color.green, color.blue, color.alpha);
			self.gl.clear(glow::COLOR_BUFFER_BIT);
		}
	}

	pub fn push_transform(
		&mut self,
		transform: Mat4<f32>,
	) -> Result<(), MaximumTransformStackDepthReached> {
		if self.transform_stack.len() == MAX_TRANSFORM_STACK_DEPTH {
			return Err(MaximumTransformStackDepthReached);
		}
		self.transform_stack.push(transform);
		Ok(())
	}

	pub fn pop_transform(&mut self) -> Result<(), NoTransformToPop> {
		if self.transform_stack.is_empty() {
			return Err(NoTransformToPop);
		}
		self.transform_stack.pop();
		Ok(())
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

	pub fn quit(&mut self) {
		self.should_quit = true;
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextSettings {
	pub window_title: String,
	pub window_size: (u32, u32),
	pub resizable: bool,
	pub vsync: bool,
}

impl Default for ContextSettings {
	fn default() -> Self {
		Self {
			window_title: "Game".into(),
			window_size: (800, 600),
			resizable: false,
			vsync: true,
		}
	}
}

fn build_window(video: &VideoSubsystem, settings: &ContextSettings) -> Result<Window, InitError> {
	let mut window_builder = video.window(
		&settings.window_title,
		settings.window_size.0,
		settings.window_size.1,
	);
	window_builder.opengl();
	if settings.resizable {
		window_builder.resizable();
	}
	Ok(window_builder.build()?)
}

fn create_gl_context(
	video: &VideoSubsystem,
	window_width: u32,
	window_height: u32,
) -> (Rc<glow::Context>, Texture, Shader) {
	let gl = Rc::new(unsafe {
		glow::Context::from_loader_function(|name| video.gl_get_proc_address(name) as *const _)
	});
	unsafe {
		gl.enable(glow::BLEND);
		gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
		gl.viewport(0, 0, window_width as i32, window_height as i32);
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

fn coordinate_system_transform(window_width: u32, window_height: u32) -> Mat4<f32> {
	Mat4::scaling_3d(Vec3::new(
		2.0 / window_width as f32,
		-2.0 / window_height as f32,
		1.0,
	))
	.translated_2d(Vec2::new(-1.0, 1.0))
}
