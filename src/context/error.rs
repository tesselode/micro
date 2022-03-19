use sdl2::video::WindowBuildError;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum InitError {
	#[error("{0}")]
	InitSdlError(String),
	#[error("{0}")]
	WindowBuildError(#[from] WindowBuildError),
}

impl From<String> for InitError {
	fn from(v: String) -> Self {
		Self::InitSdlError(v)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error("The keycode does not have a corresponding scancode")]
pub struct NoKeycodeForScancode;
