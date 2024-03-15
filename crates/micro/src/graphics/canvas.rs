use std::{rc::Rc, sync::mpsc::Sender};

use glam::{Mat4, UVec2, Vec2};
use glow::{HasContext, NativeFramebuffer, NativeRenderbuffer, NativeTexture, PixelPackData};
use palette::LinSrgba;

use crate::{context::graphics::RenderTarget, math::Rect, Context};

use super::{
	shader::Shader,
	standard_draw_command_methods,
	texture::{Texture, TextureSettings},
	unused_resource::UnusedGraphicsResource,
	BlendMode, ColorConstants,
};

#[derive(Debug)]
pub struct Canvas {
	framebuffer: NativeFramebuffer,
	texture: Texture,
	depth_stencil_renderbuffer: NativeRenderbuffer,
	multisample_framebuffer: Option<MultisampleFramebuffer>,
	unused_resource_sender: Sender<UnusedGraphicsResource>,
}

impl Canvas {
	pub fn new(size: UVec2, settings: CanvasSettings) -> Self {
		Context::with(|ctx| {
			let gl = &ctx.graphics.gl;
			let framebuffer = unsafe {
				gl.create_framebuffer()
					.expect("error creating canvas framebuffer")
			};
			unsafe {
				gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
			}
			let mut texture = Texture::new_from_gl(
				&ctx.graphics.gl,
				Context::with(|ctx| ctx.graphics.unused_resource_sender.clone()),
				size,
				None,
				settings.texture_settings,
				settings.hdr,
			);
			texture.attach_to_framebuffer();
			let multisample_framebuffer = match settings.msaa {
				Msaa::None => None,
				_ => Some(MultisampleFramebuffer::new(
					gl.clone(),
					size,
					settings.msaa.num_samples(),
					settings.hdr,
				)),
			};
			let depth_stencil_renderbuffer = unsafe {
				gl.create_renderbuffer()
					.expect("error creating depth/stencil renderbuffer")
			};
			unsafe {
				gl.bind_renderbuffer(glow::RENDERBUFFER, Some(depth_stencil_renderbuffer));
				if settings.msaa != Msaa::None {
					gl.renderbuffer_storage_multisample(
						glow::RENDERBUFFER,
						settings.msaa.num_samples().into(),
						glow::DEPTH24_STENCIL8,
						size.x as i32,
						size.y as i32,
					);
				} else {
					gl.renderbuffer_storage(
						glow::RENDERBUFFER,
						glow::DEPTH24_STENCIL8,
						size.x as i32,
						size.y as i32,
					);
				}
				gl.framebuffer_renderbuffer(
					glow::FRAMEBUFFER,
					glow::DEPTH_STENCIL_ATTACHMENT,
					glow::RENDERBUFFER,
					Some(depth_stencil_renderbuffer),
				);
			}
			unsafe {
				gl.bind_framebuffer(glow::FRAMEBUFFER, None);
			}
			Self {
				framebuffer,
				texture,
				depth_stencil_renderbuffer,
				multisample_framebuffer,
				unused_resource_sender: ctx.graphics.unused_resource_sender.clone(),
			}
		})
	}

	pub fn size(&self) -> UVec2 {
		self.texture.size()
	}

	pub fn relative_rect(&self, absolute_rect: Rect) -> Rect {
		let size = self.size().as_vec2();
		Rect::from_corners(
			absolute_rect.top_left / size,
			absolute_rect.bottom_right() / size,
		)
	}

	pub fn render_to(&self) -> OnDrop {
		if let RenderTarget::Canvas { .. } = Context::with(|ctx| ctx.graphics.render_target) {
			unimplemented!("cannot nest render_to calls");
		}
		let size = self.size();
		unsafe {
			Context::with(|ctx| {
				ctx.graphics.gl.bind_framebuffer(
					glow::FRAMEBUFFER,
					Some(
						if let Some(MultisampleFramebuffer { framebuffer, .. }) =
							self.multisample_framebuffer
						{
							framebuffer
						} else {
							self.framebuffer
						},
					),
				);
			});
		}
		Context::with_mut(|ctx| {
			ctx.graphics
				.set_render_target(RenderTarget::Canvas { size })
		});
		OnDrop {
			canvas: self,
			on_drop: |canvas| {
				if let Some(MultisampleFramebuffer { framebuffer, .. }) =
					canvas.multisample_framebuffer
				{
					unsafe {
						Context::with(|ctx| {
							ctx.graphics
								.gl
								.bind_framebuffer(glow::READ_FRAMEBUFFER, Some(framebuffer));
							ctx.graphics
								.gl
								.bind_framebuffer(glow::DRAW_FRAMEBUFFER, Some(canvas.framebuffer));
							let width = canvas.size().x as i32;
							let height = canvas.size().y as i32;
							ctx.graphics.gl.blit_framebuffer(
								0,
								0,
								width,
								height,
								0,
								0,
								width,
								height,
								glow::COLOR_BUFFER_BIT,
								glow::NEAREST,
							);
						});
					}
				}
				unsafe {
					Context::with(|ctx| ctx.graphics.gl.bind_framebuffer(glow::FRAMEBUFFER, None));
				}
				Context::with_mut(|ctx| ctx.graphics.set_render_target(RenderTarget::Window));
			},
		}
	}

