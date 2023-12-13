use glam::{IVec2, UVec2};
use palette::{rgb::channels::Rgba, Srgba};

use crate::{
	graphics::{
		texture::{Texture, TextureSettings},
		ImageData, LoadImageDataError,
	},
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

	type Error = LoadImageDataError;

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
		resource.replace(ctx, IVec2::ZERO, &ImageData::from_file(path)?);
		Ok(())
	}

	fn placeholder(&mut self, ctx: &mut Context) -> Option<Self::Resource> {
		let color = Srgba::from_u32::<Rgba>(0xe93cfcff);
		let num_pixels = self.placeholder_texture_size.x * self.placeholder_texture_size.y;
		let image_data = ImageData {
			size: self.placeholder_texture_size,
			pixels: [color.red, color.green, color.blue, color.alpha]
				.iter()
				.copied()
				.cycle()
				.take(num_pixels as usize * 4)
				.collect(),
		};
		Some(Texture::from_image_data(
			ctx,
			&image_data,
			self.default_settings,
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
