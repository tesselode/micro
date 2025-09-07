use std::path::{Path, PathBuf};

use derive_more::derive::{Display, Error, From};
use serde::{Deserialize, Serialize};

use micro::graphics::text::{Font, FontSettings, LoadFontError};

use super::AssetLoader;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontLoader {
	pub base_scale: f32,
}

impl FontLoader {
	pub fn new() -> Self {
		Self { base_scale: 1.0 }
	}
}

impl Default for FontLoader {
	fn default() -> Self {
		Self::new()
	}
}

impl AssetLoader for FontLoader {
	type Asset = Font;

	type Error = LoadFontDefinitionError;

	type Settings = ();

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["font"];

	fn load(
		&mut self,
		font_definition_path: &Path,
		_settings: Option<&Self::Settings>,
	) -> Result<Self::Asset, Self::Error> {
		let font_definition_string = std::fs::read_to_string(font_definition_path)?;
		let font_definition = serde_json::from_str::<FontDefinition>(&font_definition_string)?;
		let font_path = font_definition_path
			.parent()
			.unwrap()
			.join(font_definition.relative_font_path);
		let mut settings = font_definition.settings;
		settings.scale *= self.base_scale;
		let font = Font::from_file(font_path, settings)?;
		Ok(font)
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct FontDefinition {
	#[serde(rename = "path")]
	relative_font_path: PathBuf,
	#[serde(flatten)]
	settings: FontSettings,
}

#[derive(Debug, Error, Display, From)]
pub enum LoadFontDefinitionError {
	IoError(std::io::Error),
	DefinitionError(serde_json::Error),
	LoadFontError(LoadFontError),
}
