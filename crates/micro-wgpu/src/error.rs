use derive_more::derive::{Display, Error};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Error, Display)]
pub struct SdlError(#[error(not(source))] pub String);

impl From<String> for SdlError {
	fn from(value: String) -> Self {
		Self(value)
	}
}
