use std::{error::Error, fmt::Display};

use image::ImageError;

#[derive(Debug)]
pub enum LoadImageDataError {
	IoError(std::io::Error),
	ImageError(ImageError),
}

impl Display for LoadImageDataError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			LoadImageDataError::IoError(error) => error.fmt(f),
			LoadImageDataError::ImageError(error) => error.fmt(f),
		}
	}
}

impl Error for LoadImageDataError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			LoadImageDataError::IoError(error) => Some(error),
			LoadImageDataError::ImageError(error) => Some(error),
		}
	}
}

impl From<std::io::Error> for LoadImageDataError {
	fn from(v: std::io::Error) -> Self {
		Self::IoError(v)
	}
}

impl From<ImageError> for LoadImageDataError {
	fn from(v: ImageError) -> Self {
		Self::ImageError(v)
	}
}
