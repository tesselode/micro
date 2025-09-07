use glam::{Mat4, UVec2, Vec2};
use palette::LinSrgba;
use wgpu::TextureFormat;

use crate::{Context, color::ColorConstants, math::Rect, standard_draw_param_methods};

use super::texture::{InternalTextureSettings, Texture, TextureSettings};

#[derive(Debug, Clone, PartialEq)]
pub struct Canvas {
	pub(crate) label: String,
	pub(crate) kind: CanvasKind,
	pub(crate) depth_stencil_texture: Texture,
	format: TextureFormat,

	// draw params
	pub region: Rect,
	pub transform: Mat4,
	pub color: LinSrgba,
}

impl Canvas {
	pub fn new(size: UVec2, settings: CanvasSettings) -> Self {
		Context::with(|ctx| Self {
			label: settings.label.clone(),
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
			region: Rect::new(Vec2::ZERO, size.as_vec2()),
			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
		})
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

	pub fn render_to(&self, settings: RenderToCanvasSettings) -> OnDrop {
		let _span = tracy_client::span!();
		Context::with_mut(|ctx| {
			ctx.graphics
				.start_canvas_render_pass(self.clone(), settings.clone())
		});
		OnDrop {}
	}

	pub fn draw(&self) {
		self.drawable_texture()
			.region(self.region)
			.transformed(self.transform)
			.color(self.color)
			.draw()
	}

	pub(crate) fn drawable_texture(&self) -> Texture {
		match &self.kind {
			CanvasKind::Normal { texture } => texture.clone(),
			CanvasKind::Multisampled {
				resolve_texture, ..
			} => resolve_texture.clone(),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct CanvasSettings {
	pub label: String,
	pub texture_settings: TextureSettings,
	pub sample_count: u32,
	pub format: TextureFormat,
}

impl Default for CanvasSettings {
	fn default() -> Self {
		Self {
			label: "Canvas".into(),
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
pub struct OnDrop;

impl Drop for OnDrop {
	fn drop(&mut self) {
		Context::with_mut(|ctx| ctx.graphics.finish_canvas_render_pass());
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
