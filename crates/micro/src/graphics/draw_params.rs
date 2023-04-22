use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2};

use crate::graphics::color::Rgba;

use super::{
	graphics_pipeline::GraphicsPipeline,
	shader::{DefaultShader, Shader},
};

#[derive(Clone)]
pub struct DrawParams<S: Shader = DefaultShader> {
	pub position: Vec2,
	pub rotation: f32,
	pub scale: Vec2,
	pub origin: Vec2,
	pub color: Rgba,
	pub graphics_pipeline: Option<GraphicsPipeline<S>>,
	pub stencil_reference: u32,
}

impl DrawParams<DefaultShader> {
	pub fn new() -> Self {
		Self {
			position: Vec2::ZERO,
			rotation: 0.0,
			scale: Vec2::ONE,
			origin: Vec2::ZERO,
			color: Rgba::WHITE,
			graphics_pipeline: None,
			stencil_reference: 0,
		}
	}
}

impl<S: Shader> DrawParams<S> {
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

	pub fn graphics_pipeline<S2: Shader>(
		self,
		graphics_pipeline: impl IntoOptionalGraphicsPipeline<S2>,
	) -> DrawParams<S2> {
		DrawParams {
			graphics_pipeline: graphics_pipeline.into_optional_graphics_pipeline(),
			position: self.position,
			rotation: self.rotation,
			scale: self.scale,
			origin: self.origin,
			color: self.color,
			stencil_reference: self.stencil_reference,
		}
	}

	pub fn stencil_reference(self, stencil_reference: u32) -> Self {
		Self {
			stencil_reference,
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
		Self {
			position: Default::default(),
			rotation: Default::default(),
			scale: Default::default(),
			origin: Default::default(),
			color: Default::default(),
			graphics_pipeline: Default::default(),
			stencil_reference: Default::default(),
		}
	}
}

impl From<Vec2> for DrawParams<DefaultShader> {
	fn from(position: Vec2) -> Self {
		Self::new().position(position)
	}
}

impl From<Rgba> for DrawParams<DefaultShader> {
	fn from(color: Rgba) -> Self {
		Self::new().color(color)
	}
}

impl<S: Shader> From<GraphicsPipeline<S>> for DrawParams<S> {
	fn from(graphics_pipeline: GraphicsPipeline<S>) -> Self {
		DrawParams::new().graphics_pipeline(graphics_pipeline)
	}
}

impl<S: Shader> From<&GraphicsPipeline<S>> for DrawParams<S> {
	fn from(graphics_pipeline: &GraphicsPipeline<S>) -> Self {
		DrawParams::new().graphics_pipeline(graphics_pipeline.clone())
	}
}

pub trait IntoOptionalGraphicsPipeline<S: Shader> {
	fn into_optional_graphics_pipeline(self) -> Option<GraphicsPipeline<S>>;
}

impl<S: Shader> IntoOptionalGraphicsPipeline<S> for Option<GraphicsPipeline<S>> {
	fn into_optional_graphics_pipeline(self) -> Option<GraphicsPipeline<S>> {
		self
	}
}

impl<S: Shader> IntoOptionalGraphicsPipeline<S> for GraphicsPipeline<S> {
	fn into_optional_graphics_pipeline(self) -> Option<GraphicsPipeline<S>> {
		Some(self)
	}
}

impl<S: Shader> IntoOptionalGraphicsPipeline<S> for &GraphicsPipeline<S> {
	fn into_optional_graphics_pipeline(self) -> Option<GraphicsPipeline<S>> {
		Some(self.clone())
	}
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub(crate) struct DrawParamsUniform {
	pub transform: Mat4,
	pub color: Rgba,
}
