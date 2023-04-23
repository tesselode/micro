use std::rc::Rc;

use glam::UVec2;
use wgpu::TextureView;

use crate::{math::Rect, Context};

use super::{
	color::Rgba, shader::Shader, texture::Texture, util::create_depth_stencil_texture_view,
	DrawParams,
};

#[derive(Clone)]
pub struct Canvas(pub(crate) Rc<CanvasInner>);

impl Canvas {
	pub fn new(ctx: &Context, size: UVec2) -> Self {
		Self(Rc::new(CanvasInner {
			texture: Texture::new_render_attachment(
				size,
				&ctx.graphics_ctx.device,
				&ctx.graphics_ctx.queue,
				&ctx.graphics_ctx.config,
				&ctx.graphics_ctx.texture_bind_group_layout,
			),
			depth_stencil_texture_view: create_depth_stencil_texture_view(
				size,
				&ctx.graphics_ctx.device,
			),
		}))
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
		self.0.texture.draw(ctx, params);
	}

	pub fn draw_region<S: Shader>(
		&self,
		ctx: &mut Context,
		region: Rect,
		params: impl Into<DrawParams<S>>,
	) {
		self.0.texture.draw_region(ctx, region, params);
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderToCanvasSettings {
	pub clear_color: Option<Rgba>,
	pub clear_stencil_value: Option<u32>,
}

pub(crate) struct CanvasInner {
	pub(crate) texture: Texture,
	pub(crate) depth_stencil_texture_view: TextureView,
}
