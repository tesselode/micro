use glam::{IVec2, UVec2};
use image::ImageBuffer;
use palette::{rgb::channels::Rgba, Srgba};

use crate::{
	graphics::texture::{LoadTextureError, Texture, TextureSettings},
	Context,
};

use super::ResourceLoader;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextureLoader {
	pub default_settings: TextureSettings,
	pub placeholder_texture_size: UVec2,
}

impl ResourceLoader for TextureLoader {
	type Resource = Texture;

	type Error = LoadTextureError;

	type Settings = TextureSettings;

	const SUPPORTED_FILE_EXTENSIONS: &'static [&'static str] = &["png"];

	fn load(
		&mut self,
		ctx: &mut Context,
		path: &std::path::Path,
		settings: Option<&Self::Settings>,
	) -> Result<Self::Resource, Self::Error> {
		Texture::from_file(
			ctx,
			path,
			settings.copied().unwrap_or(self.default_settings),
		)
	}

	fn reload(
		&mut self,
		ctx: &mut Context,
		resource: &mut Self::Resource,
		path: &std::path::Path,
		_settings: Option<&Self::Settings>,
	) -> Result<(), Self::Error> {
		let image = image::io::Reader::open(path)?.decode()?.to_rgba8();
		resource.replace(ctx, IVec2::ZERO, &image);
		Ok(())
	}

	fn placeholder(&mut self, ctx: &mut Context) -> Option<Self::Resource> {
		let color = Srgba::from_u32::<Rgba>(0xe93cfcff);
		let image = ImageBuffer::from_pixel(
			self.placeholder_texture_size.x,
			self.placeholder_texture_size.y,
			image::Rgba(color.into()),
		);
		Some(Texture::from_image(ctx, &image, self.default_settings))
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
