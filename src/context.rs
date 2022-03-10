use std::rc::Rc;

use glam::{Mat4, Vec3};
use glow::HasContext;
use image::RgbaImage;
use sdl2::VideoSubsystem;

use crate::{
	color::Rgba,
	image_data::ImageData,
	shader::{RawShader, Shader},
	texture::{RawTexture, Texture},
};

pub struct Context {
	pub(crate) gl: Rc<glow::Context>,
	pub(crate) default_texture: Texture,
	pub(crate) default_shader: Shader,
	pub(crate) global_transform: Mat4,
}

impl Context {
	pub(crate) fn new(video: &VideoSubsystem, window_width: u32, window_height: u32) -> Self {
		let gl = Rc::new(unsafe {
			glow::Context::from_loader_function(|name| video.gl_get_proc_address(name) as *const _)
		});
		unsafe {
			gl.enable(glow::BLEND);
			gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
			gl.viewport(0, 0, window_width as i32, window_height as i32);
		}
		let default_texture = Texture {
			raw_texture: Rc::new(
				RawTexture::new(
					gl.clone(),
					&ImageData {
						width: 1,
						height: 1,
						pixels: vec![255, 255, 255, 255],
					},
				)
				.expect("Error creating default texture"),
			),
		};
		let default_shader = Shader {
			raw_shader: Rc::new(
				RawShader::new(
					gl.clone(),
					include_str!("vertex.glsl"),
					include_str!("fragment.glsl"),
				)
				.expect("Error compiling default shader"),
			),
		};
		Self {
			gl,
			default_texture,
			default_shader,
			global_transform: global_transform(window_width, window_height),
		}
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

fn global_transform(window_width: u32, window_height: u32) -> Mat4 {
	Mat4::from_translation(Vec3::new(-1.0, 1.0, 0.0))
		* Mat4::from_scale(Vec3::new(
			2.0 / window_width as f32,
			-2.0 / window_height as f32,
			1.0,
		))
}
