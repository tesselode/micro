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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error("Pushed too many transformations to the transformation stack")]
pub struct MaximumTransformStackDepthReached;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Error)]
#[error("Cannot pop a transformation because there's no transformations to pop")]
pub struct NoTransformToPop;
