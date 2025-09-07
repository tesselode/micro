use std::{collections::HashMap, path::Path};

use crate::animation::{AnimationData, LoadAnimationDataError};

use super::AssetLoader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct AnimationDataLoader;

impl AssetLoader for AnimationDataLoader {
	type Asset = AnimationData;

	type Error = LoadAnimationDataError;

	type Settings = ();

	type Context = ();

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["json"];

	fn load(
		&mut self,
		_ctx: &mut (),
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Asset, Self::Error> {
		AnimationData::from_file(path)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MultipleAnimationDataLoader;

impl AssetLoader for MultipleAnimationDataLoader {
	type Asset = HashMap<String, AnimationData>;

	type Error = LoadAnimationDataError;

	type Settings = ();

	type Context = ();

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["json"];

	fn load(
		&mut self,
		_ctx: &mut (),
		path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Asset, Self::Error> {
		AnimationData::multiple_from_file(path)
	}
}
