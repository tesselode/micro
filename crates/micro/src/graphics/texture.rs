use std::{num::NonZeroU32, path::Path, rc::Rc};

use wgpu::{
	AddressMode, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource,
	Device, Extent3d, FilterMode, ImageCopyTexture, ImageDataLayout, Origin3d, Queue,
	SamplerDescriptor, TextureAspect, TextureDescriptor, TextureDimension, TextureFormat,
	TextureUsages,
};

use crate::Context;

use super::image_data::{ImageData, LoadImageDataError};

#[derive(Clone)]
pub struct Texture(pub(crate) Rc<TextureInner>);

impl Texture {
	pub fn from_file(ctx: &Context, path: impl AsRef<Path>) -> Result<Self, LoadImageDataError> {
		Ok(Self::from_image_data(ctx, &ImageData::load(path)?))
	}

	pub fn from_image_data(ctx: &Context, image_data: &ImageData) -> Self {
		Self::from_image_data_internal(
			image_data,
			&ctx.graphics_ctx.device,
			&ctx.graphics_ctx.queue,
			&ctx.graphics_ctx.texture_bind_group_layout,
		)
	}

	pub(crate) fn from_image_data_internal(
		image_data: &ImageData,
		device: &Device,
		queue: &Queue,
		texture_bind_group_layout: &BindGroupLayout,
	) -> Self {
		let texture_size = Extent3d {
			width: image_data.size.x,
			height: image_data.size.y,
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
				bytes_per_row: NonZeroU32::new(4 * image_data.size.x),
				rows_per_image: NonZeroU32::new(image_data.size.y),
			},
			texture_size,
		);
		let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
		let sampler = device.create_sampler(&SamplerDescriptor {
			address_mode_u: AddressMode::ClampToEdge,
			address_mode_v: AddressMode::ClampToEdge,
			address_mode_w: AddressMode::ClampToEdge,
			mag_filter: FilterMode::Linear,
			min_filter: FilterMode::Nearest,
			mipmap_filter: FilterMode::Nearest,
			..Default::default()
		});
		let bind_group = device.create_bind_group(&BindGroupDescriptor {
			layout: texture_bind_group_layout,
			entries: &[
				BindGroupEntry {
					binding: 0,
					resource: BindingResource::TextureView(&texture_view),
				},
				BindGroupEntry {
					binding: 1,
					resource: BindingResource::Sampler(&sampler),
				},
			],
			label: Some("texture_bind_group"),
		});
		Self(Rc::new(TextureInner { bind_group }))
	}
}

pub(crate) struct TextureInner {
	pub(crate) bind_group: BindGroup,
}
