use std::{
	error::Error,
	fmt::{Display, Formatter},
};

use sdl2::video::WindowBuildError;

#[derive(Debug, Clone)]
pub enum RunError {
	InitSdlError(String),
	WindowBuildError(WindowBuildError),
}

impl Display for RunError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			RunError::InitSdlError(error) => f.write_str(error),
			RunError::WindowBuildError(error) => error.fmt(f),
		}
	}
}

impl Error for RunError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			RunError::WindowBuildError(error) => Some(error),
			_ => None,
		}
	}
}

impl From<String> for RunError {
	fn from(v: String) -> Self {
		Self::InitSdlError(v)
	}
}

impl From<WindowBuildError> for RunError {
	fn from(v: WindowBuildError) -> Self {
		Self::WindowBuildError(v)
	}
}
