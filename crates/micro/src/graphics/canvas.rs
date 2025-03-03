use std::ops::{Deref, DerefMut};

use glam::{Mat4, UVec2};
use palette::LinSrgba;
use wgpu::TextureFormat;

use crate::{Context, color::ColorConstants, math::URect, standard_draw_param_methods};

use super::texture::{InternalTextureSettings, Texture, TextureSettings};

#[derive(Debug, Clone, PartialEq)]
pub struct Canvas {
	pub(crate) kind: CanvasKind,
	pub(crate) depth_stencil_texture: Texture,

	// draw params
	pub transform: Mat4,
	pub color: LinSrgba,
	pub scissor_rect: Option<URect>,
}

impl Canvas {
	pub fn new(ctx: &Context, size: UVec2, settings: CanvasSettings) -> Self {
		Self {
			kind: match settings.sample_count {
				1 => CanvasKind::Normal {
					texture: Texture::new(
						&ctx.graphics.device,
						&ctx.graphics.queue,
						size,
						None,
						settings.texture_settings,
						InternalTextureSettings {
							sample_count: 1,
							..Default::default()
						},
					),
				},
				sample_count => CanvasKind::Multisampled {
					texture: Texture::new(
						&ctx.graphics.device,
						&ctx.graphics.queue,
						size,
						None,
						settings.texture_settings,
						InternalTextureSettings {
							sample_count,
							..Default::default()
						},
					),
					resolve_texture: Texture::empty(ctx, size, settings.texture_settings),
				},
			},
			depth_stencil_texture: Texture::new(
				&ctx.graphics.device,
				&ctx.graphics.queue,
				size,
				None,
				settings.texture_settings,
				InternalTextureSettings {
					format: TextureFormat::Depth24PlusStencil8,
					sample_count: settings.sample_count,
				},
			),

			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
			scissor_rect: None,
		}
	}

	standard_draw_param_methods!();

	pub fn size(&self) -> UVec2 {
		match &self.kind {
			CanvasKind::Normal { texture } | CanvasKind::Multisampled { texture, .. } => {
				texture.size()
			}
		}
	}

	pub fn render_to<'a, 'window>(
		&'a self,
		ctx: &'a mut Context<'window>,
		settings: RenderToCanvasSettings,
	) -> OnDrop<'window, 'a> {
		let _span = tracy_client::span!();
		ctx.graphics
			.start_canvas_render_pass(self.clone(), settings);
		OnDrop { ctx }
	}

	pub fn draw(&self, ctx: &mut Context) {
		let _span = tracy_client::span!();
		let texture = match &self.kind {
			CanvasKind::Normal { texture } => texture,
			CanvasKind::Multisampled {
				resolve_texture, ..
			} => resolve_texture,
		};
		texture
			.transformed(self.transform)
			.color(self.color)
			.draw(ctx);
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CanvasSettings {
	pub texture_settings: TextureSettings,
	pub sample_count: u32,
	// pub hdr: bool,
}

impl Default for CanvasSettings {
	fn default() -> Self {
		Self {
			texture_settings: Default::default(),
			sample_count: 1,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderToCanvasSettings {
	pub clear_color: Option<LinSrgba>,
	pub clear_depth_buffer: bool,
	pub clear_stencil_value: bool,
}

impl Default for RenderToCanvasSettings {
	fn default() -> Self {
		Self {
			clear_color: Some(LinSrgba::BLACK),
			clear_depth_buffer: true,
			clear_stencil_value: true,
		}
	}
}

#[must_use]
pub struct OnDrop<'window, 'a> {
	pub(crate) ctx: &'a mut Context<'window>,
}

impl Drop for OnDrop<'_, '_> {
	fn drop(&mut self) {
		self.ctx.graphics.finish_canvas_render_pass();
	}
}

impl<'window> Deref for OnDrop<'window, '_> {
	type Target = Context<'window>;

	fn deref(&self) -> &Self::Target {
		self.ctx
	}
}

impl DerefMut for OnDrop<'_, '_> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.ctx
	}
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CanvasKind {
	Normal {
		texture: Texture,
	},
	Multisampled {
		texture: Texture,
		resolve_texture: Texture,
	},
}
