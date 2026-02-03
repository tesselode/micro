//! Types for drawing to off-screen render targets.

use std::ops::{Deref, DerefMut};

use glam::{Mat4, UVec2, Vec2};
use palette::LinSrgba;
use wgpu::{
	Buffer, BufferUsages, Extent3d, MapMode, PollType, TexelCopyBufferInfo, TexelCopyBufferLayout,
	TextureFormat,
	wgt::{BufferDescriptor, CommandEncoderDescriptor},
};

use crate::{
	Context, color::ColorConstants, context::graphics::GraphicsContext, graphics::BlendMode,
	math::Rect, standard_draw_param_methods,
};

use super::{
	BlendAlphaMode,
	texture::{InternalTextureSettings, Texture, TextureSettings},
};

/// An off-screen surface that can be drawn to. It can then be drawn to the
/// window surface (or another canvas) like any other texture.
#[derive(Debug, Clone, PartialEq)]
pub struct Canvas {
	pub(crate) label: String,
	pub(crate) kind: CanvasKind,
	pub(crate) depth_stencil_texture: Texture,
	format: TextureFormat,
	read_buffer: Option<Buffer>,

	// draw params
	/// The portion of the canvas texture that should be drawn.
	pub region: Rect,
	/// The transform to use when drawing this canvas.
	pub transform: Mat4,
	/// The blend color to use when drawing this canvas.
	pub color: LinSrgba,
	/// The blend mode to use when drawing this canvas.
	pub blend_mode: BlendMode,
}

impl Canvas {
	/// Creates a new [`Canvas`] with the specified `size` in pixels.
	pub fn new(ctx: &Context, size: UVec2, settings: CanvasSettings) -> Self {
		Self::new_from_graphics_ctx(&ctx.graphics, size, settings)
	}

	standard_draw_param_methods!();

	/// Returns the size of the canvas in pixels.
	pub fn size(&self) -> UVec2 {
		match &self.kind {
			CanvasKind::Normal { texture } | CanvasKind::Multisampled { texture, .. } => {
				texture.size()
			}
		}
	}

	/// Returns the number of samples used for MSAA.
	pub fn sample_count(&self) -> u32 {
		match &self.kind {
			CanvasKind::Normal { .. } => 1,
			CanvasKind::Multisampled { sample_count, .. } => *sample_count,
		}
	}

	/// Returns the format of the underlying texture.
	pub fn format(&self) -> TextureFormat {
		self.format
	}

	pub fn drawable_texture(&self) -> Texture {
		match &self.kind {
			CanvasKind::Normal { texture } => texture.clone(),
			CanvasKind::Multisampled {
				resolve_texture, ..
			} => resolve_texture.clone(),
		}
	}

	pub fn read<T>(&self, ctx: &Context, f: impl FnOnce(&[u8]) -> T) -> T {
		let bytes_per_pixel = self
			.format
			.block_copy_size(None)
			.expect("could not get bytes per pixel");
		let buffer = self
			.read_buffer
			.clone()
			.expect("cannot read from a canvas not set as readable");
		let mut encoder = ctx
			.graphics
			.device
			.create_command_encoder(&CommandEncoderDescriptor {
				label: Some("Read Canvas Command Encoder"),
			});
		let source = match &self.kind {
			CanvasKind::Normal { texture } => texture,
			CanvasKind::Multisampled {
				resolve_texture, ..
			} => resolve_texture,
		};
		encoder.copy_texture_to_buffer(
			source.texture.as_image_copy(),
			TexelCopyBufferInfo {
				buffer: &buffer,
				layout: TexelCopyBufferLayout {
					offset: 0,
					bytes_per_row: Some(bytes_per_pixel * self.size().x),
					rows_per_image: Some(self.size().y),
				},
			},
			Extent3d {
				width: source.size().x,
				height: source.size().y,
				depth_or_array_layers: source.num_layers(),
			},
		);
		encoder.map_buffer_on_submit(&buffer, MapMode::Read, .., |result| {
			result.expect("error mapping buffer");
		});
		let submission = ctx.graphics.queue.submit([encoder.finish()]);
		ctx.graphics
			.device
			.poll(PollType::Wait {
				submission_index: Some(submission),
				timeout: None,
			})
			.unwrap();
		let view = buffer.get_mapped_range(..);
		let slice: &[u8] = &view;
		let result = f(slice);
		drop(view);
		buffer.unmap();
		result
	}

