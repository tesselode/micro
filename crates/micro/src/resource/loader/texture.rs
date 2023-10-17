use crate::{
	graphics::{
		texture::{Texture, TextureSettings},
		LoadImageDataError,
	},
	Context,
};

use super::ResourceLoader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureLoader;

impl ResourceLoader for TextureLoader {
	type Resource = Texture;

	type Error = LoadImageDataError;

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["png"];

	fn load(
		&mut self,
		ctx: &mut Context,
		path: &std::path::Path,
	) -> Result<Self::Resource, Self::Error> {
		Texture::from_file(ctx, path, TextureSettings::default())
	}
}
