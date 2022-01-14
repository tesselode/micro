use std::{error::Error, rc::Rc, time::Duration};

use glow::HasContext;
use sdl2::{
	event::Event,
	video::{GLContext, Window},
	EventPump, Sdl, VideoSubsystem,
};

use crate::{shader::Shader, texture::Texture, Game};

const VERTEX_SHADER: &str = include_str!("vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("fragment.glsl");

pub struct GraphicsContext {
	_sdl_gl_ctx: GLContext,
	gl: Rc<glow::Context>,
	pub(crate) default_texture: Texture,
	pub(crate) shader: Shader,
}

impl GraphicsContext {
	fn new(video: &VideoSubsystem, window: &Window) -> Result<Self, Box<dyn Error>> {
		let _sdl_gl_ctx = window.gl_create_context()?;
		let gl = Rc::new(unsafe {
			glow::Context::from_loader_function(|name| video.gl_get_proc_address(name) as *const _)
		});
		let default_texture;
		unsafe {
			// create a default texture
			default_texture = gl.create_texture()?;
			gl.bind_texture(glow::TEXTURE_2D, Some(default_texture));
			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_S,
				glow::REPEAT.try_into().unwrap(),
			);
			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_WRAP_T,
				glow::REPEAT.try_into().unwrap(),
			);
			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MIN_FILTER,
				glow::LINEAR_MIPMAP_LINEAR.try_into().unwrap(),
			);
			gl.tex_parameter_i32(
				glow::TEXTURE_2D,
				glow::TEXTURE_MAG_FILTER,
				glow::LINEAR.try_into().unwrap(),
			);
			gl.tex_image_2d(
				glow::TEXTURE_2D,
				0,
				glow::RGBA.try_into().unwrap(),
				1,
				1,
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				Some(&[255; 4]),
			);

			// enable blending
			gl.enable(glow::BLEND);
			gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
		}
		Ok(Self {
			_sdl_gl_ctx,
			gl: gl.clone(),
			default_texture: Texture::from_native_texture(gl.clone(), default_texture),
			shader: Shader::from_ctx_and_strs(gl, VERTEX_SHADER, FRAGMENT_SHADER)?,
		})
	}

	pub(crate) fn gl(&self) -> Rc<glow::Context> {
		self.gl.clone()
	}

	pub(crate) fn default_texture(&self) -> Texture {
		self.default_texture.clone()
	}

	pub fn clear(&self, red: f32, green: f32, blue: f32, alpha: f32) {
		unsafe {
			self.gl.clear_color(red, green, blue, alpha);
			self.gl.clear(glow::COLOR_BUFFER_BIT);
		}
	}
}

pub struct Context {
	_sdl: Sdl,
	video: VideoSubsystem,
	event_pump: EventPump,
	window: Window,
	graphics_ctx: GraphicsContext,
}

impl Context {
	pub fn new() -> Result<Self, Box<dyn Error>> {
		let sdl = sdl2::init()?;
		let video = sdl.video()?;
		let gl_attr = video.gl_attr();
		gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
		gl_attr.set_context_version(3, 3);
		let window = video.window("Window", 800, 600).opengl().build()?;
		let graphics_ctx = GraphicsContext::new(&video, &window)?;
		let event_pump = sdl.event_pump()?;
		Ok(Self {
			_sdl: sdl,
			video,
			event_pump,
			window,
			graphics_ctx,
		})
	}

	pub fn graphics(&self) -> &GraphicsContext {
		&self.graphics_ctx
	}

	pub fn run<E>(&mut self, mut game: impl Game<E>) -> Result<(), E> {
		loop {
			while let Some(event) = self.event_pump.poll_event() {
				if let Event::Quit { .. } = event {
					return Ok(());
				}
				game.event(self, event)?;
			}
			game.update(self)?;
			game.draw(self)?;
			self.window.gl_swap_window();
			std::thread::sleep(Duration::from_millis(2));
		}
	}
}
