use std::path::Path;

use image::ImageError;
use thiserror::Error;

use crate::color::Rgba;

pub struct ImageData {
	pub width: u32,
	pub height: u32,
	pub pixels: Vec<u8>,
}

impl ImageData {
	pub fn load(path: impl AsRef<Path>) -> Result<Self, LoadImageDataError> {
		let image_buffer = image::io::Reader::open(path)?.decode()?.to_rgba8();
		Ok(Self {
			width: image_buffer.width(),
			height: image_buffer.height(),
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
