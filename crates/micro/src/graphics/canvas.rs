use std::ops::{Deref, DerefMut};

use glam::{Mat4, UVec2};
use palette::LinSrgba;
use wgpu::TextureFormat;

use crate::{
	Context, color::ColorConstants, context::graphics::QueueDrawCommandSettings, math::URect,
	standard_draw_param_methods,
};

use super::{
	Vertex2d,
	drawable::Drawable,
	texture::{InternalTextureSettings, Texture, TextureSettings},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Canvas {
	pub(crate) kind: CanvasKind,
	pub(crate) depth_stencil_texture: Texture,
	format: TextureFormat,

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
							format: settings.format,
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
							format: settings.format,
						},
					),
					resolve_texture: Texture::new(
						&ctx.graphics.device,
						&ctx.graphics.queue,
						size,
						None,
						settings.texture_settings,
						InternalTextureSettings {
							sample_count: 1,
							format: settings.format,
						},
					),
					sample_count,
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
			format: settings.format,

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

	pub fn sample_count(&self) -> u32 {
		match &self.kind {
			CanvasKind::Normal { .. } => 1,
			CanvasKind::Multisampled { sample_count, .. } => *sample_count,
		}
	}

	pub fn format(&self) -> TextureFormat {
		self.format
	}

	pub fn render_to<'a>(
		&self,
		ctx: &'a mut Context,
		settings: RenderToCanvasSettings,
	) -> OnDrop<'a> {
		let _span = tracy_client::span!();
		ctx.graphics
			.start_canvas_render_pass(self.clone(), settings);
		OnDrop { ctx }
	}
}

impl Drawable for Canvas {
	type Vertex = Vertex2d;

	#[allow(private_interfaces)]
	fn draw(&self, ctx: &mut Context) -> Vec<QueueDrawCommandSettings> {
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
			.draw(ctx)
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CanvasSettings {
	pub texture_settings: TextureSettings,
	pub sample_count: u32,
	pub format: TextureFormat,
}

impl Default for CanvasSettings {
	fn default() -> Self {
		Self {
			texture_settings: Default::default(),
			sample_count: 1,
			format: TextureFormat::Rgba8UnormSrgb,
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenderToCanvasSettings {
	pub clear_color: Option<LinSrgba>,
	pub clear_depth_buffer: bool,
	pub clear_stencil_value: bool,
	pub render_pass_label: String,
}

impl RenderToCanvasSettings {
	pub fn no_clear() -> Self {
		Self {
			clear_color: None,
			clear_depth_buffer: false,
			clear_stencil_value: false,
			render_pass_label: "Canvas Render Pass".into(),
		}
	}
}

impl Default for RenderToCanvasSettings {
	fn default() -> Self {
		Self {
			clear_color: Some(LinSrgba::BLACK),
			clear_depth_buffer: true,
			clear_stencil_value: true,
			render_pass_label: "Canvas Render Pass".into(),
		}
	}
}

#[must_use]
pub struct OnDrop<'a> {
	pub(crate) ctx: &'a mut Context,
}

impl Drop for OnDrop<'_> {
	fn drop(&mut self) {
		self.ctx.graphics.finish_canvas_render_pass();
	}
}

impl Deref for OnDrop<'_> {
	type Target = Context;

	fn deref(&self) -> &Self::Target {
		self.ctx
	}
}

impl DerefMut for OnDrop<'_> {
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
		sample_count: u32,
	},
}
