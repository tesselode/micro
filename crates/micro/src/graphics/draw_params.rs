use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2};

use crate::graphics::color::Rgba;

use super::graphics_pipeline::GraphicsPipeline;

#[derive(Clone)]
pub struct DrawParams {
	pub position: Vec2,
	pub rotation: f32,
	pub scale: Vec2,
	pub origin: Vec2,
	pub color: Rgba,
	pub graphics_pipeline: Option<GraphicsPipeline>,
}

impl DrawParams {
	pub fn new() -> Self {
		Self {
			position: Vec2::ZERO,
			rotation: 0.0,
			scale: Vec2::ONE,
			origin: Vec2::ZERO,
			color: Rgba::WHITE,
			graphics_pipeline: None,
		}
	}

	pub fn position(self, position: Vec2) -> Self {
		Self { position, ..self }
	}

	pub fn rotation(self, rotation: f32) -> Self {
		Self { rotation, ..self }
	}

	pub fn scale(self, scale: Vec2) -> Self {
		Self { scale, ..self }
	}

	pub fn origin(self, origin: Vec2) -> Self {
		Self { origin, ..self }
	}

	pub fn color(self, color: impl Into<Rgba>) -> Self {
		Self {
			color: color.into(),
			..self
		}
	}

	pub fn graphics_pipeline(self, graphics_pipeline: impl Into<Option<GraphicsPipeline>>) -> Self {
		Self {
			graphics_pipeline: graphics_pipeline.into(),
			..self
		}
	}

	pub fn as_mat4(&self) -> Mat4 {
		Mat4::from_translation(self.position.extend(0.0))
			* Mat4::from_rotation_z(self.rotation)
			* Mat4::from_scale(self.scale.extend(1.0))
			* Mat4::from_translation(-self.origin.extend(0.0))
	}

	pub(crate) fn as_uniform(&self) -> DrawParamsUniform {
		DrawParamsUniform {
			transform: self.as_mat4(),
			color: self.color,
		}
	}
}

impl Default for DrawParams {
	fn default() -> Self {
		Self::new()
	}
}

impl From<Vec2> for DrawParams {
	fn from(position: Vec2) -> Self {
		Self::new().position(position)
	}
}

impl From<Rgba> for DrawParams {
	fn from(color: Rgba) -> Self {
		Self::new().color(color)
	}
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub(crate) struct DrawParamsUniform {
	pub transform: Mat4,
	pub color: Rgba,
}
