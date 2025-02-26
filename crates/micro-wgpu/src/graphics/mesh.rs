use std::marker::PhantomData;

use glam::Mat4;
use wgpu::{
	Buffer, BufferUsages,
	util::{BufferInitDescriptor, DeviceExt},
};

use crate::{Context, context::graphics::DrawCommand};

use super::{Vertex, Vertex2d, graphics_pipeline::GraphicsPipeline};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mesh<V: Vertex = Vertex2d> {
	pub graphics_pipeline: Option<GraphicsPipeline<V>>,
	vertex_buffer: Buffer,
	index_buffer: Buffer,
	num_indices: u32,
	_phantom_data: PhantomData<V>,
}

impl<V: Vertex> Mesh<V> {
	pub fn new(ctx: &mut Context, vertices: &[V], indices: &[u32]) -> Self {
		let vertex_buffer = ctx
			.graphics
			.device
			.create_buffer_init(&BufferInitDescriptor {
				label: None,
				contents: bytemuck::cast_slice(vertices),
				usage: BufferUsages::VERTEX,
			});
		let index_buffer = ctx
			.graphics
			.device
			.create_buffer_init(&BufferInitDescriptor {
				label: None,
				contents: bytemuck::cast_slice(indices),
				usage: BufferUsages::INDEX,
			});
		Self {
			graphics_pipeline: None,
			vertex_buffer,
			index_buffer,
			num_indices: indices.len() as u32,
			_phantom_data: PhantomData,
		}
	}

	pub fn graphics_pipeline(
		&self,
		graphics_pipeline: impl Into<Option<GraphicsPipeline<V>>>,
	) -> Self {
		Self {
			graphics_pipeline: graphics_pipeline.into(),
			..self.clone()
		}
	}

	pub fn draw(&self, ctx: &mut Context) {
		ctx.graphics.queue_draw_command(DrawCommand {
			vertex_buffer: self.vertex_buffer.clone(),
			index_buffer: self.index_buffer.clone(),
			num_indices: self.num_indices,
			render_pipeline: self
				.graphics_pipeline
				.as_ref()
				.map(|graphics_pipeline| graphics_pipeline.render_pipeline.clone()),
			transform: Mat4::IDENTITY,
		});
	}
}
