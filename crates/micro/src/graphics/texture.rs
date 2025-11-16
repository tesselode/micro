//! Types related to drawing images.

pub use wgpu::{AddressMode, FilterMode, SamplerBorderColor, TextureViewDimension};

use std::{collections::HashSet, path::Path};

use derive_more::{Display, Error, From};
use glam::{Mat4, UVec2, Vec2};
use image::{ImageBuffer, ImageError};
use palette::LinSrgba;
use wgpu::{
	Device, Extent3d, Origin3d, Queue, Sampler, SamplerDescriptor, TexelCopyBufferLayout,
	TexelCopyTextureInfo, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat,
	TextureUsages, TextureView, TextureViewDescriptor,
};

use crate::{
	Context, color::ColorConstants, graphics::BlendMode, math::Rect, standard_draw_param_methods,
};

use super::mesh::Mesh;

/// Image data that's been uploaded to the GPU and can be drawn to the screen.
///
/// This can be cheaply cloned. Clones will point to the same image data
/// on the GPU.
#[derive(Debug, Clone, PartialEq)]
pub struct Texture {
	pub(crate) texture: wgpu::Texture,
	pub(crate) view: TextureView,
	pub(crate) sampler: Sampler,
	size: UVec2,
	num_layers: u32,
	settings: TextureSettings,
	internal_settings: InternalTextureSettings,

	// draw params
	/// The portion of the texture to draw.
	pub region: Rect,
	/// The transform to use when drawing this texture.
	pub transform: Mat4,
	/// The blend color to use when drawing this texture.
	pub color: LinSrgba,
	/// The blend mode to use when drawing this texture.
	pub blend_mode: BlendMode,
}

impl Texture {
	/// Creates a new texture where all the pixels are transparent black.
	pub fn empty(ctx: &Context, size: UVec2, settings: TextureSettings) -> Self {
		let _span = tracy_client::span!();
		Self::new(
			&ctx.graphics.device,
			&ctx.graphics.queue,
			size,
			1,
			None,
			settings,
			InternalTextureSettings::default(),
		)
	}

	/// Creates a new texture from an image loaded by the [`image`] crate.
	pub fn from_image(
		ctx: &Context,
		image: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
		settings: TextureSettings,
	) -> Self {
		let _span = tracy_client::span!();
		Self::new(
			&ctx.graphics.device,
			&ctx.graphics.queue,
			UVec2::new(image.width(), image.height()),
			1,
			[image.as_raw().as_slice()],
			settings,
			InternalTextureSettings::default(),
		)
	}

	/// Creates a new texture from an image file.
	pub fn from_file(
		ctx: &Context,
		path: impl AsRef<Path>,
		settings: TextureSettings,
	) -> Result<Self, LoadTextureError> {
		let _span = tracy_client::span!();
		let image = image::ImageReader::open(path)?.decode()?.to_rgba8();
		Ok(Self::from_image(ctx, &image, settings))
	}

	/// Creates a new multi-layer texture from images loaded by the [`image`] crate.
	pub fn layered_from_images(
		ctx: &Context,
		images: &[&ImageBuffer<image::Rgba<u8>, Vec<u8>>],
		settings: TextureSettings,
	) -> Self {
		let _span = tracy_client::span!();
		assert!(!images.is_empty(), "must provide at least one image");
		let widths = images
			.iter()
			.map(|image| image.width())
			.collect::<HashSet<_>>();
		assert_eq!(widths.len(), 1, "images must all have the same width");
		let heights = images
			.iter()
			.map(|image| image.height())
			.collect::<HashSet<_>>();
		assert_eq!(heights.len(), 1, "images must all have the same height");
		let width = widths.iter().next().copied().unwrap();
		let height = heights.iter().next().copied().unwrap();
		let pixels = images.iter().map(|image| image.as_raw().as_slice());
		Self::new(
			&ctx.graphics.device,
			&ctx.graphics.queue,
			UVec2::new(width, height),
			images.len() as u32,
			pixels,
			settings,
			InternalTextureSettings::default(),
		)
	}

