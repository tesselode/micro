use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2};

use crate::graphics::color::Rgba;

use super::{
	graphics_pipeline::GraphicsPipeline,
	shader::{DefaultShader, Shader},
};

#[derive(Clone)]
pub struct DrawParams<S: Shader = DefaultShader> {
	pub transform: Mat4,
	pub color: Rgba,
	pub graphics_pipeline: Option<GraphicsPipeline<S>>,
	pub stencil_reference: u32,
}

impl DrawParams<DefaultShader> {
	pub fn new() -> Self {
		Self {
			transform: Mat4::IDENTITY,
			color: Rgba::WHITE,
			graphics_pipeline: None,
			stencil_reference: 0,
		}
	}
}

impl<S: Shader> DrawParams<S> {
	pub fn transform(self, transform: Mat4) -> Self {
		Self { transform, ..self }
	}

	pub fn translated(self, translation: Vec2) -> Self {
		Self {
			transform: Mat4::from_translation(translation.extend(0.0)) * self.transform,
			..self
		}
	}

	pub fn scaled(self, scale: Vec2) -> Self {
		Self {
			transform: Mat4::from_scale(scale.extend(0.0)) * self.transform,
			..self
		}
	}

	pub fn rotated(self, rotation: f32) -> Self {
		Self {
			transform: Mat4::from_rotation_z(rotation) * self.transform,
			..self
		}
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
			transform: self.transform,
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

	pub(crate) fn as_uniform(&self) -> DrawParamsUniform {
		DrawParamsUniform {
			transform: self.transform,
			color: self.color,
		}
	}
}

impl<S: Shader> Default for DrawParams<S> {
	fn default() -> Self {
		Self {
			transform: Mat4::IDENTITY,
			color: Default::default(),
			graphics_pipeline: Default::default(),
			stencil_reference: Default::default(),
		}
	}
}

impl From<Vec2> for DrawParams<DefaultShader> {
	fn from(translation: Vec2) -> Self {
		Self::new().translated(translation)
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
