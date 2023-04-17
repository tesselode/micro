use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2};

use crate::graphics::color::Rgba;

use super::{graphics_pipeline::GraphicsPipeline, shader::Shader};

#[derive(Clone)]
pub struct DrawParams<S: Shader> {
	pub position: Vec2,
	pub rotation: f32,
	pub scale: Vec2,
	pub origin: Vec2,
	pub color: Rgba,
	pub graphics_pipeline: Option<GraphicsPipeline<S>>,
}

impl<S: Shader> DrawParams<S> {
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

	pub fn graphics_pipeline(
		self,
		graphics_pipeline: impl Into<Option<GraphicsPipeline<S>>>,
	) -> Self {
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

impl<S: Shader> Default for DrawParams<S> {
	fn default() -> Self {
		Self::new()
	}
}

impl<S: Shader> From<Vec2> for DrawParams<S> {
	fn from(position: Vec2) -> Self {
		Self::new().position(position)
	}
}

impl<S: Shader> From<Rgba> for DrawParams<S> {
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
