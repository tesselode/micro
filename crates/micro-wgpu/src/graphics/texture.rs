pub use wgpu::{AddressMode, FilterMode, SamplerBorderColor};

use std::path::Path;

use derive_more::{Display, Error, From};
use glam::{Mat4, UVec2, Vec2};
use image::{ImageBuffer, ImageError};
use palette::LinSrgba;
use wgpu::{
	Device, Extent3d, Origin3d, Queue, Sampler, SamplerDescriptor, TexelCopyBufferLayout,
	TexelCopyTextureInfo, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat,
	TextureUsages, TextureView, TextureViewDescriptor,
};

use crate::{Context, color::ColorConstants, math::Rect, standard_draw_param_methods};

use super::mesh::Mesh;

#[derive(Debug, Clone, PartialEq)]
pub struct Texture {
	pub(crate) texture: wgpu::Texture,
	pub(crate) view: TextureView,
	pub(crate) sampler: Sampler,
	size: UVec2,

	// draw params
	pub region: Rect,

	pub transform: Mat4,
	pub color: LinSrgba,
}

impl Texture {
	pub fn empty(ctx: &Context, size: UVec2, settings: TextureSettings) -> Self {
		let _span = tracy_client::span!();
		Self::new(
			&ctx.graphics.device,
			&ctx.graphics.queue,
			size,
			None,
			settings,
			InternalTextureSettings::default(),
		)
	}

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
			Some(image.as_raw()),
			settings,
			InternalTextureSettings::default(),
		)
	}

	pub fn from_file(
		ctx: &Context,
		path: impl AsRef<Path>,
		settings: TextureSettings,
	) -> Result<Self, LoadTextureError> {
		let _span = tracy_client::span!();
		let image = image::ImageReader::open(path)?.decode()?.to_rgba8();
		Ok(Self::from_image(ctx, &image, settings))
	}

	pub fn region(&self, region: Rect) -> Self {
		let mut new = self.clone();
		new.region = region;
		new
	}

	standard_draw_param_methods!();

	pub fn size(&self) -> UVec2 {
		self.size
	}

	pub fn relative_rect(&self, absolute_rect: Rect) -> Rect {
		let size = self.size.as_vec2();
		Rect::from_corners(
			absolute_rect.top_left / size,
			absolute_rect.bottom_right() / size,
		)
	}

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
		.draw(ctx);
	}

	pub(crate) fn new(
		device: &Device,
		queue: &Queue,
		size: UVec2,
		pixels: Option<&[u8]>,
		settings: TextureSettings,
		internal_settings: InternalTextureSettings,
	) -> Self {
		let texture_extent = Extent3d {
			width: size.x,
			height: size.y,
			depth_or_array_layers: 1,
		};
		let texture = device.create_texture(&TextureDescriptor {
			label: None,
			size: texture_extent,
			mip_level_count: 1,
			sample_count: internal_settings.sample_count,
			dimension: TextureDimension::D2,
			format: TextureFormat::Rgba8UnormSrgb,
			usage: TextureUsages::TEXTURE_BINDING
				| TextureUsages::COPY_DST
				| TextureUsages::RENDER_ATTACHMENT,
			view_formats: &[],
		});
		if let Some(pixels) = pixels {
			queue.write_texture(
				TexelCopyTextureInfo {
					texture: &texture,
					mip_level: 0,
					origin: Origin3d::ZERO,
					aspect: TextureAspect::All,
				},
				pixels,
				TexelCopyBufferLayout {
					offset: 0,
					bytes_per_row: Some(4 * size.x),
					rows_per_image: Some(size.y),
				},
				texture_extent,
			);
		}
		let view = texture.create_view(&TextureViewDescriptor::default());
		let sampler = device.create_sampler(&SamplerDescriptor {
			label: None,
			address_mode_u: settings.address_mode_x,
			address_mode_v: settings.address_mode_y,
			address_mode_w: AddressMode::default(),
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
			region: Rect::new(Vec2::ZERO, size.as_vec2()),

			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serializing", serde(default))]
pub struct TextureSettings {
	pub address_mode_x: AddressMode,
	pub address_mode_y: AddressMode,
	pub border_color: SamplerBorderColor,
	pub minifying_filter: FilterMode,
	pub magnifying_filter: FilterMode,
}

impl Default for TextureSettings {
	fn default() -> Self {
		Self {
			address_mode_x: Default::default(),
			address_mode_y: Default::default(),
			border_color: SamplerBorderColor::TransparentBlack,
			minifying_filter: Default::default(),
			magnifying_filter: Default::default(),
		}
	}
}

#[derive(Debug, Error, Display, From)]
pub enum LoadTextureError {
	IoError(std::io::Error),
	ImageError(ImageError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct InternalTextureSettings {
	pub(crate) sample_count: u32,
}

impl Default for InternalTextureSettings {
	fn default() -> Self {
		Self { sample_count: 1 }
	}
}