	/// Sets future drawing operations to happen on this canvas instead of the
	/// window. Returns an object which, when dropped, sets the render
	/// target back to the window.
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

	/// Draws the canvas.
	pub fn draw(&self, ctx: &mut Context) {
		self.drawable_texture()
			.region(self.region)
			.transformed(self.transform)
			.color(self.color)
			.blend_mode(self.blend_mode)
			.draw(ctx)
	}

	pub(crate) fn new_from_graphics_ctx(
		graphics: &GraphicsContext,
		size: UVec2,
		settings: CanvasSettings,
	) -> Self {
		Self {
			label: settings.label,
			kind: match settings.sample_count {
				1 => CanvasKind::Normal {
					texture: Texture::new(
						&graphics.device,
						&graphics.queue,
						size,
						1,
						None,
						settings.texture_settings.clone(),
						InternalTextureSettings {
							sample_count: 1,
							format: settings.format,
						},
					),
				},
				sample_count => CanvasKind::Multisampled {
					texture: Texture::new(
						&graphics.device,
						&graphics.queue,
						size,
						1,
						None,
						settings.texture_settings.clone(),
						InternalTextureSettings {
							sample_count,
							format: settings.format,
						},
					),
					resolve_texture: Texture::new(
						&graphics.device,
						&graphics.queue,
						size,
						1,
						None,
						settings.texture_settings.clone(),
						InternalTextureSettings {
							sample_count: 1,
							format: settings.format,
						},
					),
					sample_count,
				},
			},
			depth_stencil_texture: Texture::new(
				&graphics.device,
				&graphics.queue,
				size,
				1,
				None,
				settings.texture_settings,
				InternalTextureSettings {
					format: TextureFormat::Depth24PlusStencil8,
					sample_count: settings.sample_count,
				},
			),
			format: settings.format,
			read_buffer: settings.readable.then(|| {
				let bytes_per_pixel = settings
					.format
					.block_copy_size(None)
					.expect("could not get bytes per pixel");
				graphics.device.create_buffer(&BufferDescriptor {
					label: Some("Canvas Read Buffer"),
					size: bytes_per_pixel as u64 * size.x as u64 * size.y as u64,
					usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
					mapped_at_creation: false,
				})
			}),
			region: Rect::new(Vec2::ZERO, size.as_vec2()),
			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
			blend_mode: BlendMode::Alpha(BlendAlphaMode::Premultiplied),
		}
	}
}

/// Settings for a [`Canvas`].
#[derive(Debug, Clone, PartialEq)]
pub struct CanvasSettings {
	/// The name of the canvas.
	///
	/// Visible in graphics debugging tools like RenderDoc.
	pub label: String,
	/// Settings for the underlying texture.
	pub texture_settings: TextureSettings,
	/// How many samples to use for MSAA. A sample count of `1`
	/// means MSAA is not used.
	///
	/// You can get the supported sample counts with
	/// [`Context::supported_sample_counts`].
	pub sample_count: u32,
	/// The format to use for the underlying texture.
	pub format: TextureFormat,
	/// Whether to allow calling [`read`](Canvas::read) on the canvas.
	///
	/// Pre-allocates some extra memory on the GPU.
	pub readable: bool,
}

impl Default for CanvasSettings {
	fn default() -> Self {
		Self {
			label: "Canvas".into(),
			texture_settings: Default::default(),
			sample_count: 1,
			format: TextureFormat::Rgba8UnormSrgb,
			readable: false,
		}
	}
}

/// Settings to use when starting rendering to a canvas.
#[derive(Debug, Clone, PartialEq)]
pub struct RenderToCanvasSettings {
	/// The color to clear the pixels to before drawing,
	/// or `None` to leave the existing pixels intact.
	pub clear_color: Option<LinSrgba>,
	/// Whether to clear the depth buffer or not.
	pub clear_depth_buffer: bool,
	/// Whether to clear the stencil buffer or not.
	pub clear_stencil_value: bool,
	/// The name of the render pass.
	///
	/// Visible in graphics debugging tools like RenderDoc.
	pub render_pass_label: String,
}

impl RenderToCanvasSettings {
	/// Returns a [`RenderToCanvasSettings`] configured to not clear
	/// anything.
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

/// Sets the render target back to the window surface when dropped.
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
