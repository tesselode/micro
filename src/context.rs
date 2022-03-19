use std::{
	rc::Rc,
	time::{Duration, Instant},
};

use glam::{Mat4, Vec3};
use glow::HasContext;
use sdl2::{
	event::{Event, WindowEvent},
	video::{GLContext, GLProfile, SwapInterval, Window},
	EventPump, Sdl, VideoSubsystem,
};

use crate::{
	error::InitError,
	graphics::{
		color::Rgba,
		shader::Shader,
		texture::{Texture, TextureSettings},
	},
	State,
};

pub struct Context {
	_sdl: Sdl,
	_video: VideoSubsystem,
	window: Window,
	_sdl_gl_ctx: GLContext,
	event_pump: EventPump,
	pub(crate) gl: Rc<glow::Context>,
	pub(crate) default_texture: Texture,
	pub(crate) default_shader: Shader,
	pub(crate) global_transform: Mat4,
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
			gl,
			default_texture,
			default_shader,
			global_transform: global_transform(window_width, window_height),
			_sdl: sdl,
			_video: video,
			window,
			_sdl_gl_ctx,
			event_pump,
		})
	}

	pub fn run<E, S, F>(&mut self, mut state_constructor: F) -> Result<(), E>
	where
		S: State<E>,
		F: FnMut(&mut Context) -> Result<S, E>,
	{
		let mut state = state_constructor(self)?;
		let mut last_update_time = Instant::now();
		'running: loop {
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
						break 'running;
					}
					_ => {}
				}
				state.event(self, event)?;
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
		self.global_transform = global_transform(window_width, window_height);
	}

	pub fn clear(&self, color: Rgba) {
		unsafe {
			self.gl
				.clear_color(color.red, color.green, color.blue, color.alpha);
			self.gl.clear(glow::COLOR_BUFFER_BIT);
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContextSettings {
	pub window_title: String,
	pub window_width: u32,
	pub window_height: u32,
	pub resizable: bool,
	pub vsync: bool,
}

impl Default for ContextSettings {
	fn default() -> Self {
		Self {
			window_title: "Game".into(),
			window_width: 800,
			window_height: 600,
			resizable: false,
			vsync: true,
		}
	}
}

fn build_window(video: &VideoSubsystem, settings: &ContextSettings) -> Result<Window, InitError> {
	let mut window_builder = video.window(
		&settings.window_title,
		settings.window_width,
		settings.window_height,
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
		1,
		1,
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

fn global_transform(window_width: u32, window_height: u32) -> Mat4 {
	Mat4::from_translation(Vec3::new(-1.0, 1.0, 0.0))
		* Mat4::from_scale(Vec3::new(
			2.0 / window_width as f32,
			-2.0 / window_height as f32,
			1.0,
		))
}