	/// Creates a new multi-layer texture from image files.
	pub fn layered_from_files(
		ctx: &Context,
		paths: &[impl AsRef<Path>],
		settings: TextureSettings,
	) -> Result<Self, LoadTextureError> {
		let _span = tracy_client::span!();
		let images = paths
			.iter()
			.map(|path| -> Result<_, LoadTextureError> {
				Ok(image::ImageReader::open(path.as_ref())?
					.decode()?
					.to_rgba8())
			})
			.collect::<Result<Vec<_>, _>>()?;
		let image_refs = images.iter().collect::<Vec<_>>();
		Ok(Self::layered_from_images(ctx, &image_refs, settings))
	}

	/// Creates a new cubemap texture from images loaded by the [`image`] crate.
	pub fn cubemap_from_images(
		ctx: &Context,
		images: Cubemap<&ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
		settings: CubemapSettings,
	) -> Self {
		Self::layered_from_images(
			ctx,
			&[
				images.right,
				images.left,
				images.top,
				images.bottom,
				images.front,
				images.back,
			],
			settings.into(),
		)
	}

	/// Creates a new cubemap texture from image files.
	pub fn cubemap_from_files(
		ctx: &Context,
		paths: Cubemap<impl AsRef<Path>>,
		settings: CubemapSettings,
	) -> Result<Self, LoadTextureError> {
		Self::layered_from_files(
			ctx,
			&[
				paths.right,
				paths.left,
				paths.top,
				paths.bottom,
				paths.front,
				paths.back,
			],
			settings.into(),
		)
	}

	/// Returns a new texture with the specified `size` and with data copied
	/// over from the previous texture.
	pub fn resized(&self, ctx: &Context, size: UVec2) -> Self {
		let mut encoder = ctx
			.graphics
			.device
			.create_command_encoder(&Default::default());
		let new_texture = Texture::new(
			&ctx.graphics.device,
			&ctx.graphics.queue,
			size,
			self.num_layers,
			None,
			self.settings.clone(),
			self.internal_settings,
		);
		let copy_size = self.size.min(size);
		encoder.copy_texture_to_texture(
			TexelCopyTextureInfo {
				texture: &self.texture,
				mip_level: 0,
				origin: Origin3d::ZERO,
				aspect: TextureAspect::All,
			},
			TexelCopyTextureInfo {
				texture: &new_texture.texture,
				mip_level: 0,
				origin: Origin3d::ZERO,
				aspect: TextureAspect::All,
			},
			Extent3d {
				width: copy_size.x,
				height: copy_size.y,
				depth_or_array_layers: self.num_layers,
			},
		);
		ctx.graphics.queue.submit([encoder.finish()]);
		new_texture
	}

	/// Sets the portion of the texture to draw.
	pub fn region(&self, region: Rect) -> Self {
		let mut new = self.clone();
		new.region = region;
		new
	}

	standard_draw_param_methods!();

	/// Returns the size of the texture in pixels.
	pub fn size(&self) -> UVec2 {
		self.size
	}

	/// Returns the kind of view the texture uses.
	pub fn view_dimension(&self) -> TextureViewDimension {
		self.settings.view_dimension
	}

	/// Turns a rectangular region in pixels into a rectangular region
	/// in the 0-1 range.
	pub fn relative_rect(&self, absolute_rect: Rect) -> Rect {
		let size = self.size.as_vec2();
		Rect::from_corners(
			absolute_rect.top_left / size,
			absolute_rect.bottom_right() / size,
		)
	}

