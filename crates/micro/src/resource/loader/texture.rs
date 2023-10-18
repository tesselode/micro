use crate::{
	graphics::{
		texture::{Texture, TextureSettings},
		LoadImageDataError,
	},
	Context,
};

use super::ResourceLoader;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TextureLoader {
	pub default_settings: TextureSettings,
}

impl ResourceLoader for TextureLoader {
	type Resource = Texture;

	type Error = LoadImageDataError;

	type Settings = TextureSettings;

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["png"];

	fn load(
		&mut self,
		ctx: &mut Context,
		path: &std::path::Path,
		settings: Option<Self::Settings>,
	) -> Result<Self::Resource, Self::Error> {
		Texture::from_file(ctx, path, settings.unwrap_or(self.default_settings))
	}
}
