pub use wgpu::FilterMode;

use std::{path::Path, rc::Rc};

use glam::{IVec2, UVec2, Vec2};
use wgpu::{
	BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource, Device,
	Extent3d, ImageCopyTexture, ImageDataLayout, Origin3d, Queue, Sampler, SamplerDescriptor,
	TextureAspect, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TextureView,
	TextureViewDescriptor,
};

use crate::{math::Rect, Context};

use super::{
	image_data::{ImageData, LoadImageDataError},
	mesh::Mesh,
	shader::Shader,
	AddressMode, DrawParams,
};

#[derive(Clone)]
pub struct Texture(pub(crate) Rc<TextureInner>);

impl Texture {
	pub fn empty(ctx: &Context, size: UVec2, settings: TextureSettings) -> Self {
		Self::new_internal(
			None,
			size,
			&ctx.graphics_ctx.device,
			&ctx.graphics_ctx.queue,
			&ctx.graphics_ctx.texture_bind_group_layout,
			settings,
		)
	}

	pub fn from_file(
		ctx: &Context,
		path: impl AsRef<Path>,
		settings: TextureSettings,
	) -> Result<Self, LoadImageDataError> {
		Ok(Self::from_image_data(
			ctx,
			&ImageData::load(path)?,
			settings,
		))
	}

	pub fn from_image_data(
		ctx: &Context,
		image_data: &ImageData,
		settings: TextureSettings,
	) -> Self {
		Self::new_internal(
			Some(image_data),
			image_data.size,
			&ctx.graphics_ctx.device,
			&ctx.graphics_ctx.queue,
			&ctx.graphics_ctx.texture_bind_group_layout,
			settings,
		)
	}

	pub fn size(&self) -> UVec2 {
		self.0.size
	}

	pub fn relative_rect(&self, absolute_rect: Rect) -> Rect {
		let size = self.0.size.as_vec2();
		Rect {
			top_left: absolute_rect.top_left / size,
			bottom_right: absolute_rect.bottom_right / size,
		}
	}

	pub fn replace(&self, ctx: &Context, top_left: IVec2, image_data: &ImageData) {
		ctx.graphics_ctx.queue.write_texture(
			ImageCopyTexture {
				texture: &self.0.texture,
				mip_level: 0,
				origin: Origin3d {
					x: top_left.x.try_into().expect("cannot convert u32 to i32"),
					y: top_left.y.try_into().expect("cannot convert u32 to i32"),
					z: 0,
				},
				aspect: TextureAspect::All,
			},
			&image_data.pixels,
			ImageDataLayout {
				offset: 0,
				bytes_per_row: Some(4 * image_data.size.x),
				rows_per_image: Some(image_data.size.y),
			},
			Extent3d {
				width: image_data.size.x,
				height: image_data.size.y,
				depth_or_array_layers: 1,
			},
		)
	}

	pub fn draw<S: Shader>(&self, ctx: &mut Context, params: impl Into<DrawParams<S>>) {
		Mesh::rectangle(ctx, Rect::new(Vec2::ZERO, self.0.size.as_vec2()))
			.draw_textured(ctx, self, params);
	}

	pub fn draw_region<S: Shader>(
		&self,
		ctx: &mut Context,
		region: Rect,
		params: impl Into<DrawParams<S>>,
	) {
		Mesh::rectangle_with_texture_region(
			ctx,
			Rect::new(Vec2::ZERO, region.size()),
			self.relative_rect(region),
		)
		.draw_textured(ctx, self, params);
	}

	pub(crate) fn new_internal(
		image_data: Option<&ImageData>,
		size: UVec2,
		device: &Device,
		queue: &Queue,
		texture_bind_group_layout: &BindGroupLayout,
		settings: TextureSettings,
	) -> Self {
		let texture_size = Extent3d {
			width: size.x,
			height: size.y,
			depth_or_array_layers: 1,
		};
		let texture = device.create_texture(&TextureDescriptor {
			size: texture_size,
			mip_level_count: 1,
			sample_count: 1,
			dimension: TextureDimension::D2,
			format: TextureFormat::Rgba8UnormSrgb,
			usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
			label: Some("Texture"),
			view_formats: &[],
		});
		if let Some(image_data) = image_data {
			queue.write_texture(
				ImageCopyTexture {
					texture: &texture,
					mip_level: 0,
					origin: Origin3d::ZERO,
					aspect: TextureAspect::All,
				},
				&image_data.pixels,
				ImageDataLayout {
					offset: 0,
					bytes_per_row: Some(4 * image_data.size.x),
					rows_per_image: Some(image_data.size.y),
				},
				texture_size,
			);
		}
		let view = texture.create_view(&TextureViewDescriptor::default());
		let address_mode = settings.address_mode.to_wgpu_address_mode();
		let sampler = device.create_sampler(&SamplerDescriptor {
			address_mode_u: address_mode,
			address_mode_v: address_mode,
			address_mode_w: address_mode,
			mag_filter: settings.magnifying_filter,
			min_filter: settings.minifying_filter,
			mipmap_filter: FilterMode::Nearest,
			border_color: settings.address_mode.border_color(),
			..Default::default()
		});
		let bind_group = device.create_bind_group(&BindGroupDescriptor {
			layout: texture_bind_group_layout,
			entries: &[
				BindGroupEntry {
					binding: 0,
					resource: BindingResource::TextureView(&view),
				},
				BindGroupEntry {
					binding: 1,
					resource: BindingResource::Sampler(&sampler),
				},
			],
			label: Some("texture_bind_group"),
		});
		Self(Rc::new(TextureInner {
			texture,
			view,
			sampler,
			bind_group,
			size,
		}))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TextureSettings {
	pub address_mode: AddressMode,
	pub minifying_filter: FilterMode,
	pub magnifying_filter: FilterMode,
}

pub(crate) struct TextureInner {
	pub(crate) texture: wgpu::Texture,
	pub(crate) view: TextureView,
	pub(crate) sampler: Sampler,
	pub(crate) bind_group: BindGroup,
	pub(crate) size: UVec2,
}
