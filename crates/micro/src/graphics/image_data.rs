use std::path::Path;

use glam::UVec2;
use image::ImageError;
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
}

#[derive(Debug, Error)]
pub enum LoadImageDataError {
	#[error("{0}")]
	IoError(#[from] std::io::Error),
	#[error("{0}")]
	ImageError(#[from] ImageError),
}