	/// Overwrites the pixels in a rectangular region with the specified
	/// top left corner.
	///
	/// This will modify all clones of this [`Texture`] as well.
	pub fn replace(
		&self,
		ctx: &Context,
		top_left: UVec2,
		image: &ImageBuffer<image::Rgba<u8>, Vec<u8>>,
	) {
		let _span = tracy_client::span!();
		let texture_extent = Extent3d {
			width: image.width(),
			height: image.height(),
			depth_or_array_layers: 1,
		};
		ctx.graphics.queue.write_texture(
			TexelCopyTextureInfo {
				texture: &self.texture,
				mip_level: 0,
				origin: Origin3d {
					x: top_left.x,
					y: top_left.y,
					z: 0,
				},
				aspect: TextureAspect::All,
			},
			image.as_raw(),
			TexelCopyBufferLayout {
				offset: 0,
				bytes_per_row: Some(4 * image.width()),
				rows_per_image: Some(image.height()),
			},
			texture_extent,
		);
	}

	/// Draws the texture.
	pub fn draw(&self, ctx: &mut Context) {
		let _span = tracy_client::span!();
		Mesh::rectangle_with_texture_region(
			ctx,
			Rect::new(Vec2::ZERO, self.region.size),
			self.relative_rect(self.region),
		)
		.texture(self)
		.transformed(self.transform)
		.color(self.color)
		.blend_mode(self.blend_mode)
		.draw(ctx)
	}

	pub(crate) fn new<'a>(
		device: &Device,
		queue: &Queue,
		size: UVec2,
		num_layers: u32,
		pixels: impl IntoIterator<Item = &'a [u8]>,
		settings: TextureSettings,
		internal_settings: InternalTextureSettings,
	) -> Self {
		let texture_extent = Extent3d {
			width: size.x,
			height: size.y,
			depth_or_array_layers: num_layers,
		};
		let texture = device.create_texture(&TextureDescriptor {
			label: Some(&settings.label),
			size: texture_extent,
			mip_level_count: 1,
			sample_count: internal_settings.sample_count,
			dimension: TextureDimension::D2,
			format: internal_settings.format,
			usage: TextureUsages::TEXTURE_BINDING
				| TextureUsages::COPY_SRC
				| TextureUsages::COPY_DST
				| TextureUsages::RENDER_ATTACHMENT,
			view_formats: &[],
		});
		for (layer_index, layer) in pixels.into_iter().enumerate() {
			queue.write_texture(
				TexelCopyTextureInfo {
					texture: &texture,
					mip_level: 0,
					origin: Origin3d {
						x: 0,
						y: 0,
						z: layer_index as u32,
					},
					aspect: TextureAspect::All,
				},
				layer,
				TexelCopyBufferLayout {
					offset: 0,
					bytes_per_row: Some(4 * size.x),
					rows_per_image: Some(size.y),
				},
				Extent3d {
					width: size.x,
					height: size.y,
					depth_or_array_layers: 1,
				},
			);
		}
		let view = texture.create_view(&TextureViewDescriptor {
			label: Some(&format!("{} - view", &settings.label)),
			dimension: Some(settings.view_dimension),
			..Default::default()
		});
		let sampler = device.create_sampler(&SamplerDescriptor {
			label: Some(&format!("{} - sampler", &settings.label)),
			address_mode_u: settings.address_mode_x,
			address_mode_v: settings.address_mode_y,
			address_mode_w: settings.address_mode_z,
			mag_filter: settings.magnifying_filter,
			min_filter: settings.minifying_filter,
			border_color: Some(settings.border_color),
			..Default::default()
		});
		Self {
			texture,
			view,
			sampler,
			size,
			num_layers,
			settings,
			internal_settings,
			region: Rect::new(Vec2::ZERO, size.as_vec2()),
			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
			blend_mode: BlendMode::default(),
		}
	}
}

