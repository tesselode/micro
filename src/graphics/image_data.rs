use std::path::Path;

use image::ImageError;
use thiserror::Error;

pub struct ImageData {
	pub size: (u32, u32),
	pub pixels: Vec<u8>,
}

impl ImageData {
	pub fn load(path: impl AsRef<Path>) -> Result<Self, LoadImageDataError> {
		let image_buffer = image::io::Reader::open(path)?.decode()?.to_rgba8();
		Ok(Self {
			size: (image_buffer.width(), image_buffer.height()),
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
