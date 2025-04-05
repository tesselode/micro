use bytemuck::Pod;
use wgpu::TextureFormat;

use crate::{
	Context,
	graphics::{BlendMode, Canvas, Shader, StencilState},
};

use super::GraphicsPipeline;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GraphicsPipelineBuilder<S: Shader> {
	pub label: String,
	pub blend_mode: BlendMode,
	pub shader_params: S::Params,
	pub storage_buffers: Vec<Vec<u8>>,
	pub enable_depth_testing: bool,
	pub stencil_state: StencilState,
	pub enable_color_writes: bool,
	pub sample_count: u32,
	pub format: TextureFormat,
}

impl<S: Shader> GraphicsPipelineBuilder<S> {
	pub fn new(ctx: &Context) -> Self
	where
		S::Params: Default,
	{
		Self {
			label: "Graphics Pipeline".into(),
			blend_mode: Default::default(),
			shader_params: Default::default(),
			storage_buffers: vec![],
			enable_depth_testing: false,
			stencil_state: Default::default(),
			enable_color_writes: true,
			sample_count: 1,
			format: ctx.surface_format(),
		}
	}

	pub fn for_canvas(canvas: &Canvas) -> Self
	where
		S::Params: Default,
	{
		Self {
			label: "Graphics Pipeline".into(),
			blend_mode: Default::default(),
			shader_params: Default::default(),
			storage_buffers: vec![],
			enable_depth_testing: false,
			stencil_state: Default::default(),
			enable_color_writes: true,
			sample_count: canvas.sample_count(),
			format: canvas.format(),
		}
	}

	pub fn label(self, label: impl Into<String>) -> Self {
		Self {
			label: label.into(),
			..self
		}
	}

	pub fn blend_mode(self, blend_mode: BlendMode) -> Self {
		Self { blend_mode, ..self }
	}

	pub fn shader_params(self, shader_params: S::Params) -> Self {
		Self {
			shader_params,
			..self
		}
	}

	pub fn with_storage_buffer<T: Pod>(mut self, data: &[T]) -> Self {
		self.storage_buffers.push(bytemuck::cast_slice(data).into());
		self
	}

	pub fn enable_depth_testing(self, enable_depth_testing: bool) -> Self {
		Self {
			enable_depth_testing,
			..self
		}
	}

	pub fn stencil_state(self, stencil_state: StencilState) -> Self {
		Self {
			stencil_state,
			..self
		}
	}

	pub fn enable_color_writes(self, enable_color_writes: bool) -> Self {
		Self {
			enable_color_writes,
			..self
		}
	}

	pub fn sample_count(self, sample_count: u32) -> Self {
		Self {
			sample_count,
			..self
		}
	}

	pub fn format(self, format: TextureFormat) -> Self {
		Self { format, ..self }
	}

	pub fn build(self, ctx: &Context) -> GraphicsPipeline<S> {
		GraphicsPipeline::new_internal(
			&ctx.graphics.device,
			&ctx.graphics.mesh_bind_group_layout,
			&ctx.graphics.shader_params_bind_group_layout,
			self,
		)
	}
}
