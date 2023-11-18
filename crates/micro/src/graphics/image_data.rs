use std::path::Path;

use glam::UVec2;
use image::{ImageBuffer, ImageError};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImageData {
	pub size: UVec2,
	pub pixels: Vec<u8>,
}

impl ImageData {
	pub fn from_file(path: impl AsRef<Path>) -> Result<Self, LoadImageDataError> {
		let image_buffer = image::io::Reader::open(path)?.decode()?.to_rgba8();
		Ok(Self {
			size: UVec2::new(image_buffer.width(), image_buffer.height()),
			pixels: image_buffer.into_raw(),
		})
	}

	pub(crate) fn flipped_vertical(&self) -> Self {
		let mut image_buffer: ImageBuffer<image::Rgba<u8>, _> =
			ImageBuffer::from_vec(self.size.x, self.size.y, self.pixels.clone()).unwrap();
		image::imageops::flip_vertical_in_place(&mut image_buffer);
		Self {
			size: self.size,
			pixels: image_buffer.into_vec(),
		}
	}
}

#[derive(Debug, Error)]
pub enum LoadImageDataError {
	#[error("{0}")]
	IoError(#[from] std::io::Error),
	#[error("{0}")]
	ImageError(#[from] ImageError),
}
