use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec3};
use palette::LinSrgba;

use crate::math::URect;

use super::{
	graphics_pipeline::GraphicsPipeline,
	shader::{DefaultShader, Shader},
	ColorConstants,
};

#[derive(Clone)]
pub struct DrawParams<S: Shader = DefaultShader> {
	pub transform: Mat4,
	pub color: LinSrgba,
	pub graphics_pipeline: Option<GraphicsPipeline<S>>,
	pub stencil_reference: u32,
	pub scissor_rect: Option<URect>,
}

impl DrawParams<DefaultShader> {
	pub fn new() -> Self {
		Self {
			transform: Mat4::IDENTITY,
			color: LinSrgba::WHITE,
			graphics_pipeline: None,
			stencil_reference: 0,
			scissor_rect: None,
		}
	}
}

impl<S: Shader> DrawParams<S> {
	pub fn transformed(self, transform: Mat4) -> Self {
		Self {
			transform: transform * self.transform,
			..self
		}
	}

	pub fn translated(self, translation: Vec3) -> Self {
		Self {
			transform: Mat4::from_translation(translation) * self.transform,
			..self
		}
	}

	pub fn scaled(self, scale: Vec3) -> Self {
		Self {
			transform: Mat4::from_scale(scale) * self.transform,
			..self
		}
	}

	pub fn color(self, color: impl Into<LinSrgba>) -> Self {
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
			scissor_rect: self.scissor_rect,
		}
	}

	pub fn stencil_reference(self, stencil_reference: u32) -> Self {
		Self {
			stencil_reference,
			..self
		}
	}

	pub fn scissor_rect(self, scissor_rect: impl Into<Option<URect>>) -> Self {
		Self {
			scissor_rect: scissor_rect.into(),
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
			scissor_rect: None,
		}
	}
}

impl From<Vec3> for DrawParams<DefaultShader> {
	fn from(translation: Vec3) -> Self {
		Self::new().translated(translation)
	}
}

impl From<Mat4> for DrawParams<DefaultShader> {
	fn from(transform: Mat4) -> Self {
		Self::new().transformed(transform)
	}
}

impl From<LinSrgba> for DrawParams<DefaultShader> {
	fn from(color: LinSrgba) -> Self {
		Self::new().color(color)
	}
}

impl From<URect> for DrawParams<DefaultShader> {
	fn from(scissor_rect: URect) -> Self {
		Self::new().scissor_rect(scissor_rect)
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
	pub color: LinSrgba,
}
