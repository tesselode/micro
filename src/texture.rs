use std::sync::Arc;

use glow::{HasContext, NativeTexture};

use crate::image_data::ImageData;

pub struct RawTexture {
	gl: Arc<glow::Context>,
	texture: NativeTexture,
}

impl RawTexture {
	pub fn new(gl: Arc<glow::Context>, image_data: &ImageData) -> Result<Self, String> {
		let texture = unsafe { gl.create_texture()? };
		unsafe {
			gl.tex_image_2d(
				glow::TEXTURE_2D,
				0,
				glow::RGBA as i32,
				image_data.0.width().try_into().expect("Image is too wide"),
				image_data.0.height().try_into().expect("Image is too tall"),
				0,
				glow::RGBA,
				glow::UNSIGNED_BYTE,
				Some(image_data.0.as_raw()),
			);
		}
		Ok(Self { gl, texture })
	}
}

impl Drop for RawTexture {
	fn drop(&mut self) {
		unsafe {
			self.gl.delete_texture(self.texture);
		}
	}
}
