use sdl2::video::WindowBuildError;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
#[error("{0}")]
pub struct GlError(pub String);

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
