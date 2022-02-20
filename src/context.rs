use std::rc::Rc;

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
}

impl Context {
	pub fn new(video: &VideoSubsystem) -> Self {
		let gl = Rc::new(unsafe {
			glow::Context::from_loader_function(|name| video.gl_get_proc_address(name) as *const _)
		});
		let default_texture = Texture {
			raw_texture: Rc::new(
				RawTexture::new(
					gl.clone(),
					&ImageData({
						let mut rgba_image = RgbaImage::new(1, 1);
						rgba_image.put_pixel(0, 0, image::Rgba([255, 255, 255, 255]));
						rgba_image
					}),
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
		}
	}

	pub fn clear(&self, color: Rgba) {
		unsafe {
			self.gl
				.clear_color(color.red, color.green, color.blue, color.alpha);
			self.gl.clear(glow::COLOR_BUFFER_BIT);
		}
	}
}