/// Settings for a [`Texture`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serializing", serde(default))]
pub struct TextureSettings {
	/// A name for the texture.
	///
	/// This is visible in graphics debugging tools like RenderDoc.
	pub label: String,
	/// What should happen when reading beyond the left or right edges
	/// of the texture.
	pub address_mode_x: AddressMode,
	/// What should happen when reading beyond the top or bottom edges
	/// of the texture.
	pub address_mode_y: AddressMode,
	/// What should happen when reading beyond the first or last layer
	/// of a multilayer texture.
	pub address_mode_z: AddressMode,
	/// What color should be read when reading out of bounds and using
	/// [`AddressMode::ClampToBorder`].
	pub border_color: SamplerBorderColor,
	/// What kind of filtering should be applied when scaling the
	/// texture down.
	pub minifying_filter: FilterMode,
	/// What kind of filtering should be applied when scaling the
	/// texture up.
	pub magnifying_filter: FilterMode,
	/// What kind of view to use for the texture.
	pub view_dimension: TextureViewDimension,
}

impl Default for TextureSettings {
	fn default() -> Self {
		Self {
			label: "Texture".to_string(),
			address_mode_x: Default::default(),
			address_mode_y: Default::default(),
			address_mode_z: Default::default(),
			border_color: SamplerBorderColor::TransparentBlack,
			minifying_filter: Default::default(),
			magnifying_filter: Default::default(),
			view_dimension: Default::default(),
		}
	}
}

/// Settings for a cubemap [`Texture`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serializing", serde(default))]
pub struct CubemapSettings {
	/// A name for the texture.
	///
	/// This is visible in graphics debugging tools like RenderDoc.
	pub label: String,
	/// What should happen when reading beyond the left or right edges
	/// of the texture.
	pub address_mode_x: AddressMode,
	/// What should happen when reading beyond the top or bottom edges
	/// of the texture.
	pub address_mode_y: AddressMode,
	/// What should happen when reading beyond the first or last layer
	/// of a multilayer texture.
	pub address_mode_z: AddressMode,
	/// What color should be read when reading out of bounds and using
	/// [`AddressMode::ClampToBorder`].
	pub border_color: SamplerBorderColor,
	/// What kind of filtering should be applied when scaling the
	/// texture down.
	pub minifying_filter: FilterMode,
	/// What kind of filtering should be applied when scaling the
	/// texture up.
	pub magnifying_filter: FilterMode,
}

impl Default for CubemapSettings {
	fn default() -> Self {
		Self {
			label: "Texture".to_string(),
			address_mode_x: Default::default(),
			address_mode_y: Default::default(),
			address_mode_z: Default::default(),
			border_color: SamplerBorderColor::TransparentBlack,
			minifying_filter: Default::default(),
			magnifying_filter: Default::default(),
		}
	}
}

impl From<CubemapSettings> for TextureSettings {
	fn from(
		CubemapSettings {
			label,
			address_mode_x,
			address_mode_y,
			address_mode_z,
			border_color,
			minifying_filter,
			magnifying_filter,
		}: CubemapSettings,
	) -> Self {
		Self {
			label,
			address_mode_x,
			address_mode_y,
			address_mode_z,
			border_color,
			minifying_filter,
			magnifying_filter,
			view_dimension: TextureViewDimension::Cube,
		}
	}
}

/// An error that can occur when loading a texture.
#[derive(Debug, Error, Display, From)]
pub enum LoadTextureError {
	/// An error loading a texture from a file.
	IoError(std::io::Error),
	/// An error interpreting the image data.
	ImageError(ImageError),
}

/// A map of cube faces to values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub struct Cubemap<T> {
	/// The right (+X) face of the cube.
	pub right: T,
	/// The left (-X) face of the cube.
	pub left: T,
	/// The top (+Y) face of the cube.
	pub top: T,
	/// The bottom (-Y) face of the cube.
	pub bottom: T,
	/// The front (+Z) face of the cube.
	pub front: T,
	/// The front (-Z) face of the cube.
	pub back: T,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct InternalTextureSettings {
	pub(crate) format: TextureFormat,
	pub(crate) sample_count: u32,
}

impl Default for InternalTextureSettings {
	fn default() -> Self {
		Self {
			format: TextureFormat::Rgba8UnormSrgb,
			sample_count: 1,
		}
	}
}
