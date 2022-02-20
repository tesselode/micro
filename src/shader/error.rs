use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum LoadShaderError {
	IoError(std::io::Error),
	ShaderError(String),
}

impl Display for LoadShaderError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			LoadShaderError::IoError(error) => error.fmt(f),
			LoadShaderError::ShaderError(error) => f.write_str(error),
		}
	}
}

impl Error for LoadShaderError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			LoadShaderError::IoError(error) => Some(error),
			_ => None,
		}
	}
}

impl From<std::io::Error> for LoadShaderError {
	fn from(v: std::io::Error) -> Self {
		Self::IoError(v)
	}
}

impl From<String> for LoadShaderError {
	fn from(v: String) -> Self {
		Self::ShaderError(v)
	}
}
