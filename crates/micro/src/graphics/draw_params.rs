use bytemuck::{Pod, Zeroable};
use glam::{Affine2, Vec2};
use palette::LinSrgba;

use crate::math::URect;

use super::{
	graphics_pipeline::GraphicsPipeline,
	shader::{DefaultShader, Shader},
};

#[derive(Clone)]
pub struct DrawParams<S: Shader = DefaultShader> {
	pub transform: Affine2,
	pub color: LinSrgba,
	pub graphics_pipeline: Option<GraphicsPipeline<S>>,
	pub stencil_reference: u32,
	pub scissor_rect: Option<URect>,
}

impl DrawParams<DefaultShader> {
	pub fn new() -> Self {
		Self {
			transform: Affine2::IDENTITY,
			color: LinSrgba::new(1.0, 1.0, 1.0, 1.0),
			graphics_pipeline: None,
			stencil_reference: 0,
			scissor_rect: None,
		}
	}
}

impl<S: Shader> DrawParams<S> {
	pub fn transformed(self, transform: Affine2) -> Self {
		Self {
			transform: transform * self.transform,
			..self
		}
	}

	pub fn translated(self, translation: Vec2) -> Self {
		Self {
			transform: Affine2::from_translation(translation) * self.transform,
			..self
		}
	}

	pub fn scaled(self, scale: Vec2) -> Self {
		Self {
			transform: Affine2::from_scale(scale) * self.transform,
			..self
		}
	}

	pub fn rotated(self, rotation: f32) -> Self {
		Self {
			transform: Affine2::from_angle(rotation) * self.transform,
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
			transform: Affine2::IDENTITY,
			color: Default::default(),
			graphics_pipeline: Default::default(),
			stencil_reference: Default::default(),
			scissor_rect: None,
		}
	}
}

impl From<Vec2> for DrawParams<DefaultShader> {
	fn from(translation: Vec2) -> Self {
		Self::new().translated(translation)
	}
}

impl From<Affine2> for DrawParams<DefaultShader> {
	fn from(transform: Affine2) -> Self {
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
	pub transform: Affine2,
	pub color: LinSrgba,
}
