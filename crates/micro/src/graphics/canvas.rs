use std::rc::Rc;

use glam::{UVec2, Vec2};
use palette::LinSrgba;
use wgpu::{
	BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource, Extent3d, FilterMode, Sampler,
	SamplerDescriptor, TextureDescriptor, TextureDimension, TextureUsages, TextureView,
	TextureViewDescriptor,
};

use crate::{math::Rect, Context};

use super::{
	mesh::Mesh, shader::Shader, util::create_depth_stencil_texture_view, AddressMode, DrawParams,
};

#[derive(Clone)]
pub struct Canvas(pub(crate) Rc<CanvasInner>);

impl Canvas {
	pub fn new(ctx: &Context, size: UVec2, settings: CanvasSettings) -> Self {
		let texture_size = Extent3d {
			width: size.x,
			height: size.y,
			depth_or_array_layers: 1,
		};
		let texture = ctx.graphics_ctx.device.create_texture(&TextureDescriptor {
			size: texture_size,
			mip_level_count: 1,
			sample_count: settings.sample_count,
			dimension: TextureDimension::D2,
			format: ctx.graphics_ctx.config.format,
			usage: TextureUsages::TEXTURE_BINDING
				| TextureUsages::COPY_DST
				| TextureUsages::RENDER_ATTACHMENT,
			label: Some("Texture"),
			view_formats: &[],
		});
		let view = texture.create_view(&TextureViewDescriptor::default());
		let address_mode = settings.address_mode.to_wgpu_address_mode();
		let (bind_group, multisample_resolve_texture_view, sampler) = if settings.sample_count > 1 {
			let (bind_group, multisample_resolve_texture_view, sampler) =
				create_multisample_resolve_texture(size, ctx, settings);
			(bind_group, Some(multisample_resolve_texture_view), sampler)
		} else {
			let sampler = ctx.graphics_ctx.device.create_sampler(&SamplerDescriptor {
				address_mode_u: address_mode,
				address_mode_v: address_mode,
				address_mode_w: address_mode,
				mag_filter: settings.magnifying_filter,
				min_filter: settings.minifying_filter,
				mipmap_filter: FilterMode::Nearest,
				border_color: settings.address_mode.border_color(),
				..Default::default()
			});
			let bind_group = ctx
				.graphics_ctx
				.device
				.create_bind_group(&BindGroupDescriptor {
					layout: &ctx.graphics_ctx.texture_bind_group_layout,
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
			(bind_group, None, sampler)
		};
		Self(Rc::new(CanvasInner {
			view,
			sampler,
			bind_group,
			multisample_resolve_texture_view,
			size,
			depth_stencil_texture_view: create_depth_stencil_texture_view(
				size,
				&ctx.graphics_ctx.device,
				settings.sample_count,
			),
		}))
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

	pub fn render_to<T>(
		&self,
		ctx: &mut Context,
		settings: RenderToCanvasSettings,
		mut f: impl FnMut(&mut Context) -> T,
	) -> T {
		ctx.graphics_ctx.set_render_target_to_canvas(
			self.clone(),
			settings.clear_color,
			settings.clear_stencil_value,
		);
		let returned_value = f(ctx);
		ctx.graphics_ctx.set_render_target_to_surface();
		returned_value
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CanvasSettings {
	pub sample_count: u32,
	pub address_mode: AddressMode,
	pub minifying_filter: FilterMode,
	pub magnifying_filter: FilterMode,
}

impl Default for CanvasSettings {
	fn default() -> Self {
		Self {
			sample_count: 1,
			address_mode: AddressMode::default(),
			minifying_filter: FilterMode::default(),
			magnifying_filter: FilterMode::default(),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderToCanvasSettings {
	pub clear_color: Option<LinSrgba>,
	pub clear_stencil_value: Option<u32>,
}

pub(crate) struct CanvasInner {
	pub(crate) view: TextureView,
	pub(crate) sampler: Sampler,
	pub(crate) bind_group: BindGroup,
	pub(crate) multisample_resolve_texture_view: Option<TextureView>,
	pub(crate) size: UVec2,
	pub(crate) depth_stencil_texture_view: TextureView,
}

fn create_multisample_resolve_texture(
	size: UVec2,
	ctx: &Context,
	settings: CanvasSettings,
) -> (BindGroup, TextureView, Sampler) {
	let texture_size = Extent3d {
		width: size.x,
		height: size.y,
		depth_or_array_layers: 1,
	};
	let texture = ctx.graphics_ctx.device.create_texture(&TextureDescriptor {
		size: texture_size,
		mip_level_count: 1,
		sample_count: 1,
		dimension: TextureDimension::D2,
		format: ctx.graphics_ctx.config.format,
		usage: TextureUsages::TEXTURE_BINDING
			| TextureUsages::COPY_DST
			| TextureUsages::RENDER_ATTACHMENT,
		label: Some("Texture"),
		view_formats: &[],
	});
	let view = texture.create_view(&TextureViewDescriptor::default());
	let address_mode = settings.address_mode.to_wgpu_address_mode();
	let sampler = ctx.graphics_ctx.device.create_sampler(&SamplerDescriptor {
		address_mode_u: address_mode,
		address_mode_v: address_mode,
		address_mode_w: address_mode,
		mag_filter: settings.magnifying_filter,
		min_filter: settings.minifying_filter,
		mipmap_filter: FilterMode::Nearest,
		border_color: settings.address_mode.border_color(),
		..Default::default()
	});
	let bind_group = ctx
		.graphics_ctx
		.device
		.create_bind_group(&BindGroupDescriptor {
			layout: &ctx.graphics_ctx.texture_bind_group_layout,
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
	(bind_group, view, sampler)
}
