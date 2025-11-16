use micro::{
	Context,
	color::{Srgba, rgb::channels::Rgba},
	graphics::texture::{LoadTextureError, Texture, TextureSettings},
	image::ImageBuffer,
	math::UVec2,
};

use super::AssetLoader;

#[derive(Debug, Clone, PartialEq)]
pub struct TextureLoader {
	pub default_settings: TextureSettings,
	pub placeholder_texture_size: UVec2,
}

impl AssetLoader for TextureLoader {
	type Asset = Texture;

	type Error = LoadTextureError;

	type Settings = TextureSettings;

	type Context = Context;

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["png"];

	fn load(
		&mut self,
		ctx: &mut Context,
		path: &std::path::Path,
		settings: Option<&Self::Settings>,
	) -> Result<Self::Asset, Self::Error> {
		Texture::from_file(
			ctx,
			path,
			settings.unwrap_or(&self.default_settings).clone(),
		)
	}

	fn reload(
		&mut self,
		ctx: &mut Context,
		asset: &mut Self::Asset,
		path: &std::path::Path,
		_settings: Option<&Self::Settings>,
	) -> Result<(), Self::Error> {
		let image = image::ImageReader::open(path)?.decode()?.to_rgba8();
		asset.replace(ctx, UVec2::ZERO, &image);
		Ok(())
	}

	fn placeholder(&mut self, ctx: &mut Context) -> Option<Self::Asset> {
		let color = Srgba::from_u32::<Rgba>(0xe93cfcff);
		let image = ImageBuffer::from_pixel(
			self.placeholder_texture_size.x,
			self.placeholder_texture_size.y,
			image::Rgba(color.into()),
		);
		Some(Texture::from_image(
			ctx,
			&image,
			self.default_settings.clone(),
		))
	}
}

impl Default for TextureLoader {
	fn default() -> Self {
		Self {
			default_settings: Default::default(),
			placeholder_texture_size: UVec2::splat(16),
		}
	}
}