	pub fn draw(&self) -> DrawCanvasCommand {
		DrawCanvasCommand {
			canvas: self,
			params: DrawCanvasParams {
				region: Rect::new(Vec2::ZERO, self.size().as_vec2()),
				shader: None,
				transform: Mat4::IDENTITY,
				color: LinSrgba::WHITE,
				blend_mode: BlendMode::default(),
			},
		}
	}

	pub fn read(&self, buffer: &mut [u8]) {
		if buffer.len() < (self.size().x * self.size().y * 4) as usize {
			panic!("buffer not big enough");
		}
		Context::with(|ctx| {
			let gl = &ctx.graphics.gl;
			unsafe {
				gl.bind_framebuffer(glow::READ_FRAMEBUFFER, Some(self.framebuffer));
				gl.read_buffer(glow::COLOR_ATTACHMENT0);
				gl.read_pixels(
					0,
					0,
					self.size().x as i32,
					self.size().y as i32,
					glow::RGBA,
					glow::UNSIGNED_BYTE,
					PixelPackData::Slice(buffer),
				);
			}
		});
	}

	fn draw_inner(&self, params: &DrawCanvasParams) {
		self.texture
			.region(params.region)
			.shader(params.shader)
			.transformed(params.transform)
			.color(params.color)
			.blend_mode(params.blend_mode)
			.draw();
	}
}

impl Drop for Canvas {
	fn drop(&mut self) {
		self.unused_resource_sender
			.send(UnusedGraphicsResource::Framebuffer(self.framebuffer))
			.ok();
		self.unused_resource_sender
			.send(UnusedGraphicsResource::Renderbuffer(
				self.depth_stencil_renderbuffer,
			))
			.ok();
		if let Some(MultisampleFramebuffer {
			framebuffer,
			texture,
		}) = self.multisample_framebuffer
		{
			self.unused_resource_sender
				.send(UnusedGraphicsResource::Texture(texture))
				.ok();
			self.unused_resource_sender
				.send(UnusedGraphicsResource::Framebuffer(framebuffer))
				.ok();
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CanvasSettings {
	pub texture_settings: TextureSettings,
	pub msaa: Msaa,
	pub hdr: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Msaa {
	None,
	X2,
	X4,
	X8,
	X16,
}

impl Msaa {
	fn num_samples(&self) -> u8 {
		match self {
			Msaa::None => 0,
			Msaa::X2 => 2,
			Msaa::X4 => 4,
			Msaa::X8 => 8,
			Msaa::X16 => 16,
		}
	}
}

impl Default for Msaa {
	fn default() -> Self {
		Self::None
	}
}

#[derive(Debug)]
struct MultisampleFramebuffer {
	framebuffer: NativeFramebuffer,
	texture: NativeTexture,
}

impl MultisampleFramebuffer {
	fn new(gl: Rc<glow::Context>, size: UVec2, num_samples: u8, float: bool) -> Self {
		let framebuffer = unsafe {
			gl.create_framebuffer()
				.expect("error creating multisample framebuffer")
		};
		unsafe {
			gl.bind_framebuffer(glow::FRAMEBUFFER, Some(framebuffer));
		}
		let texture = unsafe {
			gl.create_texture()
				.expect("error creating multisample texture")
		};
		unsafe {
			gl.bind_texture(glow::TEXTURE_2D_MULTISAMPLE, Some(texture));
			gl.tex_image_2d_multisample(
				glow::TEXTURE_2D_MULTISAMPLE,
				num_samples.into(),
				if float {
					glow::RGBA16F
				} else {
					glow::SRGB8_ALPHA8
				} as i32,
				size.x as i32,
				size.y as i32,
				true,
			);
			gl.framebuffer_texture_2d(
				glow::FRAMEBUFFER,
				glow::COLOR_ATTACHMENT0,
				glow::TEXTURE_2D_MULTISAMPLE,
				Some(texture),
				0,
			);
		};
		// intentionally not unbinding this framebuffer because the
		// depth/stencil buffer will need it bound
		Self {
			framebuffer,
			texture,
		}
	}
}

#[must_use]

pub struct OnDrop<'a> {
	pub(crate) canvas: &'a Canvas,
	pub(crate) on_drop: fn(&Canvas),
}

impl<'a> Drop for OnDrop<'a> {
	fn drop(&mut self) {
		(self.on_drop)(self.canvas);
	}
}

pub struct DrawCanvasParams<'a> {
	pub region: Rect,
	pub shader: Option<&'a Shader>,
	pub transform: Mat4,
	pub color: LinSrgba,
	pub blend_mode: BlendMode,
}

pub struct DrawCanvasCommand<'a> {
	canvas: &'a Canvas,
	params: DrawCanvasParams<'a>,
}

impl<'a> DrawCanvasCommand<'a> {
	pub fn region(mut self, region: Rect) -> Self {
		self.params.region = region;
		self
	}

	standard_draw_command_methods!();
}

impl<'a> Drop for DrawCanvasCommand<'a> {
	fn drop(&mut self) {
		self.canvas.draw_inner(&self.params);
	}
}
