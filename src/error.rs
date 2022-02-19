use std::{
	error::Error,
	fmt::{Display, Formatter},
};

use sdl2::video::WindowBuildError;

#[derive(Debug, Clone)]
pub enum InitError {
	InitSdlError(String),
	WindowBuildError(WindowBuildError),
}

impl Display for InitError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			InitError::InitSdlError(error) => f.write_str(error),
			InitError::WindowBuildError(error) => error.fmt(f),
		}
	}
}

impl Error for InitError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			InitError::WindowBuildError(error) => Some(error),
			_ => None,
		}
	}
}

impl From<String> for InitError {
	fn from(v: String) -> Self {
		Self::InitSdlError(v)
	}
}

impl From<WindowBuildError> for InitError {
	fn from(v: WindowBuildError) -> Self {
		Self::WindowBuildError(v)
	}
}
