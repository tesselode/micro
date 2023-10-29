use std::path::Path;

use thiserror::Error;

use crate::{animation::AnimationData, Context};

use super::ResourceLoader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AnimationDataLoader;

impl ResourceLoader for AnimationDataLoader {
	type Resource = AnimationData;

	type Error = LoadAnimationDataError;

	type Settings = ();

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["json"];

	fn load(
		&mut self,
		_ctx: &mut Context,
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Resource, Self::Error> {
		Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
	}
}

#[derive(Debug, Error)]
pub enum LoadAnimationDataError {
	#[error("{0}")]
	IoError(#[from] std::io::Error),
	#[error("{0}")]
	ParseError(#[from] serde_json::Error),
}
