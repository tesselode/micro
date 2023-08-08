use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error)]
#[error("{0}")]
pub struct SdlError(pub String);

impl From<String> for SdlError {
	fn from(value: String) -> Self {
		Self(value)
	}
}
