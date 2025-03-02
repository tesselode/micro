use std::ops::{Deref, DerefMut};

use exhaust::Exhaust;
use glam::{Mat4, UVec2};
use itertools::Itertools;
use palette::LinSrgba;

use crate::{Context, color::ColorConstants, standard_draw_param_methods};

use super::{
	Vertex2d,
	graphics_pipeline::GraphicsPipeline,
	texture::{Texture, TextureSettings},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Canvas {
	pub(crate) texture: Texture,

	// draw params
	pub graphics_pipeline: Option<GraphicsPipeline<Vertex2d>>,
	pub transform: Mat4,
	pub color: LinSrgba,
}

impl Canvas {
	pub fn new(ctx: &Context, size: UVec2, settings: CanvasSettings) -> Self {
		Self {
			texture: Texture::empty(ctx, size, settings.texture_settings),
			graphics_pipeline: None,
			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
		}
	}

	pub fn graphics_pipeline<'a>(
		&self,
		graphics_pipeline: impl Into<Option<&'a GraphicsPipeline<Vertex2d>>>,
	) -> Self {
		Self {
			graphics_pipeline: graphics_pipeline.into().cloned(),
			..self.clone()
		}
	}

	standard_draw_param_methods!();

	pub fn size(&self) -> UVec2 {
		self.texture.size()
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
		self.texture
			.graphics_pipeline(&self.graphics_pipeline)
			.transformed(self.transform)
			.color(self.color)
			.draw(ctx);
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CanvasSettings {
	pub texture_settings: TextureSettings,
	// pub msaa: Msaa,
	// pub hdr: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Exhaust)]
#[cfg_attr(feature = "serializing", derive(serde::Serialize, serde::Deserialize))]
pub enum Msaa {
	#[default]
	None,
	X2,
	X4,
	X8,
	X16,
}

impl Msaa {
	pub fn levels_up_to(max: Self) -> impl Iterator<Item = Self> {
		Self::exhaust().take_while_inclusive(move |&level| level < max)
	}

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

impl From<i32> for Msaa {
	fn from(value: i32) -> Self {
		match value {
			0 => Self::None,
			2 => Self::X2,
			4 => Self::X4,
			8 => Self::X8,
			16.. => Self::X16,
			_ => panic!("unexpected MSAA value"),
		}
	}
}

impl std::fmt::Display for Msaa {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(match self {
			Msaa::None => "None",
			Msaa::X2 => "X2",
			Msaa::X4 => "X4",
			Msaa::X8 => "X8",
			Msaa::X16 => "X16",
		})
	}
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderToCanvasSettings {
	pub clear_color: Option<LinSrgba>,
	// pub clear_stencil_value: Option<u32>,
}

#[must_use]
pub struct OnDrop<'window, 'a> {
	pub(crate) ctx: &'a mut Context<'window>,
}

impl<'window> Drop for OnDrop<'window, '_> {
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

impl<'window> DerefMut for OnDrop<'window, '_> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.ctx
	}
}
